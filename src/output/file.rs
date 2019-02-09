extern crate dirs;
extern crate serde_derive;

use super::{format_duration, PomodoroHandler};
use std::fs::{remove_file, File as StdFile, OpenOptions};
use std::io::prelude::*;
use std::io::ErrorKind::NotFound;
use std::path;
use std::time::Duration;

use super::config;

#[derive(Clone)]
pub struct File {
    pub config: config::Config,
}

fn _get_home_dir() -> path::PathBuf {
    match dirs::home_dir() {
        Some(path) => path,
        None => panic!("Could not define your home directory..."),
    }
}

impl File {
    fn get_file(&mut self) -> StdFile {
        let path: &path::PathBuf = &path::PathBuf::from(&self.config.outputs.file.path);
        let file = match OpenOptions::new().write(true).open(path) {
            Ok(f) => f,
            // We create the file in case it does not exist.
            Err(ref err) if err.kind() == NotFound => match StdFile::create(path) {
                Ok(f) => f,
                Err(e) => panic!("Could not create file: '{}'. Error: {}", path.display(), e),
            },
            Err(err) => panic!("Uncatched error happened in the file module: {}", err),
        };
        return file;
    }

    fn remove_file(&mut self) {
        let path: &path::PathBuf = &path::PathBuf::from(&self.config.outputs.file.path);
        match remove_file(path) {
            Err(e) => panic!("Could not delete file {}: {}", path.display(), e),
            Ok(_) => {}
        }
    }
}

impl PomodoroHandler for File {
    fn new(_output: &str, config: config::Config) -> File {
        return File { config: config };
    }

    fn start_handler(&mut self, _message: Option<&str>) {
        let mut file = self.get_file();
        match file.write_all(format_duration(self.config.pomodoro_duration).as_bytes()) {
            Ok(_) => {}
            Err(e) => panic!("Could not write to file. Error: {}", e),
        }
    }

    fn refresh(&mut self, remaining_time: Option<Duration>) {
        let mut file = self.get_file();
        match remaining_time {
            Some(remaining_time) => {
                match file.write_all(format_duration(remaining_time).as_bytes()) {
                    Err(e) => panic!("Could not write to the file. Error: {}", e),
                    Ok(_) => {}
                };
            }
            None => println!("No remaining time!"),
        }
    }

    fn end_handler(&mut self) {
        self.remove_file();
    }
}
#[derive(Deserialize, Clone)]
pub struct Config {
    pub path: String,
}
