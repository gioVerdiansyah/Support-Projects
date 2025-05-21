use std::io;
use Flutter::utils::run_command::run_basic_command;

pub fn open_wsl() -> io::Result<()> {
    run_basic_command("wsl -d Ubuntu")
}