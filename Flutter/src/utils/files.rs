use std::fs;
use std::fs::File;
use std::io::{self, Error, ErrorKind, Write};
use std::path::Path;

pub fn create_file_content<P: AsRef<Path>>(path: P, content: String) -> io::Result<()> {
    let path_ref = path.as_ref();

    let mut file = File::create(path_ref)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

pub fn insert_content_to_file<P: AsRef<Path>>(
    file_path: P,
    marker: &str,
    insert_text: &str,
) -> io::Result<()> {
    let content = fs::read_to_string(&file_path)?;

    if let Some(pos) = content.find(marker) {
        let insert_pos = pos + marker.len();
        let mut result = String::with_capacity(content.len() + insert_text.len() + 1);
        result.push_str(&content[..insert_pos]);
        result.push('\n');
        result.push_str(insert_text);
        result.push_str(&content[insert_pos..]);

        let mut file = fs::File::create(&file_path)?;
        file.write_all(result.as_bytes())?;
        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::NotFound,
            format!("Marker '{}' not found in file {:?}", marker, file_path.as_ref()),
        ))
    }
}