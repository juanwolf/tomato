use std::time::Duration;

// An output is a representation of an external resource to store your
// pomodoros. They can be ephemeral or persistant.
pub trait Output {
    fn start_handler(&mut self, message: Option<&str>);
    fn end_handler(&mut self);
    fn refresh(&mut self, remaining_time: Option<Duration>);
}

pub mod stdout;
