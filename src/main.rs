#[macro_use]
extern crate serde_derive;
extern crate clap;

use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, SystemTime};

use clap::{App, Arg, ArgMatches, SubCommand};

mod output;

use output::{Output, PomodoroHandler};

mod config;

use config::Config;

const LOCK_PATH_STR: &str = "/tmp/tomato.lock";

fn start_timer(duration: Duration) -> mpsc::Receiver<Duration> {
    let starting_time = SystemTime::now();
    let (sender, receiver): (mpsc::Sender<Duration>, mpsc::Receiver<Duration>) = mpsc::channel();
    thread::spawn(move || {
        let mut done = false;
        while !done {
            thread::sleep(Duration::from_secs(1));
            match starting_time.elapsed() {
                Ok(elapsed) => {
                    sender.send(elapsed).unwrap();
                    if elapsed.as_secs() > duration.as_secs() {
                        done = true;
                    }
                }
                Err(_elapsed) => {
                    panic!();
                }
            }
        }
    });

    return receiver;
}

// start will start a new in a new thread.
// Quite useless as the program will wait for the thread to die to end this function. :ok_hand:
fn start(mut output: Box<Output>, pomodoro_duration: Duration) -> Box<Output> {
    // TODO: if a pomodoro is already started, send warning message and ask to stop it first.
    // TODO: if not, create a new thread with a timer of 25min like a ticker or something
    let receiver = start_timer(pomodoro_duration);
    //    thread::spawn(move || {
    let mut time_spent = 0;
    while time_spent < pomodoro_duration.as_secs() {
        time_spent = receiver.recv().unwrap().as_secs();
        let duration_spent = pomodoro_duration - Duration::from_secs(time_spent);
        output.refresh(Some(duration_spent));
    }
    return output;
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

fn run(matches: ArgMatches<'static>) -> Result<(), String> {
    let config_path: Option<PathBuf> = match matches.value_of("config") {
        Some(config_path) => Some(PathBuf::from(config_path)),
        None => None,
    };

    let config: Config = config::get_config(config_path);
    //// let refresh_rate = config.refresh_rate;
    let pomodoro_duration = config.pomodoro_duration;
    let short_break_duration = config.break_duration;
    let long_break_duration = config.long_break_duration;

    let outputs = get_outputs(config);

    match matches.subcommand() {
        ("start", Some(m)) => run_start(m, outputs, pomodoro_duration),
        ("break", Some(m)) => run_break(m, outputs, short_break_duration, long_break_duration),
        _ => Ok(()),
    }
}

fn run_start(
    matches: &'static ArgMatches<'static>,
    outputs: Vec<Box<Output>>,
    pomodoro_duration: Duration,
) -> Result<(), String> {
    let message = matches.value_of("message");
    let mut handles: Vec<JoinHandle<()>> = vec![];
    for output in outputs {
        let mut local_output = output.clone();

        let handle = thread::spawn(move || {
            local_output.start_handler(message);
            local_output = start(local_output, pomodoro_duration);
            local_output.end_handler();
        });

        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}

fn run_break(
    matches: &'static ArgMatches<'static>,
    outputs: Vec<Box<Output>>,
    short_break_duration: Duration,
    long_break_duration: Duration,
) -> Result<(), String> {
    let mut timer_duration = short_break_duration;
    if matches.is_present("long") {
        timer_duration = long_break_duration;
    }
    let message = matches.value_of("message");
    let mut handles: Vec<JoinHandle<()>> = vec![];
    for output in outputs {
        let mut local_output = output.clone();

        let handle = thread::spawn(move || {
            local_output.start_handler(message);
            local_output = start(local_output, timer_duration);
            local_output.end_handler();
        });

        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}
fn main() {
    let matches: ArgMatches<'static> = App::new("Tomato")
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
        .subcommand(
            SubCommand::with_name("break")
                .about("Starts a break")
                .arg(
                    Arg::with_name("long")
                        .short("l")
                        .long("long")
                        .help("Long break."),
                )
                .arg(
                    Arg::with_name("message")
                        .short("m")
                        .long("message")
                        .value_name("YourMessage")
                        .help("Add a message to this break")
                        .takes_value(true),
                ),
        )
        .get_matches();

    if let Err(e) = run(matches) {
        panic!("Application error: {}", e);
    }
    //    let lock_path = Path::new(LOCK_PATH_STR);
    //    if lock_path.exists() {
    //        println!("Can't start more than one instance of tomato!");
    //        return;
    //    }
    //    let _file = fs::File::create(lock_path);

    //let _res = fs::remove_file(LOCK_PATH_STR);
}
