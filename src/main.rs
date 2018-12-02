extern crate clap;

use std::path::Path;
use std::sync::mpsc;
use std::time::{Duration, SystemTime};
use std::{fs, thread};

use clap::{App, Arg, SubCommand};

mod output;

use output::{PomodoroHandler, Output};

const LOCK_PATH_STR: &str = "/tmp/tomato.lock";

// start will start a new pomodoro in a new thread.
// Quite useless as the program will wait for the thread to die to end this function. :ok_hand:
fn start(message: Option<&str>, mut output: Box<Output>) {
    let lock_path = Path::new(LOCK_PATH_STR);
    output.start_handler(message);

    if lock_path.exists() {
        println!("Can't start more than one instance of tomato!");
        return;
    }
    let _file = fs::File::create(lock_path);
    let starting_time = SystemTime::now();
    let refresh_rate = Duration::from_secs(5);
    let pomodoro_duration: Duration = Duration::from_secs(60 * 25);

    // TODO: if a pomodoro is already started, send warning message and ask to stop it first.
    // TODO: if not, create a new thread with a timer of 25min like a ticker or something
    let (sender, receiver): (mpsc::Sender<u64>, mpsc::Receiver<u64>) = mpsc::channel();
    thread::spawn(move || {
        let mut done = false;
        while !done {
            thread::sleep(refresh_rate);
            match starting_time.elapsed() {
                Ok(elapsed) => {
                    sender.send(elapsed.as_secs()).unwrap();
                    if elapsed > pomodoro_duration {
                        done = true;
                        let _res = fs::remove_file(LOCK_PATH_STR);
                    };
                }
                Err(_elapsed) => {
                    panic!();
                }
            }
        }
    });

    let mut time_spent = 0;

    while time_spent < pomodoro_duration.as_secs() {
        time_spent = receiver.recv().unwrap();
        let duration_spent = pomodoro_duration - Duration::from_secs(time_spent);

        output.refresh(Some(duration_spent));
    }
}

fn get_output(output: &str) -> Box<Output> {
    return Box::new(Output::new(output, Duration::from_secs(5), Duration::from_secs(60 * 25)))
}

fn main() {
    let matches = App::new("Tomato")
        .version("0.1.0")
        .author("Jean-Loup Adde <spam@juanwolf.fr>")
        .about("Integrated Pomodoro Timer")
        .arg(
            Arg::with_name("output")
                .short("-o")
                .long("--output")
                .value_name("output")
                .help("Specific output. Current values possible: stdout"),
        ).subcommand(
            SubCommand::with_name("start")
                .about("Starts a pomodoro timer")
                .arg(
                    Arg::with_name("message")
                        .short("m")
                        .long("message")
                        .value_name("YourMessage")
                        .help("Add a message to this pomodoro")
                        .takes_value(true),
                ),
        ).get_matches();

    let output_value = matches.value_of("output").unwrap_or("stdout");

    let output = get_output(output_value);

    if let Some(matches) = matches.subcommand_matches("start") {
        let message: Option<&str> = matches.value_of("message");
        start(message, output);
    }
}
