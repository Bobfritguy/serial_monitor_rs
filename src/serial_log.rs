use std::io;
use std::io::Write;
use std::path::PathBuf;
use crate::{listening, printing, serial_setup};

pub(crate) fn serial_log(baud: u32, port: String, path: Option<PathBuf>, print: bool) {

    let mut serial_port = listening::open_port(port, baud);

    // Open file for writing
    let path = match path.clone() {
        Some(path) => path,
        _ => {
            let path_string = serial_setup::generate_log_filename();
            let path = PathBuf::from(path_string);
            path.clone()
        }
    };

    // Already validated, should be safe to unwrap here.
    println!("Logging to file: {:?}", path);
    let mut file = match std::fs::File::create(&path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create file \"{:?}\". Error: {}", path, e);
            std::process::exit(1);
        }
    };

    // Read from serial port and write to file
    loop {
        let mut buffer: Vec<u8> = vec![0; 1000];
        let curr_time = chrono::Local::now();
        match serial_port.read(buffer.as_mut_slice()) {
            Ok(t) => {
                if t > 0 {
                    if print {
                        printing::print_serial_data(&with_time(&buffer[..t]));
                    }
                    match file.write_all(&with_time(&buffer[..t])) {
                        Ok(_) => (),
                        Err(e) => {
                            eprintln!("Failed to write to file \"{:?}\". Error: {}", path, e);
                            std::process::exit(1);
                        }
                    }
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

fn with_time(text_buffer: &[u8]) -> Vec<u8> {
    // Append to the beginning of the buffer, the current time in utf-8,
    // Formatted in the style [HH:MM:SS:MS-dd/mm/yyyy]
    let curr_time = chrono::Local::now();
    let time_str = curr_time.format("[%H:%M:%S:%3f-%d/%m/%Y] ").to_string();
    let time_buffer = time_str.as_bytes();
    let mut buffer = Vec::with_capacity(text_buffer.len() + time_buffer.len());
    buffer.extend_from_slice(time_buffer);
    buffer.extend_from_slice(&text_buffer);
    buffer
}



