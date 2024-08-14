/// countdown timer for keeping track of pomodoro timer

// External Dependencies ------------------------------------------------------
use chrono::{Local, DateTime, Duration};
// use cursive::reexports::time::Duration;

pub struct CountdownTimer {
    duration: Duration,
    start: DateTime<Local>,
    paused: bool,
    pub finished: bool,
}

impl CountdownTimer {
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            start: Local::now(),
            paused: true,
            finished: false,
        }
    }

    pub fn start(&mut self) {
        if self.paused {
            self.start = Local::now();
            self.paused = false;
        }
    }

    pub fn pause(&mut self) {
        if !self.paused {
            self.duration = self.time_remaining();
            self.paused = true;
        }
    }

    pub fn reset(&mut self, duration: Duration) {
        self.duration = duration;
        self.start = Local::now();
        self.paused = true;
    }

    pub fn time_remaining(&self) -> Duration {
        if self.paused {
            self.duration
        } else {
            self.duration - self.time_elapsed()
        }
    }

    pub fn time_elapsed(&self) -> Duration {
        let elapsed = Local::now() - self.start;
        elapsed
    }

    pub fn is_time_up(&self) -> bool {
        self.time_remaining() <= Duration::zero()
    }

    pub fn update_finished_status(&mut self) {
        self.finished = self.is_time_up();
    }
}