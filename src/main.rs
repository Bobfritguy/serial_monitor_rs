extern crate core;
extern crate alloc;

mod printing;
mod serial_setup;
mod serial_log;
mod list_ports;
mod listening;
mod terminal;

use std::path::PathBuf;
use clap::Subcommand;
use clap::Parser;


/// Read a serial port and log to file, provides helper function to display available ports
#[derive(Parser)]
#[command(name = "serial_logger", version = "1.0", author = "Séamus Knightly <seamusk@netfeasa.com>", about = "Program for Logging serial data")]
struct SerialLogger {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List available ports
    ListPorts {
        /// List available ports
        #[arg(short, long)]
        all: bool,

    },
    /// Log serial data to file
    Log{
        /// Baud rate for serial connection
        #[arg(short, long, value_name = "BAUD RATE", value_parser = serial_setup::baud_validate)]
        baud: u32,
        /// Port to connect to
        #[arg(long, value_name = "PORT", value_parser = serial_setup::port_validate)]
        port: String,
        /// Path to save log file
        #[arg(long, value_name = "FILE PATH", default_value = "", value_parser = serial_setup::path_validate)]
        path: Option<PathBuf>,
        /// Print serial to terminal as well as file
        #[arg(long, value_name = "PRINT", default_value = "false")]
        print: bool,
    },
    /// Listen to serial port
    Listen{
        /// Baud rate for serial connection
        #[arg(short, long, value_name = "BAUD RATE", value_parser = serial_setup::baud_validate)]
        baud: u32,
        /// Port to connect to
        #[arg(long, value_name = "PORT", value_parser = serial_setup::port_validate)]
        port: String,
    },
    /// Terminal for serial port
    Terminal{
        /// Baud rate for serial connection
        #[arg(short, long, value_name = "BAUD RATE", value_parser = serial_setup::baud_validate)]
        baud: u32,
        /// Port to connect to
        #[arg(long, value_name = "PORT", value_parser = serial_setup::port_validate)]
        port: String,
    }
}

fn main() {
    let args = SerialLogger::parse();

    match args.command {
        Commands::ListPorts { all } => {
            list_ports::list_available_ports(all);
        }
        Commands::Log { baud, port, path, print } => {
            serial_log::serial_log(baud, port, path, print);
        }
        Commands::Listen { baud, port } => {
            listening::listen(baud, port);
        }
        Commands::Terminal { baud, port } => {
            eprintln!("Terminal not implemented yet")
        }
    }
}


