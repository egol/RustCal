/// Timer based off of the pomodoro technique
/// Allows to configure intervals 
/// 


use std::sync::{Arc, Mutex};

// External Dependencies ------------------------------------------------------
use cursive::theme::ColorStyle;
use cursive::traits::*;
use cursive::Cursive;
use cursive::Vec2;
use cursive::Printer;
use cursive::direction::Direction;
use cursive::view::CannotFocus;
use cursive::views::*;
use cursive::event::EventResult;

// Internal
use crate::util::timer::*;

const ASCII_NUMBERS: [&[&str]; 11] = [
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
    &[
        "   ",
        " ° ",
        " ° "
    ], // seperator
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

        for &digit in &digits {
            let ascii_digit = draw_ascii_number(digit);
            for (j, line) in ascii_digit.iter().enumerate() {
                ascii_representation[j].push_str(line);
            }
        }

        // Add colon separator after hours and minutes (except after the last component)
        if i < time_components.len() - 1 {
            let colon_digit = draw_ascii_number(10);
            for (j, line) in colon_digit.iter().enumerate() {
                ascii_representation[j].push_str(line);
            }
        }
    }

    ascii_representation
}

pub struct Pomodoro {
    timer: Arc<Mutex<PomodoroTimer>>,
    enabled: bool,
    size: Vec2,
    // focused: Option<Vec2>,
}

impl Pomodoro {
    pub fn new(timer: Arc<Mutex<PomodoroTimer>> ) -> Self {

        Self {
            enabled: true,
            size: (0, 0).into(),
            // focused: None,
            timer: timer,
        }
    }

    pub fn unpause(&mut self) {
        self.timer.lock().unwrap().unpause();
    }

    pub fn pause(&mut self) {
        self.timer.lock().unwrap().pause();
    }

    pub fn skip(&mut self) {
        self.timer.lock().unwrap().skip();
    }

    pub fn reset(&mut self) {
        self.timer.lock().unwrap().reset();
    }

    pub fn work_sessions_completed(&self) -> usize{
        self.timer.lock().unwrap().work_sessions_completed
    }

    pub fn short_breaks_completed(&self) -> usize {
        self.timer.lock().unwrap().short_breaks_completed
    }

    pub fn long_breaks_completed(&self) -> usize {
        self.timer.lock().unwrap().long_breaks_completed
    }

    pub fn total_work_time_str(&self) -> String{
        self.timer.lock().unwrap().total_work_time_str()
    }
    
    fn draw_timer(&self, p: &Printer) {
        let remaining = self.timer.lock().unwrap().time_remaining();
        let hours = remaining.num_hours();
        let minutes = remaining.num_minutes() % 60;
        let seconds = remaining.num_seconds() % 60;

        let ascii_time = format_time_to_ascii(hours, minutes, seconds);

        for (i, line) in ascii_time.iter().enumerate() {
            p.print((0, i), line);
        }
    }

    pub fn update_durations(&mut self, work: i64, short_break: i64, long_break: i64) {
        self.timer.lock().unwrap().update_durations(work, short_break, long_break);
    }
}

impl View for Pomodoro {

    fn draw(&self, printer: &Printer) {
        printer.with_color(ColorStyle::primary(), |printer| {
            self.draw_timer(printer);
        });
        printer.with_color(ColorStyle::secondary(), |printer| {
            printer.print((0, 4), &self.total_work_time_str());
            printer.print((0, 5), &format!("Work Sessions: {}", self.work_sessions_completed()));
            printer.print((0, 6), &format!("Breaks: {}", self.short_breaks_completed() + self.long_breaks_completed()));
        });
        // printer.print((0, 6), &format!("Short Breaks: {}", self.short_breaks_completed));
        // printer.print((0, 7), &format!("Long Breaks: {}", self.long_breaks_completed));
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        self.size = (42, 8).into();
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
                
                .child(LinearLayout::horizontal()
                    .child(Button::new("Start", |s| {
                        s.call_on_name("pomodoro", |view: &mut Pomodoro| {
                            view.unpause();
                        });
                    }))
                    .child(TextView::new("  "))
                    .child(Button::new("Pause", |s| {
                        s.call_on_name("pomodoro", |view: &mut Pomodoro| {
                            view.pause();
                        });
                    }))
                    .child(TextView::new("  "))
                    .child(Button::new("Skip", |s| {
                        s.call_on_name("pomodoro", |view: &mut Pomodoro| {
                            view.skip();
                        });
                    }))
                    .child(TextView::new("  "))
                    .child(Button::new("Reset", |s| {
                        s.call_on_name("pomodoro", |view: &mut Pomodoro| {
                            view.reset();
                        });
                    }))
                    .child(TextView::new("  "))
                    .child(Button::new("Edit", |s| {
                        // Open the Edit Dialog
                        s.add_layer(
                            Dialog::new()
                                .title("Edit Durations")
                                .content(
                                    LinearLayout::vertical()
                                        .child(TextView::new("Work Duration (minutes):"))
                                        .child(EditView::new().with_name("work_duration").fixed_width(10))
                                        .child(TextView::new("Short Break Duration (minutes):"))
                                        .child(EditView::new().with_name("short_break_duration").fixed_width(10))
                                        .child(TextView::new("Long Break Duration (minutes):"))
                                        .child(EditView::new().with_name("long_break_duration").fixed_width(10))
                                )
                                .button("OK", |s| {
                                    let work = s
                                        .call_on_name("work_duration", |view: &mut EditView| {
                                            view.get_content()
                                        })
                                        .unwrap()
                                        .parse::<i64>()
                                        .unwrap_or(25);

                                    let short_break = s
                                        .call_on_name("short_break_duration", |view: &mut EditView| {
                                            view.get_content()
                                        })
                                        .unwrap()
                                        .parse::<i64>()
                                        .unwrap_or(5);

                                    let long_break = s
                                        .call_on_name("long_break_duration", |view: &mut EditView| {
                                            view.get_content()
                                        })
                                        .unwrap()
                                        .parse::<i64>()
                                        .unwrap_or(15);

                                    s.call_on_name("pomodoro", |view: &mut Pomodoro| {
                                        view.update_durations(work, short_break, long_break);
                                        view.reset();
                                    });

                                    s.pop_layer(); // Close the edit dialog
                                })
                                .button("Cancel", |s| {
                                    s.pop_layer(); // Close the edit dialog without saving
                                })
                        );
                    }))
                )

        )
        .title("Pomodoro Timer")
        .button("Ok", |s| {
            s.call_on_name("pomodoro", |view: &mut Pomodoro| {
                view.pause();
            });
            s.pop_layer();
        }),
    );

}