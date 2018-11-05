extern crate clap;
extern crate pbr;

use std::time::{Duration, SystemTime};
use std::sync::mpsc;
use std::thread;

use clap::{App, SubCommand};
use pbr::ProgressBar;

// start will start a new pomodoro in a new thread.
// Quite useless as the program will wait for the thread to die to end this function. :ok_hand: 
fn start() {
    println!("Starting a pomodoro");
    let starting_time = SystemTime::now();
    let refresh_rate = Duration::from_secs(5);
    let pomodoro_duration : Duration = Duration::from_secs(60 * 25);

    let mut pb = ProgressBar::new(pomodoro_duration.as_secs().wrapping_div(refresh_rate.as_secs()));
    pb.show_speed     = false;
    pb.show_time_left = true;
    pb.show_counter   = false;
    pb.show_percent   = false;
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
        pb.inc();
    }
}

fn main() {
    let matches = App::new("Tomato")
        .version("0.1.0")
        .author("Jean-Loup Adde <spam@juanwolf.fr>")
        .about("Integrated Pomodoro Timer")
        .subcommand(SubCommand::with_name("start")
        .about("Starts a pomodoro timer"))
        .get_matches();

    if let Some(_matches) = matches.subcommand_matches("start") {
       start();
    }
}
