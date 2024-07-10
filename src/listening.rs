use crate::printing;
use crossterm::style::Stylize;
use serialport::SerialPort;
use std::io;
use std::time::Duration;

pub(crate) fn open_port(port: String, baud: u32) -> Box<dyn SerialPort> {
    println!(
        "Opening port: {} with baud rate: {}",
        port.clone().white(),
        baud.to_string().white()
    );
    let serial_port = match serialport::new(&port, baud)
        .timeout(Duration::from_millis(10))
        .open()
    {
        Ok(port) => port,
        Err(e) => {
            eprintln!("Failed to open \"{:?}\". Error: {}", port, e);
            std::process::exit(1);
        }
    };
    serial_port
}

pub(crate) fn filter_non_printable(data: &[u8]) -> Vec<u8> {
    data.iter()
        .filter(|&&byte| byte.is_ascii_graphic() || byte.is_ascii_whitespace())
        .copied()
        .collect()
}

pub(crate) fn listen(baud: u32, port: String) {
    let mut serial_port = open_port(port, baud);
    loop {
        let mut buffer: Vec<u8> = vec![0; 1000];
        match serial_port.read(buffer.as_mut_slice()) {
            Ok(t) => {
                if t > 0 {
                    //printing::get_serial_as_string(&buffer[..t], true);
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => {
                eprintln!("Failed to read from serial port. Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}
