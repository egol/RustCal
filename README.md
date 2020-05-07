# RustCal
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

TUI calendar app written in rust
#### Built with [Cursive](https://github.com/gyscos/cursive)
 
<p align="center">
  <img width="460" height="300" src="Capture3333.PNG">
  <img width="460" height="300" src="123.PNG">
</p>

## Building the project

 - ### [Available backends](https://github.com/gyscos/cursive/wiki/Backends)

    - `ncurses-backend` _(default)_: uses the [ncurses-rs] library directly. Currently only compatible on Linux and macOS. 
    - `pancurses-backend`: uses the [pancurses] library, which forwards calls to [ncurses-rs] on Linux/macOS or [pdcurses-sys] on Windows. 
    - `termion-backend`: uses the pure-rust [termion] library. Works on Linux, macOS, and Redox.
    - `crossterm-backend`: uses the pure-rust [crossterm] library. Works crossplatform, even for windows systems down to version 7.
    - `blt-backend`: uses the cross-platform [BearLibTerminal.rs] binding. Works on Linux and Windows.

## Features
 * Month by Month displaying
 * Basic Todo List functionality
 * Flexible cross-platform TUI library that supports Linux, Windows and Mac
 * Basic saving functionality through a single accssesible json file (press 'k' to save)
## TODO
 - Impliment a weather forecast display using a user provided API key
 - An ascii art digital clock 
 - Todo-list event naming and ability to set them on specific time
 - Custom UI theme (dark mode)
 - Complete code rewrite
 - Ability to grab events from other calendar applications
 - Windows notifications for events
 - Companion phone app
