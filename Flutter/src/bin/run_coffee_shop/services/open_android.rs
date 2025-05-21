use std::io;
use std::process::Command;

pub fn open_android_studio() -> io::Result<()> {
    Command::new(r"D:\Program Files\Android\Android Studio\bin\studio64.exe")
        .spawn()?;
    Ok(())
}