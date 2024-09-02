/// countdown timer for keeping track of pomodoro timer

// External Dependencies ------------------------------------------------------
use chrono::{Local, DateTime, Duration};

pub struct CountdownTimer {
    duration: Duration,
    start: DateTime<Local>,
    paused: bool,
}

impl CountdownTimer {
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            start: Local::now(),
            paused: true,
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
}

pub struct PomodoroTimer {
    timer: CountdownTimer,
    work_duration: Duration,
    short_break_duration: Duration,
    long_break_duration: Duration,
    on_break: bool,
    break_count: usize,
    total_work_time: Duration,
    pub work_sessions_completed: usize,
    pub short_breaks_completed: usize,
    pub long_breaks_completed: usize,
}

impl PomodoroTimer {
    pub fn new(work_duration: Duration, short_break_duration: Duration, long_break_duration: Duration) -> Self {

        Self {
            timer: CountdownTimer::new(Duration::minutes(25)),
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
        self.timer.start();
    }

    pub fn pause(&mut self) {
        self.timer.pause();
    }

    pub fn time_remaining(&mut self) -> Duration {
        self.timer.time_remaining()
    }

    pub fn reset(&mut self) {
        self.on_break = false;
        self.break_count = 0;
        self.timer.reset(self.work_duration);
        self.work_sessions_completed = 0;
        self.short_breaks_completed = 0;
        self.long_breaks_completed = 0;
        self.total_work_time = Duration::zero();
    }

    pub fn update_finished_status(&mut self) {
        if self.timer.is_time_up() {
            self.skip();
            self.pause();
        }
    }

    pub fn skip(&mut self) {

        if self.on_break {
            // Moving from break back to work
            self.on_break = false;
            self.timer.reset(self.work_duration);
    
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
            self.total_work_time += self.timer.time_elapsed();
    
            if self.break_count % 4 == 3 {
                // Every fourth break is a long break (after 3 short breaks)
                self.timer.reset(self.long_break_duration);
            } else {
                self.timer.reset(self.short_break_duration);
            }
        }
        self.timer.start();
    }

    pub fn total_work_time_str(&self) -> String {
        format!(
            "Total time worked: {:02}:{:02}:{:02}",
            self.total_work_time.num_hours(),
            self.total_work_time.num_minutes() % 60,
            self.total_work_time.num_seconds() % 60
        )
    }

    pub fn update_durations(&mut self, work: i64, short_break: i64, long_break: i64) {
        self.work_duration = Duration::minutes(work);
        self.short_break_duration = Duration::minutes(short_break);
        self.long_break_duration = Duration::minutes(long_break);
    }
}