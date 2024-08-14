/// Timer based off of the pomodoro technique
/// Allows to configure intervals 
/// 


use std::sync::{Arc, Mutex};

// External Dependencies ------------------------------------------------------
use chrono::{Local, DateTime, Duration};
use cursive::traits::*;
use cursive::Cursive;
use cursive::Vec2;
use cursive::Printer;
use cursive::direction::Direction;
use cursive::view::CannotFocus;
use cursive::views::*;
use cursive::event::{Callback, Event, EventResult, Key, MouseButton, MouseEvent};
use log::info;

// Internal
use crate::util::timer::*;

const ASCII_NUMBERS: [&[&str]; 10] = [
    &[
        "  _   ",
        " / \\  ",
        " \\_/  "
    ], // 0
    &[
        "      ",
        "  /|  ",
        "   |  "
    ], // 1
    &[
        " __   ",
        "  _)  ",
        " /__  "
    ], // 2
    &[
        " __   ",
        "  _)  ",
        " __)  "
    ], // 3
    &[
        "      ",
        " |__| ",
        "    | "
    ], // 4
    &[
        "  ___  ",
        " |__   ",
        " ___)  "
    ], // 5
    &[
        "  __  ",
        " /__  ",
        " \\__) "
    ], // 6
    &[
        " ___  ",
        "   /  ",
        "  /   "
    ], // 7
    &[
        "  __  ",
        " (__) ",
        " (__) "
    ], // 8
    &[
        "  __  ",
        " (__\\ ",
        "  __/ "
    ], // 9
];

// Mini
//      _   _          _    _   __   _    _  
//  /|   )  _)  |_|_  |_   |_    /  (_)  (_| 
//   |  /_  _)    |    _)  |_)  /   (_)    |                                       
// Straight
//       __   __          __   __   ___   __    __  
//   /|   _)   _)  |__|  |_   /__     /  (__)  (__\ 
//    |  /__  __)     |  __)  \__)   /   (__)   __/ 

fn draw_ascii_number(number: usize) -> Vec<&'static str> {
    ASCII_NUMBERS[number].to_vec()
}

fn format_time_to_ascii(hours: i64, minutes: i64, seconds: i64) -> Vec<String> {
    let time_components = [hours, minutes, seconds];
    let mut ascii_representation = vec![String::new(); 6];

    for (i, &component) in time_components.iter().enumerate() {
        let digits: Vec<_> = format!("{:02}", component)
            .chars()
            .map(|d| d.to_digit(10).unwrap() as usize)
            .collect();

        info!("{:?}", digits);

        for &digit in &digits {
            let ascii_digit = draw_ascii_number(digit);
            for (j, line) in ascii_digit.iter().enumerate() {
                ascii_representation[j].push_str(line);
            }
        }
    }

    ascii_representation
}

pub struct Pomodoro {
    enabled: bool,
    size: Vec2,
    focused: Option<Vec2>,
    timer: Arc<Mutex<CountdownTimer>>,
    work_duration: Duration,
    short_break_duration: Duration,
    long_break_duration: Duration,
    on_break: bool,
    break_count: usize,
    total_work_time: Duration,
    work_sessions_completed: usize,
    short_breaks_completed: usize,
    long_breaks_completed: usize,
}

impl Pomodoro {
    pub fn new(work_duration: Duration, short_break_duration: Duration, long_break_duration: Duration, timer: Arc<Mutex<CountdownTimer>> ) -> Self {

        Self {
            enabled: true,
            size: (0, 0).into(),
            focused: None,
            timer,
            work_duration,
            short_break_duration,
            long_break_duration,
            on_break: false,
            break_count: 0,
            total_work_time: Duration::zero(),
            work_sessions_completed: 0,
            short_breaks_completed: 0,
            long_breaks_completed: 0,
        }
    }

    pub fn unpause(&mut self) {

        self.timer.lock().unwrap().start();
        // self.timer.start();
    }

    pub fn pause(&mut self) {

        let mut timer_mut = self.timer.lock().unwrap();

        timer_mut.pause();
        if !self.on_break {
            self.total_work_time += self.work_duration - timer_mut.time_remaining();
        }
    }

    pub fn reset(&mut self) {
        self.on_break = false;
        self.break_count = 0;
        self.timer.lock().unwrap().reset(self.work_duration);
        self.work_sessions_completed = 0;
        self.short_breaks_completed = 0;
        self.long_breaks_completed = 0;
    }

    pub fn skip(&mut self) {
        if self.on_break {
            // Moving from break back to work
            self.on_break = false;
            self.timer.lock().unwrap().reset(self.work_duration);
    
            // Update tally for the break that was just completed
            self.break_count += 1;
            if self.break_count % 4 == 0 {
                self.long_breaks_completed += 1;
            } else {
                self.short_breaks_completed += 1;
            }
        } else {
            // Moving from work to break
            self.on_break = true;
            self.work_sessions_completed += 1;
    
            if self.break_count % 4 == 3 {
                // Every fourth break is a long break (after 3 short breaks)
                self.timer.lock().unwrap().reset(self.long_break_duration);
            } else {
                self.timer.lock().unwrap().reset(self.short_break_duration);
            }
        }
        self.timer.lock().unwrap().start();
    }

    pub fn handle_timer_event(&mut self) -> bool {
        if self.timer.lock().unwrap().is_time_up() {
            self.skip();
            true
        } else {
            false
        }
    }

    fn draw_list(&self, p: &Printer) {
        let remaining = self.timer.lock().unwrap().time_remaining();
        let hours = remaining.num_hours();
        let minutes = remaining.num_minutes() % 60;
        let seconds = remaining.num_seconds() % 60;

        let ascii_time = format_time_to_ascii(hours, minutes, seconds);

        for (i, line) in ascii_time.iter().enumerate() {
            p.print((0, i), line);
        }
    }

    fn total_work_time_str(&self) -> String {
        format!(
            "Total Work Time: {:02}:{:02}:{:02}",
            self.total_work_time.num_hours(),
            self.total_work_time.num_minutes() % 60,
            self.total_work_time.num_seconds() % 60
        )
    }

    fn tally_str(&self) -> String {
        format!(
            "Work Sessions: {}\nShort Breaks: {}\nLong Breaks: {}",
            self.work_sessions_completed,
            self.short_breaks_completed,
            self.long_breaks_completed
        )
    }
}

impl View for Pomodoro {

    fn draw(&self, printer: &Printer) {
        self.draw_list(printer);
        printer.print((0, 7), &self.total_work_time_str());
        printer.print((0, 9), &self.tally_str());
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        self.size = (40, 12).into();
        self.size
    }

    fn take_focus(&mut self, _: Direction) -> Result<EventResult, CannotFocus> {
        self.enabled.then(EventResult::consumed).ok_or(CannotFocus)
    }

}

pub fn create_pomodoro_timer(s: &mut Cursive, pomodoro: Pomodoro) {

    s.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(NamedView::new("pomodoro", pomodoro))
                .child(Button::new("Start", |s| {
                    s.call_on_name("pomodoro", |view: &mut Pomodoro| {
                        view.unpause();
                    });
                }))
                .child(Button::new("Pause", |s| {
                    s.call_on_name("pomodoro", |view: &mut Pomodoro| {
                        view.pause();
                    });
                }))
                .child(Button::new("Skip", |s| {
                    s.call_on_name("pomodoro", |view: &mut Pomodoro| {
                        view.skip();
                    });
                }))
                .child(Button::new("Reset", |s| {
                    s.call_on_name("pomodoro", |view: &mut Pomodoro| {
                        view.reset();
                    });
                })),
        )
        .title("Timer")
        .button("Ok", |s| {
            s.pop_layer();
        }),
    );

}