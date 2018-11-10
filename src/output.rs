extern crate pbr;

use std::io;
use std::time::Duration;

use self::pbr::ProgressBar;

// An output is a representation of an external resource to store your
// pomodoros. They can be ephemeral or persistant.
pub trait Output {
    fn start_handler(&mut self, message:Option<&str>);
    fn end_handler(&mut self);
    fn refresh(&mut self, remaining_time: Option<Duration>);
    fn save(&mut self);
}

pub struct Stdout {
    pub refresh_rate: Duration,
    pub pomodoro_duration: Duration,
    pub pb: Option<ProgressBar<io::Stdout>>,
}

impl Output for Stdout {
    fn start_handler(&mut self, message: Option<&str>) {
        self.pb = Some(ProgressBar::new(
            self.pomodoro_duration.as_secs().wrapping_div(self.refresh_rate.as_secs())
        ));
        match self.pb {
            Some(ref mut pb) => {
                pb.show_speed     = false;
                pb.show_time_left = true;
                pb.show_counter   = false;
                pb.show_percent   = false;
            }
            None => {}
        }
        match message {
            Some(message) => println!("Starting a Pomodoro for {}", message),
            None => println!("Starting a pomodoro")
        }
    }

    fn refresh(&mut self, _remaining_time: Option<Duration>) {
        match self.pb {
            Some(ref mut pb) => {
                let _res = pb.inc();
            },
            None => println!("Tried to refresh stdout handler but no pb instantiated!!!")
        }
    }

    fn end_handler(&mut self) {
        println!("Nice one! You diserve a break.");
    }

    fn save(&mut self) {}
}
