mod file_management;
mod encryption;
mod config;

use std::env;
use std::io;
use std::path::Path;

use encryption::aes_encryption::{encrypt_file, decrypt_file};
use file_management::get_files_in_dir;
use config::{Config, Command};


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let config = Config::from_args(args)?;
    let path = Path::new(config.path());

    if path.is_dir() {
        for file_path in get_files_in_dir(path)? {
            match config.command() {
                Command::Encrypt => encrypt_file(&file_path, config.passphrase()),
                Command::Decrypt => decrypt_file(&file_path, config.passphrase()),
            }
        }
    } else if path.is_file() {
        match config.command() {
            Command::Encrypt => encrypt_file(path.to_str().unwrap(), config.passphrase()),
            Command::Decrypt => decrypt_file(path.to_str().unwrap(), config.passphrase()),
        }
    }
    
    Ok(())
}
