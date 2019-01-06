#[macro_use]
extern crate serde_derive;
extern crate clap;

use std::path::{Path, PathBuf};
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
    output.start_handler(message);
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
                    }
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

fn get_outputs(config: Config) -> Vec<Box<Output>> {
    let mut outputs: Vec<Box<Output>> = Vec::with_capacity(config.outputs_to_use.len());
    let local_config = config.clone();
    for output_string in local_config.outputs_to_use {
        let c = config.clone();
        outputs.push(Box::new(Output::new(&output_string, c)))
    }
    return outputs;
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

    let config_path: Option<PathBuf> = match matches.value_of("config") {
        Some(cp) => Some(PathBuf::from(cp)),
        None => None,
    };

    let config: Config = config::get_config(config_path);

    let pomodoro_duration = config.pomodoro_duration;
    let refresh_rate = config.refresh_rate;

    let outputs = get_outputs(config);

    if let Some(matches) = matches.subcommand_matches("start") {
        let lock_path = Path::new(LOCK_PATH_STR);
        if lock_path.exists() {
            println!("Can't start more than one instance of tomato!");
            return;
        }
        let _file = fs::File::create(lock_path);
        let mut handles = vec![];
        for output in outputs {
            let local_matches = matches.clone();

            let handle = thread::spawn(move || {
                let message: Option<&str> = local_matches.value_of("message");
                start(output, pomodoro_duration, refresh_rate, message);
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let _res = fs::remove_file(LOCK_PATH_STR);
    }
}
