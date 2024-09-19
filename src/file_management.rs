use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::{ErrorKind, Read, Write};
use std::path::Path;

/// Reads a file and returns (bytes_read, file_content)
pub fn read_file(path: &str) -> std::io::Result<(usize, Vec<u8>)> {
    let mut file = OpenOptions::new()
        .read(true)
        .open(path)?;

    let mut content_buffer: Vec<u8> = Vec::new();
    let bytes_read = file.read_to_end(&mut content_buffer)?;
    file.flush()?;

    Ok((bytes_read, content_buffer))
}

/// Sets the file length to 0, then writes the given content to it.
pub fn clear_write_file(path: &str, new_content: Vec<u8>) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .open(path)?;

    file.set_len(0)?;
    file.write_all(new_content.as_ref())?;
    file.flush()?;

    Ok(())
}


/// Finds all valid files in a directory, won't return subdirectories.
pub fn get_files_in_dir(dir_path: &Path) -> io::Result<Vec<String>> {
    if !dir_path.is_dir() {
        return Err(io::Error::new(
            ErrorKind::InvalidInput,
            "Given path did not lead to a directory."
        ));
    }

    let mut file_paths = Vec::new();
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_file() {
            if let Some(i) = entry_path.to_str() {
                file_paths.push(i.to_string());
            }
        }
    }

    Ok(file_paths)
}
