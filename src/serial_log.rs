use std::io;
use std::io::Write;
use std::path::PathBuf;
use colored::{ColoredString, Colorize};
use crate::{listening, printing, serial_setup};

pub(crate) fn serial_log(baud: u32, port: String, path: Option<PathBuf>, print: bool) {
    let header = "====================".bold().white();

    let mut serial_port = listening::open_port(port, baud);

    // Create path for writing to file
    let path = match path.clone() {
        Some(path) => path,
        _ => {
            let path_string = serial_setup::generate_log_filename();
            let path = PathBuf::from(path_string);
            path.clone()
        }
    };

    if print {
        println!("{} {}", header, "Printing to screen:".bold().white());
    }

    let mut file = match std::fs::File::create_new(&path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create file \"{:?}\". Error: {}", path, e);
            std::process::exit(1);
        }
    };
    // Already validated, should be safe to unwrap here.
    println!("{} {} {}", header, "Logging to file: ".bold().white(), path.to_str().expect("Failed to convert path to string").italic().bold());
    file.write_all(format!("{} ========== Beginning Serial Log ===============\n", chrono::Local::now().format("[%a %d/%m/%Y %H:%M:%S:%3f] ")).as_bytes()).expect("Failed to write to file"); // Mess... Fix later
    file.flush().expect("Failed to flush file");




    // Read from serial port and write to file
    let mut line = String::new();
    let mut colour = String::new();
    let mut style = String::new();
    let mut reading_ansi_code = false;
    let mut coloured_string = String::new();
    let mut ansi_code = String::new();
    loop {
        let mut buffer: Vec<u8> = vec![0; 1000];
        match serial_port.read(buffer.as_mut_slice()) {
            Ok(t) => {
                if t > 0 {
                    let data = &buffer[..t];
                    // convert to utf8
                    let data = String::from_utf8_lossy(data);
                    for character in data.chars() {
                        match character { // Messy
                            '\r' => {
                            },
                            '\n' => {
                                line.push(character);
                                if print {
                                    print!("{}", &line);
                                    println!("Colour: {}", colour);
                                }
                                file.write_all(&with_time(&line).as_bytes()).expect("Failed to write to file");
                                file.flush().expect("Failed to flush file");
                                line.clear();
                            },
                            '\x1b' => {
                                reading_ansi_code = true;
                            },
                            'm' => {
                                if reading_ansi_code {
                                    reading_ansi_code = false;
                                    // TODO: Resolve colour
                                } else {
                                    line.push(character);
                                }
                            },
                            ';' => {
                                if reading_ansi_code {
                                    // TODO: Resolve colour
                                } else {
                                    line.push(character);
                                }
                            },
                            _ => {
                                if reading_ansi_code {
                                    // TODO: Resolve colour
                                } else {
                                    line.push(character);
                                }
                            }
                        }
                    }
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {},
            Err(e) => {
                eprintln!("Failed to read from serial port. Error: {}", e);
                std::process::exit(1);
            }
        }

    }
 
}

fn with_time(text_buffer: &String) -> String {
    // Append to the beginning of the buffer, the current time in utf-8,
    // Formatted in the style [HH:MM:SS:MS-dd/mm/yyyy]
    let curr_time = chrono::Local::now();
    let mut time_str = curr_time.format("[%a %d/%m/%Y %H:%M:%S:%3f] ").to_string();
    time_str.push_str(text_buffer);
    time_str
}



