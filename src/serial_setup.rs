use std::path::PathBuf;
use chrono::Local;

pub(crate) fn generate_log_filename() -> String {
    let now = Local::now();
    let formatted_date = now.format("%d_%m_%Y_%H-%M-%S").to_string();
    format!("{}.log", formatted_date)
}

pub(crate) fn baud_validate(s: &str) -> Result<u32, String> {
    // Baud rate mut be between 1 and 2^32

    let baud = s.parse::<u32>();
    match baud {
        Ok(0) => Err("Baud Rate cannot be zero".to_string()),
        Ok(rate) => Ok(rate),
        Err(_) => Err("Invalid baud rate".to_string())
    }
}

pub(crate) fn port_validate(s: &str) -> Result<String, String> {
    // Port must be in available ports
    let ports = serialport::available_ports().expect("No ports found!");
    if s.len() == 0 {
        return Err("Invalid port".to_string());
    }
    for p in ports {
        if p.port_name == s {
            return Ok(s.to_string());
        }
    }
    Err("Invalid port".to_string())
}

pub(crate) fn path_validate(s: &str) -> Result<PathBuf, String> {
    Ok(PathBuf::from(s))
}