# Tomoto

[![CircleCI](https://circleci.com/gh/juanwolf/tomato/tree/master.svg?style=svg)](https://circleci.com/gh/juanwolf/tomato/tree/master)

Pomodoro timer integrated with different services. Have you ever got distracted when you just started a pomodoro? That's the story of my life. What about preventing the distraction directly? Removing slack notification for example, or any notifications. And what if you could even notify the people instead that you're in the middle of something? It would be even better!

##

## Run it

```
git clone https://github.com/juanwolf/tomato.git
cd tomato
cargo run -- start
```

## Configuration

Tomato uses TOML as configuration language. The `~/.tomato.toml` file will be loaded by default. You can override this value by specifying the `-c` argument.

### Format

Every single output/module as its own section. At the minute tomato can only deal with one output at a time. Here's the default configuration file:

```
pomodoro_duration = 1500 # Default pomodoro duration in seconds
refresh_rate = 2 # Default refresh_rate of the outputs in seconds

[outputs]
  [outputs.stdout]
  show_percent = false # Display the percentage achieved of this pomodoro

  [outputs.file]
  path = "$HOME/.tomato" # The path to store the time left of the pomodoro in a file. (Useful for tmux)
```

The output sections are optionnal. As only one output can be used by tomato at the minute, you'll need only one output configuration section. Choose well between stdout or file! (Others will come and multi outputs support as well!)

## Planning

I'll try to keep everything on one place: https://github.com/juanwolf/tomato/projects

## Contributions

At the minute the project is pretty useless so just give me some time to release something I would be proud of :smile:. I'll accept contributions after the v1.0.0 release.

## License

MIT License
