#[derive(Debug)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

#[derive(Debug)]
pub struct Hsv
{
    pub h: f32,
    pub s: f32,
    pub v: f32
}

pub fn rgb_to_hsv(rgb: Rgb) -> Hsv {
    let r = rgb.r as f32 / 255.0;
    let g = rgb.g as f32 / 255.0;
    let b = rgb.b as f32 / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let mut h: f32;
    let s;
    let v = max;

    if delta != 0.0 {
        s = delta / max;

        if max == r {
            h = (g - b) / delta;
            if g < b {
                h += 6.0;
            }
        } else if max == g {
            h = (b - r) / delta + 2.0;
        } else {
            h = (r - g) / delta + 4.0;
        }

        h *= 60.0;
    } else {
        s = 0.0;
        h = 0.0;
    }

    Hsv { h, s, v }
}

pub fn hsv_to_rgb(hsv: Hsv) -> Rgb {
    let h = hsv.h as f32 / 360.0;
    let s = hsv.s;
    let v = hsv.v;

    let i = (h * 6.0).floor() as i32;
    let f = h * 6.0 - i as f32;
    let p = v * (1.0 - s);
    let q = v * (1.0 - f * s);
    let t = v * (1.0 - (1.0 - f) * s);

    let (r, g, b) = match i % 6 {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        5 => (v, p, q),
        _ => (0.0, 0.0, 0.0),
    };

    Rgb {
        r: (r * 255.0).round() as u8,
        g: (g * 255.0).round() as u8,
        b: (b * 255.0).round() as u8,
    }
}

pub fn yuyv_to_rgb(yuyv: &[u8]) -> Vec<u8> {
    let mut rgb = Vec::with_capacity(yuyv.len() * 3 / 2);

    let len = yuyv.len();
    let mut i = 0;

    while i < len {
        let y0 = yuyv[i] as f32;
        let u  = yuyv[i + 1] as f32;
        let y1 = yuyv[i + 2] as f32;
        let v  = yuyv[i + 3] as f32;

        let c = y0 - 16.0;
        let d = u - 128.0;
        let e = v - 128.0;

        let r1 = (298.0 * c + 409.0 * e + 128.0) / 256.0;
        let g1 = (298.0 * c - 100.0 * d - 208.0 * e + 128.0) / 256.0;
        let b1 = (298.0 * c + 516.0 * d + 128.0) / 256.0;

        rgb.push(clamp_rgb(r1));
        rgb.push(clamp_rgb(g1));
        rgb.push(clamp_rgb(b1));

        let c = y1 - 16.0;

        let r2 = (298.0 * c + 409.0 * e + 128.0) / 256.0;
        let g2 = (298.0 * c - 100.0 * d - 208.0 * e + 128.0) / 256.0;
        let b2 = (298.0 * c + 516.0 * d + 128.0) / 256.0;

        rgb.push(clamp_rgb(r2));
        rgb.push(clamp_rgb(g2));
        rgb.push(clamp_rgb(b2));

        i += 4;
    }

    rgb
}

fn clamp_rgb(value: f32) -> u8 {
    value.max(0.0).min(255.0) as u8
}