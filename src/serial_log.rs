use crate::{listening, serial_setup};
use colored::{Colorize};
use std::{io, thread};
use std::io::{Write};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self};
use termion::event::Key;
use termion::input::TermRead;

pub(crate) fn serial_log(baud: u32, port: String, path: Option<PathBuf>, print: bool) {
    // For CTRL+P print interrupt
    let print_flag = Arc::new(AtomicBool::new(print));
    let print_flag_clone = Arc::clone(&print_flag);

    // Create a channel for communication
    let (tx, rx): (mpsc::Sender<()>, mpsc::Receiver<()>) = mpsc::channel();

    // Create a thread to listen for CTRL+P to toggle print flag
    thread::spawn(move || {
        let stdin = io::stdin();
        for key in stdin.keys() {
            if let Ok(Key::Ctrl('p')) = key {
                // Send a message when CTRL+P is pressed
                tx.send(()).expect("Failed to send message");
            }
        }
    });

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

    let mut file = match std::fs::File::create(&path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create file \"{:?}\". Error: {}", path, e);
            std::process::exit(1);
        }
    };

    println!(
        "{} {} {}",
        header,
        "Logging to file: ".bold().white(),
        path.to_str()
            .expect("Failed to convert path to string")
            .italic()
            .bold()
    );

    file.write_all(
        format!(
            "{} ========== Beginning Serial Log ===============\n",
            chrono::Local::now().format("[%a %d/%m/%Y %H:%M:%S:%3f] ")
        )
        .as_bytes(),
    )
    .expect("Failed to write to file");

    file.flush().expect("Failed to flush file");

    // Read from serial port and write to file
    let mut line = String::new();
    let mut reading_ansi_code = false;

    loop {
        let mut buffer: Vec<u8> = vec![0; 1000];
        match serial_port.read(buffer.as_mut_slice()) {
            Ok(t) => {
                if t > 0 {
                    let data = &buffer[..t];
                    // convert to utf8
                    let data = String::from_utf8_lossy(data);
                    for character in data.chars() {
                        match character {
                            // Messy
                            '\r' => {}
                            '\n' => {
                                line.push(character);
                                if print {
                                    print!("{}", &line);
                                }
                                file.write_all(&with_time(&line).as_bytes())
                                    .expect("Failed to write to file");
                                file.flush().expect("Failed to flush file");
                                line.clear();
                            }
                            '\x1b' => {
                                reading_ansi_code = true;
                            }
                            'm' => {
                                if reading_ansi_code {
                                    reading_ansi_code = false;
                                    // TODO: Resolve colour
                                } else {
                                    line.push(character);
                                }
                            }
                            ';' => {
                                if reading_ansi_code {
                                    // TODO: Resolve colour
                                } else {
                                    line.push(character);
                                }
                            }
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
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {}
            Err(e) => {
                eprintln!("Failed to read from serial port. Error: {}", e);
                std::process::exit(1);
            }
        }

        // Check for messages from the key listening thread
        if let Ok(()) = rx.try_recv() {
            // Toggle the print flag
            let current_flag = print_flag.load(Ordering::SeqCst);
            print_flag.store(!current_flag, Ordering::SeqCst);
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

