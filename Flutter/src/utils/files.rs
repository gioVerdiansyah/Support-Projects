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

pub fn replace_line_with_prefix<P: AsRef<Path>>(
    file_path: P,
    prefix: &str,
    new_line: &str,
) -> io::Result<()> {
    let content = fs::read_to_string(&file_path)?;
    let mut found = false;

    let updated_content = content
        .lines()
        .map(|line| {
            if line.trim_start().starts_with(prefix) {
                found = true;
                new_line.to_string()
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    if !found {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("Prefix '{}' not found in file {:?}", prefix, file_path.as_ref()),
        ));
    }

    let mut file = fs::File::create(&file_path)?;
    file.write_all(updated_content.as_bytes())?;

    Ok(())
}