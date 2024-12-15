use std::time::Duration;

use serialport::SerialPort;

pub fn init_port() -> Box<dyn SerialPort> {
    let port = serialport::new("/dev/ttyUSB0", 9600)
        .data_bits(serialport::DataBits::Eight)
        .stop_bits(serialport::StopBits::One)
        .flow_control(serialport::FlowControl::None)
        .parity(serialport::Parity::None)
        .timeout(Duration::from_secs(10))
        .open().expect("failed to open port");

    port.clear(serialport::ClearBuffer::All).expect("failed to clear input buffer");

    return port;
}
