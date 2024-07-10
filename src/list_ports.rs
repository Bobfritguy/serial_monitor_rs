pub(crate) fn list_available_ports(all: bool) {
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        match p.port_type {
            serialport::SerialPortType::UsbPort(info) => {
                println!(
                    "Port: {:?}, {:?}, {:?}",
                    p.port_name,
                    info.manufacturer
                        .unwrap_or("No Manufacturer".parse().unwrap()),
                    info.product.unwrap_or("No Product".parse().unwrap())
                );
            }
            serialport::SerialPortType::BluetoothPort => {
                println!("Port: {:?}, Bluetooth", p.port_name);
            }
            serialport::SerialPortType::PciPort => {
                println!("Port: {:?}, PCI", p.port_name);
            }
            serialport::SerialPortType::Unknown => {
                if all {
                    println!("Port: {:?}, Unknown", p.port_name);
                }
            }
        }
    }
}
