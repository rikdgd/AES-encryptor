use std::io;
use std::fs;
use std::io::ErrorKind;



#[derive(Debug, Clone, PartialEq)]
pub struct Config<> {
    command: Command,
    path: String,
    is_dir: bool,
    passphrase: String,
}
impl Config {
    pub fn from_args(args: Vec<String>) -> io::Result<Self> {
        let command = args.get(1).expect("command was not provided").to_string();
        let path = args.get(2).expect("file path was not provided.").to_string();
        let passphrase = args.get(3).expect("key was not provided").to_string();

        let read_dir_result = fs::read_dir(&path);
        if fs::read(&path).is_err() && read_dir_result.is_err() {
            panic!("Invalid path selected.");
        }
        
        let is_dir = read_dir_result.is_ok();

        if passphrase.len() < 6 {
            panic!("The chosen passphrase is to short.");
        }

        let command = Command::from_str(command.as_ref());
        if let Some(command) = command {
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

    pub fn command(&self) -> &Command {
        &self.command
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn passphrase(&self) -> &str {
        &self.passphrase
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Command {
    Encrypt,
    Decrypt
}
impl Command {
    pub fn from_str(command: &str) -> Option<Self> {
        match command {
            "encrypt" => {
                Some(Command::Encrypt)
            },
            "decrypt" => {
                Some(Command::Decrypt)
            },
            _ => {
                None
            }
        }
    }
}
