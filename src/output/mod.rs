use std::time::Duration;

fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    return format!("{:02}:{:02}", secs / 60, secs % 60);
}

// An output is a representation of an external resource to store your
// pomodoros. They can be ephemeral or persistant.
pub trait PomodoroHandler {
    fn new(output: &str, refresh_rate: Duration, pomodoro_duration: Duration) -> Self;
    fn start_handler(&mut self, message: Option<&str>);
    fn end_handler(&mut self);
    fn refresh(&mut self, remaining_time: Option<Duration>);
}

pub mod file;
pub mod stdout;

pub enum Output {
    Stdout(stdout::Stdout),
    File(file::File),
}

impl PomodoroHandler for Output {
    fn new(output: &str, refresh_rate: Duration, pomodoro_duration: Duration) -> Self {
        match output {
            "file" => return Output::File(file::File::new("", refresh_rate, pomodoro_duration)),
            "stdout" => {
                return Output::Stdout(stdout::Stdout::new("", refresh_rate, pomodoro_duration))
            }
            unknown_output => panic!(
                "Unknown output type '{}'. Feel free to contribute if you're missing it out!",
                unknown_output
            ),
        }
    }

    fn start_handler(&mut self, message: Option<&str>) {
        match *self {
            Output::Stdout(ref mut stdout) => stdout.start_handler(message),
            Output::File(ref mut file) => file.start_handler(message),
        }
    }

    fn refresh(&mut self, remaining_time: Option<Duration>) {
        match *self {
            Output::Stdout(ref mut stdout) => stdout.refresh(remaining_time),
            Output::File(ref mut file) => file.refresh(remaining_time),
        }
    }

    fn end_handler(&mut self) {
        match *self {
            Output::Stdout(ref mut stdout) => stdout.end_handler(),
            Output::File(ref mut file) => file.end_handler(),
        }
    }
}
