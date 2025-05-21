use std::io;
use std::process::{Command, Stdio};
use Flutter::utils::run_command::run_powershell_admin;

pub fn wsl_ip(wsl_ip: &str) -> io::Result<()>{
    let ip_uot = Command::new("wsl")
        .arg("hostname")
        .arg("-I")
        .stdout(Stdio::piped())
        .output()?;

    let new_ip = String::from_utf8_lossy(&ip_uot.stdout).trim().to_string();

    println!("Checking WSL hostname IP....");
    println!("Got IP: {}", new_ip);

    if wsl_ip != new_ip {
        println!("WSL IP changed!, updating portproxy...");

        let full_command = format!(
            "netsh interface portproxy delete v4tov4 listenport=8999 listenaddress=0.0.0.0; \
     netsh interface portproxy add v4tov4 listenport=8999 listenaddress=0.0.0.0 connectport=8999 connectaddress={}",
            new_ip
        );

        run_powershell_admin(full_command);

        println!("Successfully change!");
    }

    Ok(())
}