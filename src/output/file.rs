extern crate dirs;

use std::time::Duration;
use std::fs::{remove_file, File as StdFile};
use std::path;
use std::io::prelude::*;
use super::{Output, format_duration};

pub struct File {
    pub refresh_rate: Duration,
    pub pomodoro_duration: Duration,
    file_path: path::PathBuf,
}

fn get_home_dir() -> path::PathBuf {
    return dirs::home_dir().unwrap_or_default();
}

impl File {
    fn get_file(&mut self) -> StdFile {
        let file = match StdFile::open(self.file_path) {
            Ok(f) => f,
            // We create the file in case it does not exist.
            Err(_) => match StdFile::create(self.file_path) {
                Ok(f) => f,
                Err(e) => panic!(e),
            }
        };
        return file;
    }

    fn remove_file(&mut self) {
        match remove_file(self.file_path) {
            Err(e) => panic!("Could not delete file {}: {}", self.file_path.display(), e),
        }
    }

}

impl Output for File {
    fn new(refresh_rate: super::Duration, pomodoro_duration: Duration) -> File {
        let home_dir = get_home_dir();
        let file_path = home_dir.join("/.tomato");

        return File {
            pomodoro_duration: pomodoro_duration,
            refresh_rate: refresh_rate,
            file_path: file_path,
        };
    }

    fn start_handler(&mut self, message: Option<&str>) {
        let file = self.get_file(); 
        file.write_all(format_duration(self.pomodoro_duration).as_bytes());
    }


    fn refresh(&mut self, remaining_time: Option<Duration>) {
        let file = self.get_file();
        match remaining_time {
            Some(remaining_time) => {
                file.write_all(format_duration(remaining_time).as_bytes());
            },
            None => println!("No remaining time!"),
        }
    }

    fn end_handler(&mut self) {
        self.remove_file();
    }
}
