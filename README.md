# RustCal
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE) [![GitHub release](https://img.shields.io/github/release/egol/RustCal?include_prereleases=&sort=semver&color=orange)](https://github.com/egol/RustCal/releases/)


Terminal based calendar app written in rust
#### Built with [Cursive](https://github.com/gyscos/cursive)
 
<p align="center">
  <img height=300 src="./images/3.PNG">
  <img height=300 src="./images/4.PNG">
</p>
pictured above: version 0.1

## Features
 * Month by Month displaying
 * Basic Todo List functionality
 * Terminal based UI
 * Flexible cross-platform TUI library that supports Linux, Windows and Mac
 * Basic saving functionality through a single json file (press 'k' to save)
 * A realtime clock
## Goals
 - Todo-list time based granularity + striking off completed tasks
 - Custom UI theme (dark mode)

## Building the project

 - ### [Available backends](https://github.com/gyscos/cursive/wiki/Backends)

    - `ncurses-backend` _(default)_: uses the [ncurses-rs] library directly. Currently only compatible on Linux and macOS. 
    - `pancurses-backend`: uses the [pancurses] library, which forwards calls to [ncurses-rs] on Linux/macOS or [pdcurses-sys] on Windows. 
    - `termion-backend`: uses the pure-rust [termion] library. Works on Linux, macOS, and Redox.
    - `crossterm-backend`: uses the pure-rust [crossterm] library. Works crossplatform, even for windows systems down to version 7.
    - `blt-backend`: uses the cross-platform [BearLibTerminal.rs] binding. Works on Linux and Windows.
 - ### Instructions
    1. clone the git repository
    2. modify the toml file line pictured below with the backend you desire
    ```[dependencies.cursive]
   version = "0.14"
   default-features = false
   features = ["<Your backend here>"]
   ```
    3. use the command `cargo run` to launch the application

## User Manual
* `Left click` on mouse can be used to click on all buttons
* `Right click` on mouse opens up todo list at the date clicked
* `Arrow keys` are used to navigate without mouse
* `Tab` cycles through all buttons
* `Enter` opens todo list at selected day
* `K` saves the state to a json file
* Date in the top left corner navigates to current day in calendar when clicked
