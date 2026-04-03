#[macro_use]
extern crate serde_derive;

use std::cmp;
use std::collections::HashMap;
use std::io::{self, Read, Write};

use chrono::{Datelike, Duration, Local, NaiveDate};

#[path = "../util/mod.rs"]
mod util;

use util::file::{read, save_data};
use util::month::{month_to_string, Month};
use util::textevent::TextEvent;

#[derive(Clone, Copy, PartialEq, Eq)]
enum ViewMode {
    Calendar,
    Day,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Key {
    Up,
    Down,
    Left,
    Right,
    Enter,
    Backspace,
    Char(char),
    Unknown,
}

struct App {
    today: NaiveDate,
    selected: NaiveDate,
    month: NaiveDate,
    events: HashMap<NaiveDate, Vec<TextEvent>>,
    mode: ViewMode,
    day_cursor: usize,
    status: String,
}

impl App {
    fn new() -> Self {
        let today = Local::now().date_naive();
        Self {
            today,
            selected: today,
            month: month_start(today),
            events: read(),
            mode: ViewMode::Calendar,
            day_cursor: 0,
            status: String::from(
                "Arrow keys move. Enter opens a day. 't' adds a task. 's' saves. 'q' quits.",
            ),
        }
    }

    fn render(&self, stdout: &mut io::Stdout) -> io::Result<()> {
        write!(stdout, "\x1b[2J\x1b[H\x1b[?25l")?;
        writeln!(stdout, "RustCal Browser Demo")?;
        writeln!(stdout, "{}", "=".repeat(72))?;
        writeln!(
            stdout,
            "{} {}",
            month_to_string(self.month.month() as i32),
            self.month.year()
        )?;
        writeln!(
            stdout,
            "Selected: {}   Save file: /data/test.json",
            self.selected.format("%Y-%m-%d")
        )?;
        writeln!(stdout)?;

        match self.mode {
            ViewMode::Calendar => self.render_calendar(stdout)?,
            ViewMode::Day => self.render_day(stdout)?,
        }

        writeln!(stdout)?;
        writeln!(stdout, "{}", "-".repeat(72))?;
        writeln!(stdout, "{}", self.status)?;
        stdout.flush()
    }

    fn render_calendar(&self, stdout: &mut io::Stdout) -> io::Result<()> {
        writeln!(stdout, " Mo  Tu  We  Th  Fr  Sa  Su")?;

        let first = self.month;
        let offset = first.weekday().num_days_from_monday() as i64;
        let start = first - Duration::days(offset);

        for week in 0..6 {
            for day in 0..7 {
                let current = start + Duration::days((week * 7 + day) as i64);
                let in_month = current.month() == self.month.month();
                let is_today = current == self.today;
                let is_selected = current == self.selected;
                let has_tasks = self
                    .events
                    .get(&current)
                    .map(|items| !items.is_empty())
                    .unwrap_or(false);

                let mut label =
                    format!("{:>2}{}", current.day(), if has_tasks { "*" } else { " " });

                if !in_month {
                    label = format!("\x1b[2m{}\x1b[0m", label);
                }
                if is_today {
                    label = format!("\x1b[1m{}\x1b[0m", label);
                }
                if is_selected {
                    label = format!("\x1b[7m{}\x1b[0m", label);
                }

                write!(stdout, " {label}")?;
            }
            writeln!(stdout)?;
        }

        writeln!(stdout)?;
        writeln!(stdout, "Upcoming Tasks")?;
        writeln!(stdout, "{}", "-".repeat(72))?;

        let mut upcoming = self.upcoming_tasks();
        if upcoming.is_empty() {
            writeln!(
                stdout,
                "No tasks yet. Press 't' to add one to the selected day."
            )?;
        } else {
            for (date, task) in upcoming.drain(..cmp::min(8, upcoming.len())) {
                let marker = if task.completed { "[x]" } else { "[ ]" };
                writeln!(
                    stdout,
                    "{} {} {}",
                    date.format("%a %Y-%m-%d"),
                    marker,
                    task.content
                )?;
            }
        }

        writeln!(stdout)?;
        writeln!(stdout, "Calendar Keys: arrows move | Enter open day | n/p month | t new task | s save | q quit")?;
        Ok(())
    }

    fn render_day(&self, stdout: &mut io::Stdout) -> io::Result<()> {
        writeln!(stdout, "Tasks for {}", self.selected.format("%A, %B %d %Y"))?;
        writeln!(stdout, "{}", "-".repeat(72))?;

        match self.events.get(&self.selected) {
            Some(tasks) if !tasks.is_empty() => {
                for (index, task) in tasks.iter().enumerate() {
                    let cursor = if index == self.day_cursor { ">" } else { " " };
                    let state = if task.completed { "[x]" } else { "[ ]" };
                    writeln!(stdout, "{} {} {}", cursor, state, task.content)?;
                }
            }
            _ => {
                writeln!(stdout, "No tasks for this day yet.")?;
            }
        }

        writeln!(stdout)?;
        writeln!(stdout, "Day Keys: arrows move selection | t add | x toggle | d delete | b back | s save | q quit")?;
        Ok(())
    }

    fn upcoming_tasks(&self) -> Vec<(NaiveDate, TextEvent)> {
        let mut items = self
            .events
            .iter()
            .filter(|(date, tasks)| **date >= self.today && !tasks.is_empty())
            .flat_map(|(date, tasks)| tasks.iter().cloned().map(move |task| (*date, task)))
            .collect::<Vec<_>>();

        items.sort_by_key(|(date, task)| (*date, task.completed, task.content.clone()));
        items
    }

    fn handle_key(&mut self, key: Key, stdout: &mut io::Stdout) -> io::Result<bool> {
        match self.mode {
            ViewMode::Calendar => self.handle_calendar_key(key, stdout),
            ViewMode::Day => self.handle_day_key(key, stdout),
        }
    }

    fn handle_calendar_key(&mut self, key: Key, stdout: &mut io::Stdout) -> io::Result<bool> {
        match key {
            Key::Up => self.move_selection(-7),
            Key::Down => self.move_selection(7),
            Key::Left => self.move_selection(-1),
            Key::Right => self.move_selection(1),
            Key::Enter => {
                self.mode = ViewMode::Day;
                self.day_cursor = 0;
                self.status = format!("Viewing {}", self.selected.format("%Y-%m-%d"));
            }
            Key::Char('n') => {
                self.month = shift_month(self.month, 1);
                self.selected = clamp_to_month_day(self.selected, self.month);
                self.status = String::from("Moved to next month");
            }
            Key::Char('p') => {
                self.month = shift_month(self.month, -1);
                self.selected = clamp_to_month_day(self.selected, self.month);
                self.status = String::from("Moved to previous month");
            }
            Key::Char('t') => self.add_task(stdout)?,
            Key::Char('s') => self.save(),
            Key::Char('q') => return Ok(false),
            _ => {}
        }

        Ok(true)
    }

    fn handle_day_key(&mut self, key: Key, stdout: &mut io::Stdout) -> io::Result<bool> {
        match key {
            Key::Up => {
                if self.day_cursor > 0 {
                    self.day_cursor -= 1;
                }
            }
            Key::Down => {
                let len = self.selected_tasks_len();
                if self.day_cursor + 1 < len {
                    self.day_cursor += 1;
                }
            }
            Key::Char('b') => {
                self.mode = ViewMode::Calendar;
                self.status = format!("Back on {}", self.selected.format("%Y-%m-%d"));
            }
            Key::Char('t') => self.add_task(stdout)?,
            Key::Char('x') => self.toggle_task(),
            Key::Char('d') => self.delete_task(),
            Key::Char('s') => self.save(),
            Key::Char('q') => return Ok(false),
            _ => {}
        }

        Ok(true)
    }

    fn add_task(&mut self, stdout: &mut io::Stdout) -> io::Result<()> {
        if let Some(task) = prompt(stdout, "New task (blank cancels): ")? {
            let task = task.trim();
            if !task.is_empty() {
                self.events
                    .entry(self.selected)
                    .or_default()
                    .push(TextEvent::new(task.to_string()));
                self.day_cursor = self.selected_tasks_len().saturating_sub(1);
                self.status = format!("Added task on {}", self.selected.format("%Y-%m-%d"));
            } else {
                self.status = String::from("Task creation cancelled");
            }
        } else {
            self.status = String::from("Task creation cancelled");
        }

        Ok(())
    }

    fn toggle_task(&mut self) {
        if let Some(tasks) = self.events.get_mut(&self.selected) {
            if let Some(task) = tasks.get_mut(self.day_cursor) {
                task.completed = !task.completed;
                self.status = if task.completed {
                    String::from("Task marked complete")
                } else {
                    String::from("Task marked incomplete")
                };
                return;
            }
        }

        self.status = String::from("No task selected");
    }

    fn delete_task(&mut self) {
        if let Some(tasks) = self.events.get_mut(&self.selected) {
            if self.day_cursor < tasks.len() {
                tasks.remove(self.day_cursor);
                if tasks.is_empty() {
                    self.events.remove(&self.selected);
                    self.day_cursor = 0;
                } else if self.day_cursor >= tasks.len() {
                    self.day_cursor = tasks.len().saturating_sub(1);
                }
                self.status = String::from("Task removed");
                return;
            }
        }

        self.status = String::from("No task selected");
    }

    fn save(&mut self) {
        save_data(&self.events);
        self.status = String::from("Saved to /data/test.json");
    }

    fn move_selection(&mut self, delta_days: i64) {
        self.selected += Duration::days(delta_days);
        self.month = month_start(self.selected);
        self.day_cursor = 0;
        self.status = format!("Selected {}", self.selected.format("%Y-%m-%d"));
    }

    fn selected_tasks_len(&self) -> usize {
        self.events
            .get(&self.selected)
            .map(|tasks| tasks.len())
            .unwrap_or(0)
    }
}

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    let mut app = App::new();
    app.render(&mut stdout)?;

    loop {
        let key = read_key()?;
        if !app.handle_key(key, &mut stdout)? {
            break;
        }
        app.render(&mut stdout)?;
    }

    write!(
        stdout,
        "\x1b[2J\x1b[H\x1b[?25hThanks for trying RustCal in the browser.\r\n"
    )?;
    stdout.flush()
}

fn read_key() -> io::Result<Key> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut byte = [0_u8; 1];
    handle.read_exact(&mut byte)?;

    let key = match byte[0] {
        b'\r' | b'\n' => Key::Enter,
        8 | 127 => Key::Backspace,
        0x1b => {
            let mut seq = [0_u8; 2];
            if handle.read_exact(&mut seq).is_ok() && seq[0] == b'[' {
                match seq[1] {
                    b'A' => Key::Up,
                    b'B' => Key::Down,
                    b'C' => Key::Right,
                    b'D' => Key::Left,
                    _ => Key::Unknown,
                }
            } else {
                Key::Unknown
            }
        }
        value => Key::Char((value as char).to_ascii_lowercase()),
    };

    Ok(key)
}

fn prompt(stdout: &mut io::Stdout, label: &str) -> io::Result<Option<String>> {
    write!(stdout, "\r\n{}{}", label, "\x1b[K")?;
    stdout.flush()?;

    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut input = String::new();

    loop {
        let mut byte = [0_u8; 1];
        handle.read_exact(&mut byte)?;
        match byte[0] {
            b'\r' | b'\n' => {
                writeln!(stdout)?;
                stdout.flush()?;
                return Ok(Some(input));
            }
            8 | 127 => {
                if !input.is_empty() {
                    input.pop();
                    write!(stdout, "\x08 \x08")?;
                    stdout.flush()?;
                }
            }
            0x1b => return Ok(None),
            value if value.is_ascii_graphic() || value == b' ' => {
                let ch = value as char;
                input.push(ch);
                write!(stdout, "{ch}")?;
                stdout.flush()?;
            }
            _ => {}
        }
    }
}

fn month_start(date: NaiveDate) -> NaiveDate {
    NaiveDate::from_ymd_opt(date.year(), date.month(), 1).expect("valid month start")
}

fn shift_month(month_start_date: NaiveDate, delta: i32) -> NaiveDate {
    let mut year = month_start_date.year();
    let mut month = month_start_date.month() as i32 + delta;

    while month < 1 {
        month += 12;
        year -= 1;
    }

    while month > 12 {
        month -= 12;
        year += 1;
    }

    NaiveDate::from_ymd_opt(year, month as u32, 1).expect("valid shifted month")
}

fn clamp_to_month_day(selected: NaiveDate, month_start_date: NaiveDate) -> NaiveDate {
    let days = days_in_month(month_start_date.year(), month_start_date.month());
    let day = cmp::min(selected.day(), days);
    NaiveDate::from_ymd_opt(month_start_date.year(), month_start_date.month(), day)
        .expect("valid clamped day")
}

fn days_in_month(year: i32, month: u32) -> u32 {
    let month: Month = (month - 1).into();
    month.number_of_days(year) as u32
}
