use std::io;
use std::process::Command;

pub fn get_public_ip() -> io::Result<String> {
    let output = Command::new("ipconfig")
        .output()?;

    let text = String::from_utf8_lossy(&output.stdout);

    for line in text.lines() {
        let line = line.trim_start();

        if line.starts_with("IPv4 Address") {
            if let Some(ip_part) = line.split(':').nth(1) {
                let ip = ip_part.trim();
                if ip.starts_with("192.168.") {
                    return Ok(ip.to_string());
                }
            }
        }
    }

    Err(io::Error::new(io::ErrorKind::NotFound, "192.168 IP not found"))
}