#[macro_use]
extern crate serde_derive;
extern crate clap;

use std::path::Path;
use std::sync::mpsc;
use std::time::{Duration, SystemTime};
use std::{fs, thread};

use clap::{App, Arg, SubCommand};

mod output;

use output::{Output, PomodoroHandler};

mod config;

use config::Config;

const LOCK_PATH_STR: &str = "/tmp/tomato.lock";

// start will start a new pomodoro in a new thread.
// Quite useless as the program will wait for the thread to die to end this function. :ok_hand:
fn start(
    mut output: Box<Output>,
    pomodoro_duration: Duration,
    refresh_rate: Duration,
    message: Option<&str>,
) {
    let lock_path = Path::new(LOCK_PATH_STR);
    output.start_handler(message);

    if lock_path.exists() {
        println!("Can't start more than one instance of tomato!");
        return;
    }
    let _file = fs::File::create(lock_path);
    let starting_time = SystemTime::now();
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
    //    thread::spawn(move || {
    let mut time_spent = 0;
    while time_spent < pomodoro_duration.as_secs() {
        time_spent = receiver.recv().unwrap();
        let duration_spent = pomodoro_duration - Duration::from_secs(time_spent);
        output.refresh(Some(duration_spent));
    }
    //   });
}

fn get_output(output: &str, config: Config) -> Box<Output> {
    return Box::new(Output::new(output, config));
}

fn main() {
    let matches = App::new("Tomato")
        .version("0.1.0")
        .author("Jean-Loup Adde <spam@juanwolf.fr>")
        .about("Integrated Pomodoro Timer")
        .arg(
            Arg::with_name("config")
                .short("-c")
                .long("--config")
                .value_name("config")
                .help("Config file to use. Default: ~/.tomato.toml"),
        )
        .arg(
            Arg::with_name("output")
                .short("-o")
                .long("--output")
                .value_name("output")
                .help("Specific output. Current values possible: stdout"),
        )
        .arg(
            Arg::with_name("pomodoro_duration")
                .short("-d")
                .long("--pomodoro_duration")
                .value_name("pomodoro_duration")
                .help("Duration of the pomodoro in seconds. Default: 1500 (25min)"),
        )
        .arg(
            Arg::with_name("refresh_rate")
                .short("-r")
                .long("refresh_rate")
                .value_name("refresh_rate")
                .help("The refresh rate of the output in seconds. Default: 5"),
        )
        .subcommand(
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
        )
        .get_matches();

    let output_value = matches.value_of("output").unwrap_or("stdout");
    let pomodoro_duration_input: u64 = matches
        .value_of("pomodoro_duration")
        .unwrap_or("1500")
        .parse()
        .unwrap();
    let refresh_rate_input: u64 = matches
        .value_of("refresh_rate")
        .unwrap_or("5")
        .parse()
        .unwrap();
    let pomodoro_duration = Duration::from_secs(pomodoro_duration_input);
    let refresh_rate = Duration::from_secs(refresh_rate_input);

    let config: Config = config::get_config(None);

    let output = get_output(output_value, config);

    if let Some(matches) = matches.subcommand_matches("start") {
        let message: Option<&str> = matches.value_of("message");
        start(output, pomodoro_duration, refresh_rate, message);
    }
}
