extern crate pbr;
extern crate serde_derive;

use std::io;

use self::pbr::ProgressBar;

use super::PomodoroHandler;

use super::config;

pub struct Stdout {
    pub config: config::Config,
    pb: Option<ProgressBar<io::Stdout>>,
}

impl PomodoroHandler for Stdout {
    fn new(_output: &str, config: config::Config) -> Stdout {
        return Stdout {
            config: config,
            pb: None,
        };
    }

    fn start_handler(&mut self, message: Option<&str>) {
        self.pb = Some(ProgressBar::new(
            self.config
                .pomodoro_duration
                .as_secs()
                .wrapping_div(self.config.refresh_rate.as_secs()),
        ));
        match self.pb {
            Some(ref mut pb) => {
                pb.show_speed = false;
                pb.show_time_left = true;
                pb.show_counter = false;
                pb.show_percent = self.config.outputs.stdout.show_percent;
            }
            None => {}
        }
        match message {
            Some(message) => println!("Starting a Pomodoro for {}", message),
            None => println!("Starting a pomodoro"),
        }
    }

    fn refresh(&mut self, _remaining_time: Option<super::Duration>) {
        match self.pb {
            Some(ref mut pb) => {
                let _res = pb.inc();
            }
            None => println!("Tried to refresh stdout handler but no pb instantiated!!!"),
        }
    }

    fn end_handler(&mut self) {
        println!("Nice one! You diserve a break.");
    }
}

impl Clone for Stdout {
    fn clone(&self) -> Stdout {
        return Stdout::new("", self.config.clone());
    }
}

#[derive(Deserialize, Clone)]
pub struct Config {
    pub show_percent: bool,
}
