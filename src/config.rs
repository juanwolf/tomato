extern crate toml;

use std::env;
use std::path::{Path, PathBuf};

use std::fs::File;
use std::io::prelude::*;
use std::time::Duration;

use output::file::Config as FileConfig;
use output::stdout::Config as StdoutConfig;

#[derive(Deserialize)]
struct ConfigFromFile {
    pub refresh_rate: u64,
    pub pomodoro_duration: u64,
    pub outputs: OutputConfigFromFile,
}

#[derive(Deserialize)]
struct OutputConfigFromFile {
    pub stdout: Option<StdoutConfig>,
    pub file: Option<FileConfig>,
}

pub struct Config {
    pub refresh_rate: Duration,
    pub pomodoro_duration: Duration,
    pub output_to_use: String,
    pub outputs: OutputConfig,
}

pub struct OutputConfig {
    pub stdout: StdoutConfig,
    pub file: FileConfig,
}

// Where the default config file can be found from the $HOME folder
const DEFAULT_CONFIG_FILE: &str = ".tomato.toml";


pub fn get_default_config() -> Config {
    let default_tomato_file = String::from("./.tomato");
    let default_output = String::from("stdout");

    return Config {
        refresh_rate: Duration::from_secs(2),
        pomodoro_duration: Duration::from_secs(1500),
        output_to_use: default_output,
        outputs: OutputConfig {
            stdout: StdoutConfig {
                show_percent: false,
            },
            file: FileConfig{
                path: default_tomato_file,
            },
        },
    };
}

pub fn get_config(config_path: Option<PathBuf>) -> Config {
    let home_dir = env::var("HOME").unwrap_or_else(|_| {
        panic!("HOME environment variable not found.")
    });
    let default_config_file_path = Path::new(&home_dir).join(DEFAULT_CONFIG_FILE);
    let config_file_path = config_path.unwrap_or(default_config_file_path);

    let mut f = File::open(config_file_path).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("With text:\n{}", contents);

    return parse_config(&contents);
}

fn parse_config(_config: &str) -> Config {
    return get_default_config();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_config_file_returns_empty_config_file() {
        let config = parse_config("");
        let default_config = get_default_config();

        assert_eq!(config.refresh_rate, default_config.refresh_rate);
        assert_eq!(config.pomodoro_duration, default_config.pomodoro_duration);
        assert_eq!(config.output_to_use, default_config.output_to_use);
        assert_eq!(config.outputs.file.path, default_config.outputs.file.path);
        assert_eq!(config.outputs.stdout.show_percent, default_config.outputs.stdout.show_percent);
    }

}
