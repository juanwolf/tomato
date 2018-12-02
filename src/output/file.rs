extern crate dirs;

use std::time::Duration;
use std::fs::{remove_file, File as StdFile, OpenOptions};
use std::path;
use std::io::prelude::*;
use super::{PomodoroHandler, format_duration};

pub struct File {
    pub refresh_rate: Duration,
    pub pomodoro_duration: Duration,
    file_path: path::PathBuf,
}

fn get_home_dir() -> path::PathBuf {
    match dirs::home_dir() {
        Some(path) => path,
        None => panic!("Could not define your home directory..."),
    }
}

impl File {
    fn get_file(&mut self) -> StdFile {
        let path: &path::PathBuf = &self.file_path;
        let file = match OpenOptions::new().write(true).open(path) {
            Ok(f) => f,
            // We create the file in case it does not exist.
            Err(_) => match StdFile::create(path) {
                Ok(f) => f,
                Err(e) => panic!("Could not create file: {}. Error: {}", path.display(), e),
            }
        };
        return file;
    }

    fn remove_file(&mut self) {
        let path: &path::PathBuf = &self.file_path;
        match remove_file(path) {
            Err(e) => panic!("Could not delete file {}: {}", self.file_path.display(), e),
            Ok(_) => {},
        }
    }
}

impl PomodoroHandler for File {
    fn new(_output: &str, refresh_rate: super::Duration, pomodoro_duration: Duration) -> File {
        let home_dir = get_home_dir();
        let file_path = home_dir.join(".tomato");

        return File {
            pomodoro_duration: pomodoro_duration,
            refresh_rate: refresh_rate,
            file_path: file_path,
        };
    }

    fn start_handler(&mut self, _message: Option<&str>) {
        let mut file = self.get_file(); 
        match file.write_all(format_duration(self.pomodoro_duration).as_bytes()) {
            Ok(_) => {},
            Err(e) => panic!("Could not write to file. Error: {}", e),
        }
    }


    fn refresh(&mut self, remaining_time: Option<Duration>) {
        let mut file = self.get_file();
        match remaining_time {
            Some(remaining_time) => {
                match file.write_all(format_duration(remaining_time).as_bytes()) {
                    Err(e) => panic!("Could not write to the file. Error: {}", e),
                    Ok(_) => {},
                };
            },
            None => println!("No remaining time!"),
        }
    }

    fn end_handler(&mut self) {
        self.remove_file();
    }
}
