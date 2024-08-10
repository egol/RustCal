//TODO
//https://docs.rs/cursive/0.14.1/cursive/view/trait.View.html
//implement the needs re-layout function to improve performance

//TODO
// 3. Add more priorities?
// 4. Rework Readme for release
// 5. Add todo list functionality?
// 6. Add in pomodoro timer button
// 7. at 12:37am time shows as 00:37

// STD Dependencies -----------------------------------------------------------
use std::cmp;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::fs;

use std::sync::{Arc, Mutex};

// External Dependencies ------------------------------------------------------
use chrono::{prelude::*, Duration};
use chrono::{Datelike, Weekday};
use cursive::traits::*;
use cursive::Cursive;
use cursive::Vec2;
#[macro_use] extern crate serde_derive;

use cursive::Printer;
use cursive::direction::Direction;
use cursive::view::CannotFocus;
use cursive::theme::*;
use cursive::views::*;
use cursive::view::Position;
use cursive::views::NamedView;
use cursive::views::LayerPosition;
use cursive::event::{Callback, Event, EventResult, Key, MouseButton, MouseEvent};
use cursive::utils::markup::StyledString;

use log::{info, LevelFilter};
use simplelog::{Config, WriteLogger};

mod util;

const TOP: &str=   "  .oooo.   #     .o    #  .oooo.   #   .oooo.  #      .o   #  oooooooo #   .ooo    # ooooooooo # .ooooo.   #  .ooooo.  #    ";
const M1: &str =   " d8P'`Y8b  #   o888    #.dP\"\"Y88b  #.dP\"\"Y88b  #    .d88   # dP\"\"\"\"\"\"\" #  .88'     #d\"\"\"\"\"\"\"8' #d88'   `8. #888' `Y88. #    ";
const M2: &str =r##"888    888 #    888    #      |8P' #      |8P' #  .d'888   #d88888b.   # d88'      #      .8'  #Y88..  .8' #888    888 #    "##  ;
const M3: &str =   "888    888 #    888    #    .d8P'  #    <88b.  #.d'  888   #    `Y88b  #d888P\"Ybo. #     .8'   # `88888b.  # `Vbood888 #    " ;
const M4: &str =   "888    888 #    888    #  .dP'     #     `88b. #88ooo888oo #      |88  #Y88|   |88 #    .8'    #.8'  ``88b #      888' #    ";
const M5: &str =   "`88b  d88' #    888    #.oP     .o #o.   .88P  #     888   #o.   .88P  #`Y88   88P #   .8'     #`8.   .88P #    .88P'  #    ";
const BOT: &str=   " `Y8bd8P'  #   o888o   #8888888888 #`8bd88P'   #    o888o  #`8bd88P'   # `88bod8'  #  .8'      # `boood8'  #  .oP'     #    ";

// const BADGE: &str= "
// ██████╗  ██████╗
// ██╔══██╗██╔════╝
// ██████╔╝██║     
// ██╔══██╗██║     
// ██║  ██║╚██████╗
// ╚═╝  ╚═╝ ╚═════╝
// ";

fn month_to_string(num : i32) -> String{
    match num{
        1 => String::from("January"),
        2 => String::from("February"),
        3 => String::from("March"),
        4 => String::from("April"),
        5 => String::from("May"),
        6 => String::from("June"),
        7 => String::from("July"),
        8 => String::from("August"),    
        9 => String::from("September"),
        10 => String::from("October"),
        11 => String::from("November"),
        12 => String::from("December"),
        _ => String::from("Invalid Month"),
    }
}

fn abbr_month_to_string(num : i32) -> String{
    match num{
        1 => String::from("Jan"),
        2 => String::from("Feb"),
        3 => String::from("Mar"),
        4 => String::from("Apr"),
        5 => String::from("May"),
        6 => String::from("Jun"),
        7 => String::from("Jul"),
        8 => String::from("Aug"),    
        9 => String::from("Sep"),
        10 => String::from("Oct"),
        11 => String::from("Nov"),
        12 => String::from("Dec"),
        _ => String::from("Invalid Month"),
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TextEvent{
    content: String,
    status: i8,
}

impl TextEvent {
    pub fn new(s: String) -> Self {
        Self {
            content: s,
            status: 0,
        }
    }

}

#[derive(Serialize, Deserialize)]
pub struct Storage{
    events: HashMap<NaiveDate, Vec<TextEvent>>,
}

impl Storage {
    pub fn new(e : HashMap<NaiveDate, Vec<TextEvent>>) -> Self {
        Self {
            events: e,
        }
    }
} 

pub struct Clock{
    time: String
}

impl Clock {
    pub fn new() -> Self {
        Self {
            time: get_ascii_time(),
        }
    }

    pub fn update_time(&mut self){
        self.time = get_ascii_time();
    }
}

fn get_ascii_time() -> String{
    let top: Vec<&str> = TOP.split('#').collect();

    let m1: Vec<&str> = M1.split('#').collect();
    let m2: Vec<&str> = M2.split('#').collect();
    let m3: Vec<&str> = M3.split('#').collect();
    let m4: Vec<&str> = M4.split('#').collect();
    let m5: Vec<&str> = M5.split('#').collect();

    let bot: Vec<&str> = BOT.split('#').collect();

    // Get the current local time
    let local: DateTime<Local> = Local::now();

    // Extract hour, minute, and second components
    let mut hour = local.hour();
    let minute = local.minute();
    let second = local.second();

    // Convert to 12-hour format if needed
    if hour > 12 {
        hour -= 12;
    }

    // Format the time components into a string with zero-padding
    let tm = format!("{:02}:{:02}:{:02}", hour, minute, second);

    let top_str: String = tm.chars().map(|x| top[x as usize - '0' as usize]).collect();
    let m1_str: String = tm.chars().map(|x| m1[x as usize - '0' as usize]).collect();
    let m2_str: String = tm.chars().map(|x| m2[x as usize - '0' as usize]).collect();
    let m3_str: String = tm.chars().map(|x| m3[x as usize - '0' as usize]).collect();
    let m4_str: String = tm.chars().map(|x| m4[x as usize - '0' as usize]).collect();
    let m5_str: String = tm.chars().map(|x| m5[x as usize - '0' as usize]).collect();
    let bot_str: String = tm.chars().map(|x| bot[x as usize - '0' as usize]).collect();

    return format!(
        "{}\n{}\n{}\n{}\n{}\n{}\n{}", 
        top_str, m1_str, m2_str, m3_str, m4_str, m5_str, bot_str
    )
}

impl View for Clock {
    fn draw(&self, p: &Printer) {
        let mut y = 0;
        let mut temp = 0;
        let mut len = 100;

        let time = get_ascii_time();

        for (x, c) in time.chars().enumerate() {
            temp += 1;
            if c == '\n'{
                y += 1;
                len = temp;
                temp = 0;
            }
            else{
                p.with_color(ColorStyle::primary(), |printer| {
                    printer.print((x%len, y), &format!("{}", c));
                });
            }

        }
    }    

    fn on_event(&mut self, _event: Event) -> EventResult {

        self.update_time();

        return EventResult::Ignored
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        Vec2::new(78, 8)
    }
}

pub fn create_calendar(year : i32, month : u32, s : Arc<Mutex<Storage>>) -> LinearLayout {

    //TODO MOVE OUT OF FUNCTION
    let mut linear_layout = LinearLayout::vertical();

    let mut h_l = LinearLayout::horizontal();

    h_l.add_child(
        Panel::new(ResizedView::with_fixed_size((9,1), TextView::new("Sunday")))
    );
    h_l.add_child(
        Panel::new(ResizedView::with_fixed_size((9,1), TextView::new("Monday")))
    );
    h_l.add_child(
        Panel::new(ResizedView::with_fixed_size((9,1), TextView::new("Tuesday")))
    );
    h_l.add_child(
        Panel::new(ResizedView::with_fixed_size((9,1), TextView::new("Wednesday")))
    );
    h_l.add_child(
        Panel::new(ResizedView::with_fixed_size((9,1), TextView::new("Thursday")))
    );
    h_l.add_child(
        Panel::new(ResizedView::with_fixed_size((9,1), TextView::new("Friday")))
    );
    h_l.add_child(
        Panel::new(ResizedView::with_fixed_size((9,1), TextView::new("Saturday")))
    );

    linear_layout.add_child(LinearLayout::vertical().child(h_l));

    let calendar = CalendarView::<Utc>::new(Utc.with_ymd_and_hms(year, month, 1, 0, 0, 0).unwrap(), s);

    linear_layout.add_child(calendar.with_name("calendar"));

    linear_layout
}


pub struct CalendarView<T: TimeZone> {
    enabled: bool,
    /// date that is selected
    view_date: DateTime<T>,
    on_select: Option<DateCallback<T>>,
    size: Vec2,
    earliest_date: Option<DateTime<T>>,
    latest_date: Option<DateTime<T>>,
    date: DateTime<chrono::Local>,
    focused: Option<Vec2>,
    /// todays date
    current_date: cursive::XY<i32>,
    storage: Arc<Mutex<Storage>>,
}

type DateCallback<T> = Arc<dyn Fn(&mut Cursive, &DateTime<T>) + Send + Sync>;

impl<T: TimeZone + Send + Sync> CalendarView<T>
where
    T::Offset: Send + Sync,
{
    pub fn new(prev_date: DateTime<T>, s : Arc<Mutex<Storage>>) -> Self {
        Self {
            enabled: true,
            // changed: false,
            size: (0, 0).into(),
            date: Local::now(),
            view_date: prev_date,
            on_select: None,
            earliest_date: None,
            latest_date: None,
            focused: None,
            current_date: (0,0).into(),
            storage: s,
        }
    }

    /// Sets a callback to be used when an a new date is visually selected.
    pub fn set_on_select<F>(&mut self, cb: F)
    where
        F: Fn(&mut Cursive, &DateTime<T>) + Send + Sync + 'static,
    {
        self.on_select = Some(Arc::new(move |s, date| cb(s, date)));
    }

    /// Sets a callback to be used when an a new date is visually selected.
    pub fn on_select<F>(self, cb: F) -> Self
    where
        F: Fn(&mut Cursive, &DateTime<T>) + Send + Sync + 'static,
    {
        self.with(|v| v.set_on_select(cb))
    }

    /// Sets the visually selected date of this view.
    pub fn set_view_date(&mut self, mut date: DateTime<T>) {
        if let Some(ref earliest) = self.earliest_date {
            if date < *earliest {
                date = earliest.clone();
            }
        }

        if let Some(ref latest) = self.latest_date {
            if date > *latest {
                date = latest.clone();
            }
        }

        self.view_date = date;
    }
}

impl<T: TimeZone + Send + Sync> CalendarView<T>
where
    T::Offset: Send + Sync,
{
    
    fn date_available(&self, date: &DateTime<T>) -> bool {
        if let Some(ref earliest) = self.earliest_date {
            if *date < *earliest {
                return false;
            }
        }

        if let Some(ref latest) = self.latest_date {
            if *date > *latest {
                return false;
            }
        }

        true
    }

    fn draw_days(&self, printer: &Printer) {
        let year = self.view_date.year();
        let month: util::month::Month = self.view_date.month0().into();
        let month_start = self.view_date.with_day0(0).unwrap();

        let active_day = self.date.day0() as i32;
        let view_day = self.view_date.day0() as i32;

        let d_month = self.date.month0() as i32 - self.view_date.month0() as i32;
        let d_year = self.date.year() - year;

        let month_days = month.number_of_days(year);
        let prev_month_days = month.prev_number_of_days(year);

        let first_week_day = month_start.weekday() as i32;

        // let mut counter = 0;

        // Draw days
        let w_offset: i32 = 0;
        let d_shift = ((1 as i32 - w_offset) + 7) % 7;
        let d_offset = (first_week_day + d_shift) % 7;
    
        for (index, i) in (-d_offset..-d_offset + 42).enumerate() {
            let (day_number, month_offset) = if i < 0 {
                (prev_month_days + i, -1)
            } else if i > month_days - 1 {
                (i - month_days, 1)
            } else {
                (i, 0)
            };

            if let Some(exact_date) =
            date_from_cell_offset(&self.view_date, Some(day_number), 0, 0, month_offset, 0)
            {
                let color = if !self.date_available(&exact_date) {
                    ColorStyle::tertiary()
                } else if i < 0 {
                    if active_day == prev_month_days + i && d_month == -1 && d_year == 0 {
                        if self.enabled && printer.focused {
                            ColorStyle::highlight_inactive()
                        } else {
                            ColorStyle::secondary()
                        }
                    } else {
                        ColorStyle::secondary()
                    }
                } else if i > month_days - 1 {
                    if active_day == i - month_days && d_month == 1 && d_year == 0 {
                        if self.enabled && printer.focused {
                            ColorStyle::highlight_inactive()
                        } else {
                            ColorStyle::secondary()
                        }
                    } else {
                        ColorStyle::secondary()
                    }
                } else if view_day == i {
                    if self.enabled && printer.focused {
                        ColorStyle::highlight()
                    } else {
                        ColorStyle::primary()
                    }
                } else if active_day == i && d_month == 0 && d_year == 0 {
                    if self.enabled {
                        ColorStyle::highlight_inactive()
                    } else {
                        ColorStyle::primary()
                    }
                } else {
                    ColorStyle::primary()
                };

                let num = index as i32;

                // Draw day number
                let (x, y) = (num%7*11, num/7*6);
                
                // Borrow the reference to storage using the async Arc and Mutex combination
                let storage_ref = self.storage.lock().unwrap();
                let events = storage_ref.events
                                .get(&NaiveDate::from_ymd_opt(exact_date.year(), exact_date.month(), exact_date.day()).unwrap())
                                .unwrap_or(&Vec::new()).clone();
                    
                let mut past = true;

                if (exact_date.day0() >= active_day as u32 && d_month == 0 && d_year == 0) || d_year < 0 || (d_month < 0 && d_year <= 0){
                    past = false;
                }
                
                let mut totals = vec![0,0,0];

                // totals up the status numbers for the calendar display
                for a in events.iter(){
                    totals[a.status as usize] += 1;
                }

                self.draw_cell(printer, x as u8, y as u8, format!("{:>2}", day_number + 1), color, totals, past);
            }
        }

    }

    fn draw_cell (&self, p: &Printer, offset_x : u8, offset_y : u8, day : String, color : ColorStyle, nums : Vec<i32>, past : bool) {
        // let x_max = p.size.x as u8;
        // let y_max = p.size.y as u8;

        let x_max : u8 = 10;
        let y_max : u8 = 5;

        // prints one calendar square
        for x in 0..x_max {
            for y in 0..y_max {

                // top left corner
                if x == 0 && y == 0{
                    p.with_color(color, |printer| {
                        printer.print((x + offset_x, y + offset_y), "┌");
                    });
                }
                //top right corner
                else if y == 0 && x == x_max-1 {
                    p.with_color(color, |printer| {
                        printer.print((x + offset_x, y + offset_y), "┐");
                    });
                }
                //bottom right corner
                else if y == y_max-1 && x == x_max-1 {
                    p.with_color(color, |printer| {
                        printer.print((x + offset_x, y + offset_y), "┘");
                    });
                }
                //bottom left corner
                else if x == 0 && y == y_max-1 {
                    p.with_color(color, |printer| {
                        printer.print((x + offset_x, y + offset_y), "└");
                    });
                }
                //top and bottom
                else if x > 0 && x < x_max-1 && (y == 0 || y == y_max-1) {
                    p.with_color(color, |printer| {
                        printer.print((x + offset_x, y + offset_y), "─");
                    });
                }
                //right and left
                else if y > 0 && y < y_max-1 && (x == x_max-1 || x == 0){
                    p.with_color(color, |printer| {
                        printer.print((x + offset_x, y + offset_y), "│");
                    });
                }

                //draw past event totals (greyed out)
                else if x == 1 && !past{
                    if x == 1 && y == 1 && nums[0] > 0{
                        p.with_color(ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(90, 190, 90)), |printer| {
                            printer.print((x + offset_x, y + offset_y), &format!("{:>2}", nums[0]));
                        });
                    }
                    else if x == 1 && y == 2 && nums[1] > 0{
                        p.with_color(ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(190, 190, 90)), |printer| {
                            printer.print((x + offset_x, y + offset_y), &format!("{:>2}", nums[1]));
                        });
                    }
                    else if x == 1 && y == 3 && nums[2] > 0{
                        p.with_color(ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(190, 90, 90)), |printer| {
                            printer.print((x + offset_x, y + offset_y), &format!("{:>2}", nums[2]));
                        });
                    }
                }
                //draw current/future event totals
                else if x == 1 && past{
                    if x == 1 && y == 1 && nums[0] > 0{
                        p.with_color(ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(70, 70, 70)), |printer| {
                            printer.print((x + offset_x, y + offset_y), &format!("{:>2}", nums[0]));
                        });
                    }
                    else if x == 1 && y == 2 && nums[1] > 0{
                        p.with_color(ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(70, 70, 70)), |printer| {
                            printer.print((x + offset_x, y + offset_y), &format!("{:>2}", nums[1]));
                        });
                    }
                    else if x == 1 && y == 3 && nums[2] > 0{
                        p.with_color(ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(70, 70, 70)), |printer| {
                            printer.print((x + offset_x, y + offset_y), &format!("{:>2}", nums[2]));
                        });
                    }
                }
                else if x == 6 && y == 1{
                    if color == ColorStyle::secondary() {
                        p.with_color(color, |printer| {
                            printer.print((x + offset_x, y + offset_y), &day);
                        });
                    }
                    else if color == ColorStyle::highlight_inactive() {
                        p.with_color(ColorStyle::secondary(), |printer| {
                            printer.print((x + offset_x, y + offset_y), &day);
                        });
                    }
                    else {
                        p.with_color(ColorStyle::primary(), |printer| {
                            printer.print((x + offset_x, y + offset_y), &day);
                        });
                    }
                }
            }
        }
    }

    fn get_cell(&mut self, mouse_pos: Vec2, offset: Vec2) -> Option<Vec2> {

        //size 78 by 36

        //offset is top left corner of view (in this case the plane)

        //mouse pos is the position of the mouse

        //11 by 6 is the size of one cell

        let diff = mouse_pos.map(|v| v as i64) - offset.map(|v| v as i64);
        let pos : cursive::XY<i32> = ((diff.x/11) as i32, (diff.y/6) as i32).into();
        Some((pos.x, pos.y).into())
        
    }

}

impl<T: TimeZone + Send + Sync + 'static> View for CalendarView<T>
where
    T::Offset: Send + Sync,
{

    fn draw(&self, printer: &Printer) {
        //if self.changed {
            self.draw_days(printer);
        //}
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        //(11, 5).into()
        self.size = (78, 36).into();
        (78, 36).into()
    }

    fn on_event(&mut self, event: Event) -> EventResult {

        if !self.enabled {
            return EventResult::Ignored;
        }

        let last_view_date = self.view_date.clone();

        let viewdate_xy = date_to_cell(&last_view_date);

        match event {
            Event::Mouse {
                offset,
                position,
                event: MouseEvent::Press(_btn),
            } => {
                // Get cell for position
                if let Some(pos) = self.get_cell(position, offset) {
                    self.focused = Some(pos);
                    return EventResult::Consumed(None);
                }
                
            }
            Event::Mouse {
                offset,
                position,
                event: MouseEvent::Release(btn),
            } => {
                // Get cell for position
                if let Some(pos) = self.get_cell(position, offset) {
                    if self.focused == Some(pos) {
                        // We got a click here!
                        match btn {
                            MouseButton::Left => { 
                                if let Some(date) = date_from_cell_offset(
                                    &last_view_date, None,
                                    pos.x as i32 - viewdate_xy.x,
                                    pos.y as i32 - viewdate_xy.y,
                                    0, 0) {
                                    self.set_view_date(date);
                                }
                            }
                            MouseButton::Right => {
                                if let Some(date) = date_from_cell_offset(
                                    &last_view_date, None,
                                    pos.x as i32 - viewdate_xy.x,
                                    pos.y as i32 - viewdate_xy.y,
                                    0, 0) {
                                    self.set_view_date(date);
                                }   

                                let mut storage_ref_mut = self.storage.lock().unwrap();

                                let events: Vec<TextEvent> = storage_ref_mut.events.entry(
                                    NaiveDate::from_ymd_opt(self.view_date.year(), self.view_date.month(), self.view_date.day()).unwrap()).or_default().clone();

                                return EventResult::Consumed(Some(Callback::from_fn(move |s: &mut Cursive| {

                                    let mut list: TodoList = TodoList::new();
                                    list.sync_events(events.clone());

                                    create_todo_list::<T>(list, s);
                                    
                                })));
                            }
                            _ => (),
                        }
                    }
                    self.focused = None;
                }
            }
            _ => (),
        }

        let offsets = match event {
            Event::Key(Key::Up) => Some((0, -1, 0, 0)
            ),
            Event::Key(Key::Down) => Some((0, 1, 0, 0)
            ),
            Event::Key(Key::Right) => Some((1, 0, 0, 0)
            ),
            Event::Key(Key::Left) => Some((-1, 0, 0, 0)
            ),
            Event::Key(Key::Enter) => {

                let mut storage_ref_mut = self.storage.lock().unwrap();

                let events = storage_ref_mut.events.entry(
                    NaiveDate::from_ymd_opt(
                        self.view_date.year(), self.view_date.month(),
                        self.view_date.day()).unwrap()).or_default().clone();

                return EventResult::Consumed(Some(Callback::from_fn(move |s| {

                    let mut list: TodoList = TodoList::new();
                    list.sync_events(events.clone());

                    create_todo_list::<T>(list, s);
                    
                })));

            }
            _ => None,
        };

        if let Some((x, y, month, year)) = offsets {
            if let Some(date) = date_from_cell_offset(&last_view_date, None, x, y, month, year) {
                self.current_date = date_to_cell(&date);
                self.set_view_date(date);
            }
        }

        if self.view_date != last_view_date {

            let year_string = self.view_date.year().to_string();
            let month_string = month_to_string(self.view_date.month() as i32);

            EventResult::Consumed(Some(Callback::from_fn(move |s| {
                s.call_on_name("view1", |view: &mut NamedView<Panel<LinearLayout>>| {
                    view.get_mut().set_title(year_string.clone());
                });
                s.call_on_name("view2", |view: &mut NamedView<Panel<LinearLayout>>| {
                    view.get_mut().set_title(month_string.clone());
                });
            })))
        } else {
            EventResult::Ignored
        }
    }
    
    fn take_focus(&mut self, _: Direction) -> Result<EventResult, CannotFocus> {
        self.enabled.then(EventResult::consumed).ok_or(CannotFocus)
    }
    
}

pub fn create_event_editor<T: TimeZone + Send + Sync + 'static>(content : String, i : usize) -> ResizedView<EditView>
where
    T: TimeZone + Send + Sync + 'static,
    CalendarView<T>: View,
{
    EditView::new()
        .on_edit(move |s, text, _cursor| {
            s.call_on_name("calendar", |view: &mut CalendarView<T>| {

                let mut storage_ref_mut = view.storage.lock().unwrap();

                storage_ref_mut.events.entry(NaiveDate::from_ymd_opt(view.view_date.year(), view.view_date.month(), view.view_date.day()).unwrap())
                    .or_insert(Vec::new())[i].content = String::from(text);
            });
        })
        .content(content)
        .fixed_width(25)
}

// creates the todo list popup
pub fn create_todo_list<T: TimeZone + Send + Sync + 'static>(todo_list: TodoList, s: &mut Cursive) 
where
    T: TimeZone + Send + Sync + 'static,
    CalendarView<T>: View,
    {

    let events = todo_list.get_events();

    s.add_layer(Dialog::around(LinearLayout::horizontal()
        .child(NamedView::new("list", todo_list))
        .child(
                LinearLayout::vertical().child(NamedView::new("todo", ListView::new()
                    // Each child is a single-line view with a label
                    // This part loads any existing events from the storage
                    .delimiter()
                    .with(|list| {

                        // list.add_child("", DummyView);

                        // generate the children with a for loop
                        for (i, value) in events.iter().enumerate() {
                            list.add_child("",
                                create_event_editor::<T>(value.content.clone(), i),
                            );
                        }
                    }))
                    .scrollable())
                    .child(Button::new_raw("<+>", |s| {

                        let mut events: Vec<TextEvent> = Vec::new();

                        // creates the entries in the calendar cells + storage
                        s.call_on_name("calendar", |view: &mut CalendarView<T>| {

                            let mut storage_ref_mut = view.storage.lock().unwrap();
                            
                            // create a new event in the storage if there is none at the hash map date specified
                            storage_ref_mut.events.entry(
                                NaiveDate::from_ymd_opt(view.view_date.year(), view.view_date.month(), view.view_date.day()).unwrap())
                                .or_insert(Vec::new()).push(TextEvent::new(String::from("")));

                            events = storage_ref_mut.events.entry(
                                NaiveDate::from_ymd_opt(view.view_date.year(), view.view_date.month(), view.view_date.day()).unwrap()).or_default().clone();

                        });

                        // creates the entries in the todo popup
                        s.call_on_name("todo", |view: &mut NamedView<ListView>| {
                            let mut view = view.get_mut();
                            let len = view.len()-1;

                            view.add_child(
                                "",// &format!("Event {}:", len),
                                create_event_editor::<T>(String::from(""), len),
                            );
                        });

                        // adds the newly created event to the todolist
                        s.call_on_name("list", |view: &mut NamedView<TodoList>| {
                            let mut view = view.get_mut();
                            // view.add_status(0);
                            view.sync_events(events.clone());
                        });
                        
                    }))//.max_height(15)
        ))
        .title("Todo")
        
        //TODO save content here
        .button("Ok", |s| {
            s.pop_layer();
        }));

}

pub struct TaskList {
    enabled: bool,
    size: Vec2,
    focused: Option<Vec2>,
    events_list: Vec<TextEvent>,
}

impl TaskList {

    // Initializes the struct
    pub fn new() -> Self {
        Self {
            enabled: true,
            size: (0, 0).into(),
            focused: None,
            events_list: Vec::new(),
        }
    }

    pub fn sync_events(&mut self, events: Vec<TextEvent>) {
        self.events_list = events.clone();
    }

    pub fn get_events(&self) -> Vec<TextEvent> {
        self.events_list.clone()
    }

    fn get_cell(&mut self, mouse_pos: Vec2, offset: Vec2) -> Option<Vec2> {
        let diff = mouse_pos.map(|v| v as i64) - offset.map(|v| v as i64);
        let pos : cursive::XY<i32> = ((diff.x/1) as i32, (diff.y/1) as i32).into();

        Some((pos.x, pos.y).into())   
    }

    fn draw_list(&self, p: &Printer) {
        if self.events_list.len() > 0{
            for y in 1..self.events_list.len()+1 {
                for x in 0..self.size.x {
                    if x == 0 {
                        if self.focused != None && self.focused.unwrap().x == 0 && self.focused.unwrap().y == y{
                            p.with_color(ColorStyle::highlight(), |printer| {
                                printer.print((x, y), "X");
                            });
                        }
                        else{
                            p.with_color(ColorStyle::primary(), |printer| {
                                printer.print((x, y), "x");
                            });
                        }
                    }
                    if x == 2 || x == 3{
                        if self.events_list[y-1].status == 1{
                            p.with_color(ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(190, 190, 90)), |printer| {
                                printer.print((x, y), " ");
                            });
                        }
                        else if self.events_list[y-1].status == 2{
                            p.with_color(ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(190, 90, 90)), |printer| {
                                printer.print((x, y), " ");
                            });
                        }
                        else{
                            p.with_color(ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(90, 190, 90)), |printer| {
                                printer.print((x, y), " ");
                            });
                        }
                    }
                }
            }   
        }
    }
}

impl View for TaskList {

    fn draw(&self, printer: &Printer) {
        self.draw_list(printer);
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        self.size = (5, 10).into();
        (5, 10).into()
    }

    fn take_focus(&mut self, _: Direction) -> Result<EventResult, CannotFocus> {
        self.enabled.then(EventResult::consumed).ok_or(CannotFocus)
    }

    fn on_event(&mut self, event: Event) -> EventResult {

        if !self.enabled {
            return EventResult::Ignored;
        }

        match event {
            Event::Mouse {
                offset,
                position,
                event: MouseEvent::Press(_btn),
            } => {
                // Get cell for position
                if let Some(pos) = self.get_cell(position, offset) {
                    self.focused = Some(pos);
                    return EventResult::Consumed(None);
                }
            }
            Event::Mouse {
                offset,
                position,
                event: MouseEvent::Release(btn),
            } => {
                // Get cell for position
                if let Some(pos) = self.get_cell(position, offset) {
                    if self.focused == Some(pos) {
                        // We got a click here!
                        match btn {
                            MouseButton::Left => { 
                                if (pos.x > 0 && pos.x < 4) && pos.y > 0 && self.events_list.len() > 0 && pos.y <= self.events_list.len(){

                                    // completed logic

                                }
                            }
                            _ => (),
                        }
                    }
                    self.focused = None;
                }
            }
            _ => (),
        }

        return EventResult::Ignored;
    }

}

pub struct TodoList{
    enabled: bool,
    size: Vec2,
    focused: Option<Vec2>,
    events_list: Vec<TextEvent>,
}

impl TodoList {

    // Initializes the struct
    pub fn new() -> Self {
        Self {
            enabled: true,
            size: (0, 0).into(),
            focused: None,
            events_list: Vec::new(),
        }
    }

    pub fn sync_events(&mut self, events: Vec<TextEvent>) {
        self.events_list = events.clone();
    }

    pub fn get_events(&self) -> Vec<TextEvent> {
        self.events_list.clone()
    }

    fn get_cell(&mut self, mouse_pos: Vec2, offset: Vec2) -> Option<Vec2> {
        let diff = mouse_pos.map(|v| v as i64) - offset.map(|v| v as i64);
        let pos : cursive::XY<i32> = ((diff.x/1) as i32, (diff.y/1) as i32).into();

        Some((pos.x, pos.y).into())
    }

    fn draw_list(&self, p: &Printer) {
        if self.events_list.len() > 0{
            for y in 1..self.events_list.len()+1 {
                for x in 0..self.size.x {
                    if x == 0 {
                        if self.focused != None && self.focused.unwrap().x == 0 && self.focused.unwrap().y == y{
                            p.with_color(ColorStyle::highlight(), |printer| {
                                printer.print((x, y), "X");
                            });
                        }
                        else{
                            p.with_color(ColorStyle::primary(), |printer| {
                                printer.print((x, y), "x");
                            });
                        }
                    }
                    if x == 2 || x == 3{
                        if self.events_list[y-1].status == 1{
                            p.with_color(ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(190, 190, 90)), |printer| {
                                printer.print((x, y), " ");
                            });
                        }
                        else if self.events_list[y-1].status == 2{
                            p.with_color(ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(190, 90, 90)), |printer| {
                                printer.print((x, y), " ");
                            });
                        }
                        else{
                            p.with_color(ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(90, 190, 90)), |printer| {
                                printer.print((x, y), " ");
                            });
                        }
                    }
                }
            }   
        }
    }
}

impl View for TodoList {

    fn draw(&self, printer: &Printer) {
        self.draw_list(printer);
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        self.size = (5, 10).into();
        (5, 10).into()
    }

    fn take_focus(&mut self, _: Direction) -> Result<EventResult, CannotFocus> {
        self.enabled.then(EventResult::consumed).ok_or(CannotFocus)
    }

    fn on_event(&mut self, event: Event) -> EventResult {

        if !self.enabled {
            return EventResult::Ignored;
        }

        match event {
            Event::Mouse {
                offset,
                position,
                event: MouseEvent::Press(_btn),
            } => {
                // Get cell for position
                if let Some(pos) = self.get_cell(position, offset) {
                    self.focused = Some(pos);
                    return EventResult::Consumed(None);
                }
            }
            Event::Mouse {
                offset,
                position,
                event: MouseEvent::Release(btn),
            } => {
                // Get cell for position
                if let Some(pos) = self.get_cell(position, offset) {
                    if self.focused == Some(pos) {
                        // We got a click here!
                        match btn {
                            MouseButton::Left => { 
                                if (pos.x > 0 && pos.x < 4) && pos.y > 0 && self.events_list.len() > 0 && pos.y <= self.events_list.len(){
                                    if self.events_list[pos.y-1].status < 2 {
                                        self.events_list[pos.y-1].status += 1;
                                    }
                                    else {
                                        self.events_list[pos.y-1].status = 0;
                                    }

                                    let status = self.events_list[pos.y-1].status;

                                    return EventResult::Consumed(Some(Callback::from_fn(move |s| {

                                        s.call_on_name("calendar", |view: &mut CalendarView<Utc>| {

                                            let mut storage_ref_mut = view.storage.lock().unwrap();

                                            storage_ref_mut.events.entry(
                                                NaiveDate::from_ymd_opt(view.view_date.year(), view.view_date.month(), view.view_date.day()).unwrap())
                                                .or_insert(Vec::new())[pos.y-1].status = status;
                                        });

                                    })));
                                }
                                else if pos.x == 0 && pos.y > 0 && self.events_list.len() > 0 && pos.y <= self.events_list.len(){
                                    self.events_list.remove(pos.y-1);

                                    return EventResult::Consumed(Some(Callback::from_fn(move |s| {
                                        s.call_on_name("calendar", |view: &mut CalendarView<Utc>| {

                                            let mut storage_ref_mut = view.storage.lock().unwrap();

                                            storage_ref_mut.events.entry(NaiveDate::from_ymd_opt(view.view_date.year(),
                                                view.view_date.month(), view.view_date.day()).unwrap())
                                                .or_default().remove(pos.y-1);

                                        });

                                        s.call_on_name("todo", |view: &mut NamedView<ListView>| {
                                            let mut view = view.get_mut();
                                            view.remove_child(pos.y);
                                        });
                                        
                                    })));
                                }
                            }
                            _ => (),
                        }
                    }
                    self.focused = None;
                }
            }
            _ => (),
        }
        return EventResult::Ignored;
    }
}

// creates the Vec of "coming up" tasks taken from the todo list
fn create_task_list(s: Arc<Mutex<Storage>>) -> Vec<Vec<TextEvent>> {

    // A list of all events in the next 7 days
    let mut week_events: Vec<Vec<TextEvent>> = Vec::new();

    let mut storage_ref_mut = s.lock().unwrap();

    // get current date
    let utc: DateTime<Local> = Local::now();

    // need to go through the next 7 days where the first day is the present
    for i in 0..7 {
        week_events.push(storage_ref_mut.events.entry(
            NaiveDate::from_ymd_opt(utc.year(), utc.month(), utc.day() + i).unwrap()).or_default().clone());
    }

    // start creating the list
    // sort the tasks by days, then priorities, then alphabetical
    for events in &mut week_events {
        events.sort_by(|a, b| {

            // First, compare by status in descending order
            let status_cmp = b.status.cmp(&a.status);
            
            // If status is equal, compare alphabetically by content
            if status_cmp == std::cmp::Ordering::Equal {
                a.content.cmp(&b.content)
            } else {
                status_cmp
            }
        });
    }

    return week_events

}




// creates the main panel which includes the entire calendar, clock and other components
fn create_panel(year : i32, month : u32, st : Arc<Mutex<Storage>>) -> Panel<LinearLayout>{

    //TODO MOVE OUT OF FUNCTION
    let utc: DateTime<Local> = Local::now();
    let c_year = utc.year();
    let c_month = utc.month();
    let c_day = utc.day();

    // TODO: must be a better way of organizing/doing this
    let st_clone_panel = Arc::clone(&st);
    let st_clone_panel2 = Arc::clone(&st);
    let st_clone_panel3 = Arc::clone(&st);
    let st_clone_panel4 = Arc::clone(&st);
    let st_clone_panel5 = Arc::clone(&st);

    // create the entire calendar display
    Panel::new(
        LinearLayout::vertical()
        .child(LinearLayout::horizontal()
            // .child(Panel::new(Layer::new(TextView::new(StyledString::styled(BADGE, Color::Dark(BaseColor::Blue))).center().fixed_width(21))
            // .child(Panel::new(Layer::new(TextView::new(" ").center().fixed_width(21))))
            
            .child(Panel::new(PaddedView::lrtb( 2, 2, 2, 2, LinearLayout::vertical()
                .child(
                Panel::new(Button::new(format!("{}/{}/{}", c_month, c_day, c_year), move|s| {
                        
                    s.pop_layer();
    
                    let st_clone = Arc::clone(&st_clone_panel4);
    
                    s.add_layer(create_panel(c_year, c_month, Arc::clone(&st_clone)));
    
                }))
                )
            )).min_width(23))

            .child(NamedView::new("clock", 
            Panel::new(PaddedView::lrtb(3, 0, 0, 0, Clock::new())).max_width(80)
            ))
        )
        .child(
            LinearLayout::horizontal()
            // create the month selector
            .child(NamedView::new("view1", 
                Panel::new(LinearLayout::vertical()
                    .with(|column| {

                        // TODO move this outside as a global setting
                        let num_rows = 3;
                        let mut month = 1;

                        // generate each row 
                        for _rows in 0..(12/num_rows) {

                            column.add_child(LinearLayout::horizontal()
                                .with(|row| {

                                    // generate the children with a for loop
                                    for _columns in 0..num_rows {

                                        let st_clone = Arc::clone(&st);
                                        
                                        row.add_child(Panel::new(Button::new(abbr_month_to_string(month), move |s| {
                                            s.pop_layer();
                                            s.add_layer(create_panel(year, month as u32, Arc::clone(&st_clone)));
                                        })));

                                        month += 1;
                                    }
                                })
                            )

                        }
                    })
                    // spacer
                    .child(Layer::new(TextView::new(" ")))

                    // Task list
                    .child(
                    Panel::new(
                            // TODO:
                            // move this into its own custom class with a draw call and event handling
                            // in order to update the task list as tasks are created
                            // also in order to add the custom task color printing and completion toggle
                        ListView::new()
                            .delimiter()
                            .with(|list| {

                                let global_tasks = create_task_list(st_clone_panel5);

                                let utc: DateTime<Local> = Local::now();

                                let mut id_counter = 0;

                                for (day, tasks) in global_tasks.iter().enumerate() { 
                                    // info!("sorting: {:?}", tasks);

                                    if !tasks.is_empty() {
                                        let task_date = utc + Duration::days(day as i64);

                                        if day != 0 {
                                            list.add_child("", DummyView);
                                        }

                                        // seperate by day
                                        list.add_child("",
                                            TextView::new(StyledString::styled(
                                            format!("{}", format_task_date(task_date)), 
                                                    Style::secondary().combine(Effect::Bold))));

                                        list.add_child("", DummyView);

                                        let mut task_counter = 0;

                                        for task in tasks {

                                            //TODO: when clicking on task toggle it as completed

                                            let task_name: &str = &task.content.clone().to_owned();

                                            list.add_child(
                                                "",
                                                LinearLayout::horizontal()
                                                        .child(TextView::new(StyledString::styled(
                                                            format!("{}. ", task_counter+1), Style::secondary().combine(Effect::Bold))))
                                                        .child(TextView::new(StyledString::styled(
                                                            format!("{}",task_name), Style::primary().combine(Effect::Simple))).with_name(format!("{id_counter}")))
                                                        .child(Checkbox::new().on_change(move |s, checked| {
                                                            // Enable/Disable the next field depending on this checkbox
                                                            if checked {
                                                                s.call_on_name(&format!("{id_counter}"), |view: &mut TextView| {
                                                                    view.set_style(Style::secondary().combine(Effect::Strikethrough));
                                                                });
                                                            }
                                                            else {
                                                                s.call_on_name(&format!("{id_counter}"), |view: &mut TextView| {
                                                                    view.set_style(Style::secondary().combine(Effect::Simple));
                                                                });
                                                            }
                                                        }))
                                            );

                                            task_counter += 1;
                                            id_counter += 1;
                                        }
                                    }
                                }
                            })
                            .scrollable()
                                
                        ).title("Next 7 Days").max_height(25)
                    )

                    // spacer
                    // .child(Layer::new(TextView::new(" ")))

                    .child(Panel::new(Button::new("Pomodoro Timer", move|s| {
                        
                        
        
                    })))

                    ).title(year.to_string()).max_width(23)
                )
            )
            .child(
                    // calendar with days
                NamedView::new("view2", Panel::new(create_calendar(year, month, Arc::clone(&st_clone_panel))
                    .child(Layer::new(
                        LinearLayout::horizontal()
                            .child(
                                PaddedView::lrtb(36, 0, 0, 0, LinearLayout::vertical()
                                    .child(
                                        Button::new("Up", move |s| {

                                            s.pop_layer();

                                            let st_clone = Arc::clone(&st_clone_panel2);

                                            if month-1 == 0{
                                                s.add_layer(create_panel(year-1, 12, Arc::clone(&st_clone)));
                                            } else {
                                                s.add_layer(create_panel(year, month-1, Arc::clone(&st_clone)));
                                            }
                                        })
                                    )
                                    .child(
                                        Button::new("Down", move |s| {

                                            s.pop_layer();

                                            let st_clone = Arc::clone(&st_clone_panel3);

                                            if month+1 == 13{
                                                s.add_layer(create_panel(year+1, 1, Arc::clone(&st_clone)));
                                            } else {
                                                s.add_layer(create_panel(year, month+1, Arc::clone(&st_clone)));
                                            }
                                        })
                                    )
                                )
                            )
                        )
                    )
                )
                .title(month_to_string(month as i32)))
            )
        )
    )
}

/// Moves top layer by the specified amount
fn move_top(c: &mut Cursive, x_in: isize, y_in: isize) {
    // Step 1. Get the current position of the layer.
    let s = c.screen_mut();
    let l = LayerPosition::FromFront(0);

    // Step 2. add the specifed amount
    // let pos = s.offset().saturating_add((x_in, y_in));
    let pos = s.layer_offset(LayerPosition::FromFront(0)).unwrap().saturating_add((x_in, y_in));

    // convert the new x and y into a position
    let p = Position::absolute(pos);

    // Step 3. Apply the new position
    s.reposition_layer(l, p);
}

fn date_to_cell<T: TimeZone>(date: &DateTime<T>) -> cursive::XY<i32>{
    let cd = Utc.with_ymd_and_hms(date.year(), date.month(), 1, 0, 0, 0).unwrap();
    let day_of_week = cd.weekday().number_from_sunday();

    let num = date.day0() + (day_of_week-1);
    let row = num/7;
    let column = num - (row*7);

    return (column as i32, row as i32).into()
}

// calculates the date by using the y and x coordinates of a calendar date
// helper function for converting mouse position to a date selection
fn date_from_cell_offset<T: TimeZone>(
    date: &DateTime<T>,
    set_day: Option<i32>,
    x_offset: i32,
    y_offset: i32,
    month_offset: i32,
    year_offset: i32,
) -> Option<DateTime<T>> {
    let mut year = date.year() + year_offset;
    let mut month = date.month0() as i32;
    month += month_offset;

    // let num_days = ndays_in_month(date.year(), date.month());

    //let mut current_cell = date_to_cell(date);
    //current_cell = (current_cell.x + x_offset, current_cell.y + y_offset).into();

    let offset = y_offset*7 + x_offset;

    while month < 0 {
        year -= 1;
        month += 12;
    }

    while month >= 12 {
        month -= 12;
        year += 1;
    }

    let d = date
        .with_day0(0)?
        .with_year(year)?
        .with_month0(month as u32)?;

    let month: util::month::Month = d.month0().into();
    let number_of_days = month.number_of_days(year);

    let mut day = set_day.unwrap_or_else(|| cmp::min(number_of_days - 1, date.day0() as i32));

    day += offset as i32;

    if day < 0 {
        day += month.prev_number_of_days(year);
        date_from_cell_offset(&d, Some(day), 0, 0, -1, 0)
    } else if day >= number_of_days {
        day -= number_of_days;
        date_from_cell_offset(&d, Some(day), 0, 0, 1, 0)
    } else {
        d.with_day0(day as u32)
    }

}

// placeholder method for future theme managment
fn custom_theme_from_cursive(siv: &Cursive) -> Theme {
    // We'll return the current theme with a small modification.
    let theme = siv.current_theme().clone();

    theme
}

// Writes storage hash map to json
// responsible for saving user data
// TODO: allow for save file naming and directory selection
fn write(s: &HashMap<NaiveDate, Vec<TextEvent>>) {
    let j = serde_json::to_string(&s).expect("could not serialize hashmap");
    fs::write("test.json", j).expect("Unable to write file");
}

// Converts json save file back into a hashmap
// that is used to load previous data
fn read() -> HashMap<NaiveDate, Vec<TextEvent>>{
    if let Ok(mut f) = File::open("test.json") {
        let mut contents = String::new();
        if f.read_to_string(&mut contents).is_err(){
            return HashMap::new();
        }
        if let Ok(p) = serde_json::from_str::<HashMap<NaiveDate, Vec<TextEvent>>>(&contents){
            return p;
        }
        else{
            return HashMap::new();
        }
    } else {
        HashMap::new()
    }
}

/// Outputs a custom date fromat given a DateTime
/// 8/9/2024 becomes: Friday, 9th
fn format_task_date(date: DateTime<Local>) -> String {
    // Get the weekday and day of the month
    let weekday = match date.weekday() {
        Weekday::Mon => "Monday",
        Weekday::Tue => "Tuesday",
        Weekday::Wed => "Wednesday",
        Weekday::Thu => "Thursday",
        Weekday::Fri => "Friday",
        Weekday::Sat => "Saturday",
        Weekday::Sun => "Sunday",
    };
    
    let day_of_month = date.day();

    // Convert the day of the month to a string with a suffix (e.g., "13th")
    let day_suffix = match day_of_month {
        1 | 21 | 31 => "st",
        2 | 22 => "nd",
        3 | 23 => "rd",
        _ => "th",
    };

    // Format the date as "Friday, 13th"
    format!(
        "{}, {}{}",
        weekday,
        day_of_month,
        day_suffix
    )
}

fn main() {
    let utc: DateTime<Local> = Local::now();
    let year = utc.year();
    let month = utc.month();

    // Create a log file
    let log_file = File::create("app.log").unwrap();

    // Initialize the logger to write to the file
    WriteLogger::init(LevelFilter::Info, Config::default(), log_file).unwrap();

    // let data = Rc::new(RefCell::new(Storage::new(read())));
    let data = Arc::new(Mutex::new(Storage::new(read())));

    let mut siv = cursive::default();

    siv.add_global_callback('w', |s| move_top(s, 0, -1));
    siv.add_global_callback('a', |s| move_top(s, -1, 0));
    siv.add_global_callback('s', |s| move_top(s, 0, 1));
    siv.add_global_callback('d', |s| move_top(s, 1, 0));

    // save button 'k'
    siv.add_global_callback('k', |s| {
        s.call_on_name("calendar", |view: &mut CalendarView<Utc>| {

            let mut_storage_ref = view.storage.lock().unwrap();

            write(&mut_storage_ref.events);

        });
    });

    let theme = custom_theme_from_cursive(&siv);
    siv.set_theme(theme);

    siv.add_layer(create_panel(year, month, data));

    siv.set_autorefresh(true);

    siv.run();
}