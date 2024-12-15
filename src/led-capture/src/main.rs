use core::str;
use std::io;
use std::thread::sleep;
use std::time::{Duration, Instant};

use colors::{hsv_to_rgb, rgb_to_hsv, yuyv_to_rgb, Rgb};
use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};
use serial::init_port;
use serialport::SerialPort;
use space::{Matrix, Position};
use v4l::buffer::Type;
use v4l::io::traits::CaptureStream;
use v4l::{prelude::*, FourCC};
use v4l::video::Capture;

mod colors;
mod space;
mod serial;

const NUM_LEDS: i32 = 120;
const CAPTURE_DEVICE: &str = "/dev/video0";

fn dim_pixel(rgb: Rgb) -> Rgb {
    let mut hsv = rgb_to_hsv(rgb);
    hsv.s = 1.0;
    hsv.v = hsv.v / 4.0;
    hsv_to_rgb(hsv)
}

fn send_frame(port: &mut Box<dyn SerialPort>, frame: &Matrix, positions: &Vec<Position>) -> Option<String> {
    let mut data: Vec<u8> = vec![0; 360];
    let mut data_i = 0;
    let pixels = positions
        .iter()
        .map(|pos| frame.get_avg_color(pos.x, pos.y, 10, 10))
        .map(|x| dim_pixel(x));
    for pixel in pixels {
        data[data_i + 0] = pixel.r;
        data[data_i + 1] = pixel.g;
        data[data_i + 2] = pixel.b;
        data_i += 3;
    }
    let length: Vec<u8> = (u32::try_from(data.len()).expect("isn't u32")).to_be_bytes().to_vec();
    let data: Vec<u8> = [length, data].concat();
    port.write(&data).expect("failed to write to serial port");

    let mut buf: Vec<u8> = vec![0; 32];
    let read = port.read(&mut buf);
    let read = match read {
        Ok(x) => x,
        Err(_) => return None
    };

    let result = str::from_utf8(&buf[0..read]);
    let result = match result {
        Ok(x) => x.trim(),
        Err(_) => return None
    };

    Some(result.to_owned())
}

fn main() -> io::Result<()> {
    let seed = [42u8; 32];
    let mut rand = StdRng::from_seed(seed);

    println!("init serial port");
    let mut port = init_port();

    // idk why does rapid pixel write cause an error by i/o port
    sleep(Duration::from_secs(3));

    println!("using capture device: {}\n", CAPTURE_DEVICE);
    let dev = Device::with_path(CAPTURE_DEVICE)?;
    let mut format = dev.format().expect("failed to get format");
    format.fourcc = FourCC::new(b"YUYV");

    let positions: Vec<Position> = (0..NUM_LEDS)
        .map(|_| Position { x: rand.next_u32() % format.width, y: rand.next_u32() % format.height })
        .collect();

    let format = dev.set_format(&format).expect("fail to set format");
    let params = dev.params()?;
    println!("active format:\n{}", format);
    println!("active parameters:\n{}", params);

    let mut stream = MmapStream::with_buffers(&dev, Type::VideoCapture, 1)?;
    let mut prev_sequence = 0;
    loop {
        let t0 = Instant::now();
        let (buf, meta) = stream.next()?;
        let buf = yuyv_to_rgb(buf);
        let matrix = Matrix::new(format.width, format.height, &buf);
        println!("frame process time: {:#?}", t0.elapsed());
        println!("sequence lag      : {}", meta.sequence - prev_sequence);
        prev_sequence = meta.sequence;

        let t0 = Instant::now();
        let result = send_frame(&mut port, &matrix, &positions);
        println!("send time         : {:#?}", t0.elapsed());
        println!("{}", "-".repeat(32));
        if let None = result {
            println!("failed to write pixels");
            return Ok(());
        }
        if let Some(result) = result {
            if result != "ok" {
                println!("error from firmware: '{}'", result.replace("\n", "\\n"));
                println!("maybe you need to reset receive sequence (restart firmware)");
                return Ok(());
            }
        }
    }
}
