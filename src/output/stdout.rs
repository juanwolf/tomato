extern crate pbr;

use std::io;
use std::time::Duration;

use self::pbr::ProgressBar;

use super::PomodoroHandler;

pub struct Stdout {
    pub refresh_rate: Duration,
    pub pomodoro_duration: Duration,
    pb: Option<ProgressBar<io::Stdout>>,
}

impl PomodoroHandler for Stdout {
    fn new(_output: &str, refresh_rate: Duration, pomodoro_duration: Duration) -> Stdout {
        return Stdout {
            refresh_rate: refresh_rate,
            pomodoro_duration: pomodoro_duration,
            pb: None,
        };
    }

    fn start_handler(&mut self, message: Option<&str>) {
        self.pb = Some(ProgressBar::new(
            self.pomodoro_duration
                .as_secs()
                .wrapping_div(self.refresh_rate.as_secs()),
        ));
        match self.pb {
            Some(ref mut pb) => {
                pb.show_speed = false;
                pb.show_time_left = true;
                pb.show_counter = false;
                pb.show_percent = false;
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
