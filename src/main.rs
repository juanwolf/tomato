extern crate clap;

use clap::{App, SubCommand};

fn main() {

    let matches = App::new("Tomato")
        .version("0.1.0")
        .author("Jean-Loup Adde <spam@juanwolf.fr>")
        .about("Integrated Pomodoro Timer")
        .args_from_usage("-c, --config=[FILE] 'Configuration file'
                          -d, --debug 'Turn debugging information on'")
        .subcommand(SubCommand::with_name("start")
        .about("Starts a pomodoro timer"))
        .get_matches();

    if let Some(c) = matches.value_of("config") {
        println!("Value for config: {}", c);
    }

    if matches.is_present("debug") {
        println!("Debug mode is on");
    } else {
        println!("Debug mode is off");
    }

    if let Some(_matches) = matches.subcommand_matches("start") {
        println!("Starting a pomodoro...");
    }
}
