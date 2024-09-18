mod aes_encryption;
mod key_generation;
mod file_management;

use std::env;
use std::fs;
use std::io::ErrorKind;
use aes_encryption::{encrypt_file, decrypt_file};


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::from_args(args).unwrap();
    
    match config.command {
        Command::Encrypt => {
            todo!()
        },
        Command::Decrypt => {
            todo!()
        },
    }
}

struct Config<> {
    command: Command,
    path: String,
    is_dir: bool,
    passphrase: String,
}

impl Config {
    pub fn from_args(args: Vec<String>) -> std::io::Result<Self> {
        let command = args.get(1).expect("command was not provided").to_string();
        let path = args.get(2).expect("file path was not provided.").to_string();
        let passphrase = args.get(3).expect("key was not provided").to_string();
        
        // Parse command to enum for type safety.
        let parsed_command: Option<Command> = match command.as_ref() {
            "encrypt" => {
                Some(Command::Encrypt)
            },
            "decrypt" => {
                Some(Command::Decrypt)
            },
            _ => {
                panic!("Invalid command.");
            }
        };

        let read_dir_result = fs::read_dir(&path);
        if fs::read(&path).is_err() && read_dir_result.is_err() {
            panic!("Invalid path selected.");
        }
        
        let is_dir = match read_dir_result {
            Ok(_) => true,
            Err(_) => false,
        };
        
        if passphrase.len() < 6 {
            panic!("The chosen passphrase is to short.");
        }
        
        if let Some(command) = parsed_command {
            Ok(Self {
                command,
                path,
                is_dir,
                passphrase,
            })
        } else {
            Err(std::io::Error::new(
                ErrorKind::InvalidInput, 
                "Invalid command."
            ))
        }
    }
}

enum Command {
    Encrypt,
    Decrypt
}
