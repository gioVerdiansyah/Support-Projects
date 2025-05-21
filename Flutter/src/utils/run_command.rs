use std::io;
use std::process::Command;

pub fn run_basic_command(command: &str) -> io::Result<()> {
    let status = Command::new("cmd")
        .args(["/C", command])
        .status()?;

    if !status.success() {
        println!("{}", format!("Failed to run: {}", command));
        std::process::exit(1)
    }
    Ok(())
}
pub fn run_powershell_admin(command: String) {
    let full_command = format!(
        "Start-Process powershell -ArgumentList '-NoExit','-Command','{}' -Verb RunAs",
        command.replace("'", "''")
    );

    let status = Command::new("powershell")
        .args(&["-Command", &full_command])
        .status();

    match status {
        Ok(s) if s.success() => println!("Command run successfully."),
        Ok(s) => eprintln!("Command exited with status: {}", s),
        Err(e) => eprintln!("Failed to run PowerShell command: {}", e),
    }
}