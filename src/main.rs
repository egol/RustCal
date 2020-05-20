// use chrono::prelude::*;

// let numrows = num_days/7;


//TODO
//https://docs.rs/cursive/0.14.1/cursive/view/trait.View.html
//implement the needs re-layout function to improve performance


// Crate Dependencies ---------------------------------------------------------
extern crate chrono;
extern crate cursive;
extern crate cursive_calendar_view;

// STD Dependencies -----------------------------------------------------------
use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use std::cmp;
use std::collections::HashMap;
use std::{thread, time};
use std::fs::File;
use std::io::prelude::*;
use std::fs;

// External Dependencies ------------------------------------------------------
use chrono::prelude::*;
use cursive::traits::*;
use cursive::Cursive;
use cursive::Vec2;
use cursive::{Printer};
#[macro_use] extern crate serde_derive;
use serde_json::json;
// extern crate time;

use cursive::theme::*;
// use cursive::views::{Button, LinearLayout, TextView, PaddedView, Dialog, BoxedView, EditView, ResizedView, Panel, ListView, Layer, DummyView};
use cursive::views::*;
use cursive::traits::Boxable;
use cursive::view::Position;
use cursive::views::NamedView;
use cursive::views::LayerPosition;
use cursive::event::{Callback, Event, EventResult, Key, MouseButton, MouseEvent};

use cursive::direction::Direction;

mod util;

use std::time::Duration;
 
// const TOP: &str = " ⡎⢉⢵ ⠀⢺⠀ ⠊⠉⡱ ⠊⣉⡱ ⢀⠔⡇ ⣏⣉⡉ ⣎⣉⡁ ⠊⢉⠝ ⢎⣉⡱ ⡎⠉⢱ ⠀⠶⠀";
// const BOT: &str = " ⢗⣁⡸ ⢀⣸⣀ ⣔⣉⣀ ⢄⣀⡸ ⠉⠉⡏ ⢄⣀⡸ ⢇⣀⡸ ⢰⠁⠀ ⢇⣀⡸ ⢈⣉⡹ ⠀⠶⠀";


const TOP: &str="   .oooo.  #   .o #   .oooo.  #   .oooo. #      .o  #  oooooooo#   .ooo   # ooooooooo# .ooooo.  #  .ooooo. #    ";
const M1: &str ="  d8P'`Y8b # o888 # .dP\"\"Y88b #.dP\"\"Y88b #    .d88  # dP\"\"\"\"\"\"\"#  .88'    #d\"\"\"\"\"\"\"8'#d88'   `8.#888' `Y88.#    ";
const M2: &str =r##" 888    888#  888 #       |8P'#      |8P'#  .d'888  #d88888b.  # d88'     #      .8' #Y88..  .8'#888    888#    "##  ;
const M3: &str =" 888    888#  888 #     .d8P' #    <88b. #.d'  888  #    `Y88b #d888P\"Ybo.#     .8'  # `88888b. # `Vbood888#    " ;
const M4: &str =" 888    888#  888 #   .dP'    #     `88b.#88ooo888oo#      |88 #Y88|   |88#    .8'   #.8'  ``88b#      888'#    ";
const M5: &str =" `88b  d88'#  888 # .oP     .o#o.   .88P #     888  #o.   .88P #`Y88   88P#   .8'    #`8.   .88P#    .88P' #    ";
const BOT: &str="  `Y8bd8P' # o888o# 8888888888#`8bd88P'  #    o888o #`8bd88P'  # `88bod8' #  .8'     # `boood8' #  .oP'    #    ";
                                                                                                          
                                                                                                             
                                                                                                             
                                                                                                             
                                                                         
 
// fn main() {
//     let top: Vec<&str> = TOP.split_whitespace().collect();
//     let bot: Vec<&str> = BOT.split_whitespace().collect();
 
//     loop {
//         let tm = &time::now().rfc822().to_string()[17..25];
//         let top_str: String = tm.chars().map(|x| top[x as usize - '0' as usize]).collect();
//         let bot_str: String = tm.chars().map(|x| bot[x as usize - '0' as usize]).collect();
 
//         clear_screen();
//         println!("{}", top_str);
//         println!("{}", bot_str);
 
//         thread::sleep(Duration::from_secs(1));
//     }
// }

fn ndays_in_month(year: i32, month: u32) -> u32 {
    // the first day of the next month...
    let (y, m) = if month == 12 { (year + 1, 1) } else { (year, month + 1) };
    let d = NaiveDate::from_ymd(y, m, 1);

    // ...is preceded by the last day of the original month
    d.pred().day()
}

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

fn num_to_string(num : i32) -> String{
    match num {
        1 => String::from("
         .o  \n
        o888 \n
         888 \n  
         888 \n
         888 \n
         888 \n
        o888o\n
        "),
        2 => String::from("
          .oooo.   \n
        .dP\"\"Y88b  \n
              ]8P' \n
            .d8P'  \n
          .dP'     \n
        .oP     .o \n
        8888888888 \n
        "),
        3 => String::from("
          .oooo.   \n
        .dP\"\"Y88b  \n
              ]8P' \n
            <88b.  \n
             `88b. \n
        o.   .88P  \n
        `8bd88P'   \n
        "),
        4 => String::from("
          .o   \n
        .d88   \n
      .d'888   \n
    .d'  888   \n
    88ooo888oo \n
         888   \n
        o888o  \n
        "),
        5 => String::from("
         oooooooo \n
        dP\"\"\"\"\"\"\" \n
       d88888b.  \n
           `Y88b \n
             ]88 \n
       o.   .88P \n
       `8bd88P'  \n
        "),
        6 => String::from("
          .ooo    \n
        .88'      \n
       d88'       \n
      d888P\"Ybo. \n
      Y88[   ]88  \n
      `Y88   88P  \n
       `88bod8'   \n
        "),
        7 => String::from("
        ooooooooo  \n
        d\"\"\"\"\"\"\"8' \n
              .8'  \n
             .8'   \n
            .8'    \n
           .8'     \n
          .8'      \n      
        "),
        8 => String::from("
         .ooooo.   \n
        d88'   `8. \n
        Y88..  .8' \n
         `88888b.  \n
        .8'  ``88b \n
        `8.   .88P \n
         `boood8'  \n      
        "),
        9 => String::from("
         .ooooo.   \n
        888' `Y88. \n
        888    888 \n
         `Vbood888 \n
              888' \n
            .88P'  \n
          .oP'     \n      
        "),
        0 => String::from("
         .oooo.   \n
        d8P'`Y8b  \n
       888    888 \n
       888    888 \n
       888    888 \n
       `88b  d88' \n
        `Y8bd8P'  \n     
        "),
        _ => String::from("Invalid number"),
    }
}


#[derive(Clone, Serialize, Deserialize)]
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

// <T: TimeZone>

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

    let local: DateTime<Local> = Local::now();

    // println!("{:?}", &local.to_rfc2822());

    let mut tm = local.to_rfc2822().to_string()[17..25].to_owned();
    let mut temp = 0;
    for (i, a) in tm.chars().enumerate(){

        if i == 0 && a as i32 > 0{
            temp = 1;
        }
        if i == 1 && temp == 1 && a as i32 > 1{
            temp = 2;
        }

    }

    if temp == 2 {
        let num = tm[0..2].parse::<i32>().unwrap() - 12;
        tm = format!("{}{}", num, &tm[2..8]);
    }

    let top_str: String = tm.chars().map(|x| top[x as usize - '0' as usize]).collect();

    let m1_str: String = tm.chars().map(|x| m1[x as usize - '0' as usize]).collect();
    let m2_str: String = tm.chars().map(|x| m2[x as usize - '0' as usize]).collect();
    let m3_str: String = tm.chars().map(|x| m3[x as usize - '0' as usize]).collect();
    let m4_str: String = tm.chars().map(|x| m4[x as usize - '0' as usize]).collect();
    let m5_str: String = tm.chars().map(|x| m5[x as usize - '0' as usize]).collect();

    let bot_str: String = tm.chars().map(|x| bot[x as usize - '0' as usize]).collect();

    // println!("{}\n{}\n{}\n{}\n{}\n{}\n{}", top_str, m1_str, m2_str, m3_str, m4_str, m5_str, bot_str);

    return format!("{}\n{}\n{}\n{}\n{}\n{}\n{}", top_str, m1_str, m2_str, m3_str, m4_str, m5_str, bot_str);
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
                // do something with character `c` and index `i`
                p.with_color(ColorStyle::primary(), |printer| {
                    printer.print((x%len, y), &format!("{}", c));
                });
            }

        }
    }    

    fn on_event(&mut self, event: Event) -> EventResult {

        self.update_time();

        return EventResult::Ignored
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        //(11, 5).into()
        //self.size = (78, 36).into();
        (68, 30).into()
    }
}


pub fn create_calendar(year : i32, month : u32, s : Rc<RefCell<Storage>>) -> LinearLayout{

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

    let calendar = CalendarView::<Utc>::new(Utc.ymd(year, month, 1), s);

    // let temp: &mut Vec<TextEvent> = view.events.entry(view.view_date.clone()).or_default();

    // linear_layout.add_child(calendar2.with_name("calendar"));
    linear_layout.add_child(calendar.with_name("calendar"));

    linear_layout
}


pub struct CalendarView<T: TimeZone> {
    enabled: bool,
    changed: bool,
    view_date: Date<T>,
    on_select: Option<DateCallback<T>>,
    size: Vec2,
    earliest_date: Option<Date<T>>,
    latest_date: Option<Date<T>>,
    date: Date<chrono::Local>,
    focused: Option<Vec2>,
    current_date: cursive::XY<i32>,
    storage: Rc<RefCell<Storage>>,
}

type DateCallback<T> = Rc<dyn Fn(&mut Cursive, &Date<T>)>;

impl<T: TimeZone> CalendarView<T> {
    pub fn new(prev_date: Date<T>, s : Rc<RefCell<Storage>>) -> Self {
        Self {
            enabled: true,
            changed: false,
            size: (0, 0).into(),
            date: Local::now().date(),
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
        F: Fn(&mut Cursive, &Date<T>) + 'static,
    {
        self.on_select = Some(Rc::new(move |s, date| cb(s, date)));
    }

    /// Sets a callback to be used when an a new date is visually selected.
    ///
    /// Chainable variant.
    pub fn on_select<F>(self, cb: F) -> Self
    where
        F: Fn(&mut Cursive, &Date<T>) + 'static,
    {
        self.with(|v| v.set_on_select(cb))
    }

    /// Sets the visually selected date of this view.
    pub fn set_view_date(&mut self, mut date: Date<T>) {
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

impl<T: TimeZone> CalendarView<T> {
    /// Method used to draw the cube.
    ///
    /// This takes as input the Canvas state and a printer.
    // fn draw_days(&self, p: &Printer) {
    //     for a in 0..7 {
    //         for b in 0..6{
    //             self.draw_cell(p, a, b);
    //         }
    //     }

    // }
    
    fn date_available(&self, date: &Date<T>) -> bool {
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

        let mut counter = 0;

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
                // println!("({}, {})", x, y);
                // printer.with_color(color, |printer| {
                //     printer.print((x, y), &format!("{:>2}", day_number + 1));
                // }); format!("{:>2}", day_number + 1)
                // let events = self.events.get(&exact_date).unwrap_or(&Vec::new()).clone();

                let events = self.storage.borrow_mut().events
                                .get(&NaiveDate::from_ymd(exact_date.year(), exact_date.month(), exact_date.day()))
                                .unwrap_or(&Vec::new()).clone();
                    
                let mut past = true;

                if (exact_date.day0() > active_day as u32 && d_month == 0 && d_year == 0) || d_year < 0 || (d_month < 0 && d_year <= 0){
                    past = false;
                }
                
                let mut totals = vec![0,0,0];

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
                // else if x == x_max-1 || y == y_max-1 || x == 0 || y == 0{
                //     p.with_color(color, |printer| {
                //         printer.print((x + offset_x, y + offset_y), " ");
                //     });
                // }
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
                else if x == 7 && y == 1{
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

        // mouse_pos
        //     .checked_sub(offset)
        //     .map(|pos| pos.map_x(|x| x / 11))
        //     .map(|pos| pos.map_y(|y| y / 6))
        //     .and_then(|pos| {
        //         if pos.fits_in(self.size) {
        //             Some(pos)
        //         } else {
        //             None
        //         }
        //     })852
        let diff = mouse_pos.map(|v| v as i64) - offset.map(|v| v as i64);
        let pos : cursive::XY<i32> = ((diff.x/11) as i32, (diff.y/6) as i32).into();

        // println!("prev pos {}, {}\n\n", self.prev_date.x, self.prev_date.y);
        //self.current_date = pos.clone();
        // println!("mouse pos {}, {}", mouse_pos.x, mouse_pos.y);
        // println!("offset {}, {}", offset.x, offset.y);
        // println!("diffrence {}, {}", diff.x, diff.y);
        //println!("position {}, {}\n\n", pos.x, pos.y);

        Some((pos.x, pos.y).into())
        
    }

}


impl<T: TimeZone + 'static> View for CalendarView<T> {

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

    fn take_focus(&mut self, _: Direction) -> bool {
        self.enabled
    }

    fn on_event(&mut self, event: Event) -> EventResult {

        //println!("current date pos: ({},{})", self.current_date.x, self.current_date.y);

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
                                if let Some(date) = date_from_cell_offset(&last_view_date, None, pos.x as i32 - viewdate_xy.x, pos.y as i32 - viewdate_xy.y, 0, 0) {
                                    //println!("date: {:?}", date);
                                    self.set_view_date(date);
                                }
                            }
                            MouseButton::Right => {
                                if let Some(date) = date_from_cell_offset(&last_view_date, None, pos.x as i32 - viewdate_xy.x, pos.y as i32 - viewdate_xy.y, 0, 0) {
                                    //println!("date: {:?}", date);
                                    self.set_view_date(date);
                                }   

                                let events = self.storage.borrow_mut().events.entry(NaiveDate::from_ymd(self.view_date.year(), self.view_date.month(), self.view_date.day())).or_default().clone();

                                return EventResult::Consumed(Some(Callback::from_fn(move |s| {

                                    // s.add_layer(make_event_list(events));

                                    //let t = self.events.entry(self.view_date.clone()).or_default().len().clone();

                                    let mut list = TodoList::new();
                                    for a in events.iter(){
                                        list.add_status(a.status);
                                    }

                                    s.add_layer(Dialog::around(LinearLayout::horizontal()
                                    .child(NamedView::new("list", list))
                                    .child(
                                            LinearLayout::vertical().child(NamedView::new("todo", ListView::new()
                                                // Each child is a single-line view with a label
                                                // .child("Events", EditView::new().fixed_width(20))
                                                .delimiter()
                                                .with(|list| {
                                                    // We can also add children procedurally

                                                    for (i, value) in events.iter().enumerate() {
                                                        list.add_child(
                                                            &format!("Event {}:", i),
                                                            create_event_editor::<T>(value.content.clone(), i),
                                                        );
                                                    }

                                                }))
                                                .scrollable())
                                                .child(Button::new_raw("<+>", |s| {
                                                    s.call_on_name("calendar", |view: &mut CalendarView<T>| {
                                                        // let temp: &mut Vec<TextEvent> = view.storage.borrow_mut().events.entry(view.view_date.clone()).or_insert(Vec::new());
                                                        view.storage.borrow_mut().events.entry(NaiveDate::from_ymd(view.view_date.year(), view.view_date.month(), view.view_date.day()))
                                                            .or_insert(Vec::new()).push(TextEvent::new(String::from("")));
                                                    });
                                                    s.call_on_name("todo", |view: &mut NamedView<ListView>| {
                                                        let mut view = view.get_mut();
                                                        let len = view.len()-1;

                                                        view.add_child(
                                                            &format!("Event {}:", len),
                                                            create_event_editor::<T>(String::from(""), len),
                                                        );
                                                    });

                                                    s.call_on_name("list", |view: &mut NamedView<TodoList>| {
                                                        let mut view = view.get_mut();
                                                        view.add_status(0);
                                                    });

                                                    //list.set_num_event(temp.len());
                                                    // s.add_layer();
                                                    
                                                }))
                                    ))
                                    .title("Todo")
                                    
                                    //save content here
                                    .button("Ok", |s| {
                                        s.pop_layer();
                                    }));
                                })));
                            }
                            // MouseButton::Middle => {
                            //     return self.auto_reveal(pos);
                            // }
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
            // Event::Key(Key::PageUp) => Some((0, 0, 1, 0)
            // ),
            // Event::Key(Key::PageDown) => Some((0, 0, -1, 0)
            // ),
            // Event::Mouse(Mouse::offset) => Some((0, -1, 0),
            //Event::Key(Key::Backspace) => {}
            //Event::Key(Key::Enter) => {}
            _ => None,
        };

        //let date : Date<TimeZone> = Utc.ymd(last_view_date.year(), last_view_date.month(), 1);


        if let Some((x, y, month, year)) = offsets {
            if let Some(date) = date_from_cell_offset(&last_view_date, None, x, y, month, year) {
                self.current_date = date_to_cell(&date);
                self.set_view_date(date);
            }
        }

        if self.view_date != last_view_date {
            let date = self.view_date.clone();

            // self.call_on_name("view1", |view: &mut NamedView<Panel<LinearLayout>>| {
            //     println!("here", );
            //     view.get_mut().set_title("works");
            // });

            // EventResult::Consumed(
            //     self.on_select
            //         .clone()
            //         .map(|cb| Callback::from_fn(move |s| {
            //                 cb(s, &date);
            //             }
            //         )),
            // )
            EventResult::Consumed(Some(Callback::from_fn(move |s| {
                s.call_on_name("view1", |view: &mut NamedView<Panel<LinearLayout>>| {
                    view.get_mut().set_title(date.year().to_string());
                });
                s.call_on_name("view2", |view: &mut NamedView<Panel<LinearLayout>>| {
                    view.get_mut().set_title(month_to_string(date.month() as i32));
                });
            })))
        } else {
            EventResult::Ignored
        }
    }
    
}

pub fn create_event_editor<T: TimeZone + 'static>(s : String, i : usize) -> ResizedView<EditView>{
    EditView::new()
        .on_edit(move |s, text, _cursor| {
            s.call_on_name("calendar", |view: &mut CalendarView<T>| {
                // let temp: &mut Vec<TextEvent> = view.storage.borrow_mut().events.entry(view.view_date.clone()).or_insert(Vec::new());
                view.storage.borrow_mut().events.entry(NaiveDate::from_ymd(view.view_date.year(), view.view_date.month(), view.view_date.day()))
                    .or_insert(Vec::new())[i] = TextEvent::new(String::from(text));
            });
        })
        .content(s)
        .fixed_width(25)
}

pub struct TodoList{
    enabled: bool,
    size: Vec2,
    focused: Option<Vec2>,
    importance_list: Vec<i8>,
}

impl TodoList {
    pub fn new() -> Self {
        Self {
            enabled: true,
            size: (0, 0).into(),
            focused: None,
            importance_list: Vec::new(),
        }
    }

    pub fn add_status(&mut self, status : i8){
        self.importance_list.push(status);
    }

    fn get_cell(&mut self, mouse_pos: Vec2, offset: Vec2) -> Option<Vec2> {
        let diff = mouse_pos.map(|v| v as i64) - offset.map(|v| v as i64);
        let pos : cursive::XY<i32> = ((diff.x/1) as i32, (diff.y/1) as i32).into();

        Some((pos.x, pos.y).into())
        
    }
    fn draw_list(&self, p: &Printer) {
        if self.importance_list.len() > 0{
            for y in 1..self.importance_list.len()+1 {
                for x in 0..self.size.x {
                    if x == 0 {
                        if self.focused != None && self.focused.unwrap().x == 0 && self.focused.unwrap().y == y{
                            p.with_color(ColorStyle::highlight(), |printer| {
                                printer.print((x, y), "-");
                            });
                        }
                        else{
                            p.with_color(ColorStyle::primary(), |printer| {
                                printer.print((x, y), "-");
                            });
                        }
                    }
                    if x == 2 || x == 3{
                        if self.importance_list[y-1] == 1{
                            p.with_color(ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(190, 190, 90)), |printer| {
                                printer.print((x, y), " ");
                            });
                        }
                        else if self.importance_list[y-1] == 2{
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
        // else if x == 2 && y == 1{
        //     p.with_color(ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(190, 190, 90)), |printer| {
        //         printer.print((x + offset_x, y + offset_y), "0");
        //     });
        // }
        // else if x == 3 && y == 1{
        //     p.with_color(ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(190, 90, 90)), |printer| {
        //         printer.print((x + offset_x, y + offset_y), "0");
        //     });
        // }
    }
}

impl View for TodoList {

    fn draw(&self, printer: &Printer) {
        self.draw_list(printer);
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        //(11, 5).into()
        self.size = (5, 10).into();
        (5, 10).into()
    }

    fn take_focus(&mut self, _: Direction) -> bool {
        self.enabled
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
                                if pos.x > 0 && pos.y > 0 && self.importance_list.len() > 0 && pos.y <= self.importance_list.len(){
                                    if self.importance_list[pos.y-1] < 2{
                                        self.importance_list[pos.y-1] += 1;
                                    }
                                    else{
                                        self.importance_list[pos.y-1] = 0;
                                    }

                                    let status = self.importance_list[pos.y-1];

                                    return EventResult::Consumed(Some(Callback::from_fn(move |s| {

                                        s.call_on_name("calendar", |view: &mut CalendarView<Utc>| {
                                            // let temp: &mut Vec<TextEvent> = view.storage.borrow_mut().events.entry(view.view_date.clone()).or_insert(Vec::new());
                                            // view.storage.borrow_mut().events.entry(view.view_date.clone()).or_insert(Vec::new())[pos.y-1].status = status;

                                            view.storage.borrow_mut().events.entry(
                                                NaiveDate::from_ymd(view.view_date.year(), view.view_date.month(), view.view_date.day()))
                                                .or_insert(Vec::new())[pos.y-1].status = status;
                                        });

                                    })));
                                }
                                else if pos.y > 0 && self.importance_list.len() > 0 && pos.y <= self.importance_list.len(){
                                    //println!("{} |    {}", self.importance_list.len(), pos.y-1);

                                    self.importance_list.remove(pos.y-1);

                                    return EventResult::Consumed(Some(Callback::from_fn(move |s| {
                                        s.call_on_name("calendar", |view: &mut CalendarView<Utc>| {
                                            //let temp: &mut Vec<TextEvent> = view.storage.borrow_mut().events.entry(view.view_date.clone()).or_default();
                                            //if temp.len() > 0 {
                                            view.storage.borrow_mut().events.entry(NaiveDate::from_ymd(view.view_date.year(), view.view_date.month(), view.view_date.day()))
                                                .or_default().remove(pos.y-1);
                                            //}
                                        });

                                        s.call_on_name("todo", |view: &mut NamedView<ListView>| {
                                            let mut view = view.get_mut();

                                            view.remove_child(pos.y);
                                        });
                                        
                                    })));
                                }
                            }
                            // MouseButton::Right => {
                            // }
                            // MouseButton::Middle => {
                            //     return self.auto_reveal(pos);
                            // }
                            _ => (),
                        }
                    }
                    self.focused = None;
                }
            }
            _ => (),
        }

        return EventResult::Ignored;
        //let date : Date<TimeZone> = Utc.ymd(last_view_date.year(), last_view_date.month(), 1);
    }
}

fn create_panel(year : i32, month : u32, st : Rc<RefCell<Storage>>) -> Panel<LinearLayout>{

    //TODO MOVE OUT OF FUNCTION
    let utc: DateTime<Local> = Local::now();
    let c_year = utc.year();
    let c_month = utc.month();
    let c_day = utc.day();

    let r = st.clone();
    let r1 = st.clone();
    let r2 = st.clone();
    let r3 = st.clone();
    let r4 = st.clone();
    let r5 = st.clone();
    let r6 = st.clone();
    let r7 = st.clone();
    let r8 = st.clone();
    let r9 = st.clone();
    let r10 = st.clone();
    let r11 = st.clone();
    let r12 = st.clone();
    let r13 = st.clone();
    let r14 = st.clone();
    let r15 = st.clone();
    let r16 = st.clone();
    let r17 = st.clone();

    Panel::new(LinearLayout::horizontal()
    .child(NamedView::new("view1", Panel::new(LinearLayout::vertical()
        .child(LinearLayout::horizontal()
            .child(Panel::new(Button::new("Dec", move |s| {
                s.pop_layer();
                s.add_layer(create_panel(year, 12, r.clone()));
            })))
            .child(Panel::new(Button::new("Jan", move |s| {
                s.pop_layer();
                s.add_layer(create_panel(year, 1, r1.clone()));
            })))
            .child(Panel::new(Button::new("Feb", move |s| {
                s.pop_layer();
                s.add_layer(create_panel(year, 2, r2.clone()));
            })))
        )
        .child(LinearLayout::horizontal()
            .child(Panel::new(Button::new("Mar", move |s| {
                s.pop_layer();
                s.add_layer(create_panel(year, 3, r3.clone()));
            })))
            .child(Panel::new(Button::new("Apr", move |s| {
                s.pop_layer();
                s.add_layer(create_panel(year, 4, r4.clone()));
            })))
            .child(Panel::new(Button::new("May", move |s| {
                s.pop_layer();
                s.add_layer(create_panel(year, 5, r5.clone()));
            })))
        )
        .child(LinearLayout::horizontal()
            .child(Panel::new(Button::new("Jun", move |s| {
                s.pop_layer();
                s.add_layer(create_panel(year, 6, r6.clone()));
            })))
            .child(Panel::new(Button::new("Jul", move |s| {
                s.pop_layer();
                s.add_layer(create_panel(year, 7, r7.clone()));
            })))
            .child(Panel::new(Button::new("Aug", move |s| {
                s.pop_layer();
                s.add_layer(create_panel(year, 8, r8.clone()));
            })))
        )
        .child(LinearLayout::horizontal()
            .child(Panel::new(Button::new("Sep", move |s| {
                s.pop_layer();
                s.add_layer(create_panel(year, 9, r9.clone()));
            })))
            .child(Panel::new(Button::new("Oct", move |s| {
                s.pop_layer();
                s.add_layer(create_panel(year, 10, r10.clone()));
            })))
            .child(Panel::new(Button::new("Nov", move |s| {
                s.pop_layer();
                s.add_layer(create_panel(year, 11, r11.clone()));
            })))
        )
        .child(Layer::new(TextView::new(" ")))
        .child(Panel::new(Button::new(format!("{}/{}/{}", c_month, c_day, c_year), move |s| {
            s.pop_layer();
            s.add_layer(create_panel(c_year, c_month, r12.clone()));
        })))
        ).title(year.to_string()))
    )
    .child(
        NamedView::new("view2", Panel::new(create_calendar(year, month, r13)
            .child(Layer::new(
                    LinearLayout::horizontal()
                        .child(
                            PaddedView::lrtb(36, 0, 0, 0, LinearLayout::vertical()
                                .child(
                                    Button::new("Up", move |s| {
                                        s.pop_layer();
                                        if month-1 == 0{
                                            s.add_layer(create_panel(year-1, 12, r14.clone()));
                                        } else {
                                            s.add_layer(create_panel(year, month-1, r15.clone()));
                                        }
                                    })
                                )
                                .child(
                                    Button::new("Down", move |s| {
                                        s.pop_layer();
                                        if month+1 == 13{
                                            s.add_layer(create_panel(year+1, 1, r16.clone()));
                                        } else {
                                            s.add_layer(create_panel(year, month+1, r17.clone()));
                                        }
                                    })
                                )
                            )
                        )
                        //.child(TextView::new(year.to_string()))
                )
            ))
            .title(month_to_string(month as i32)))
    )
    .child(Panel::new(Clock::new()))
    )
}

/// Moves top layer by the specified amount
fn move_top(c: &mut Cursive, x_in: isize, y_in: isize) {
    // Step 1. Get the current position of the layer.
    let s = c.screen_mut();
    let l = LayerPosition::FromFront(0);

    // Step 2. add the specifed amount
    let pos = s.offset().saturating_add((x_in, y_in));

    // convert the new x and y into a position
    let p = Position::absolute(pos);

    // Step 3. Apply the new position
    s.reposition_layer(l, p);
}

fn date_to_cell<T: TimeZone>(date: &Date<T>) -> cursive::XY<i32>{
    let cd = Utc.ymd(date.year(), date.month(), 1);
    let day_of_week = cd.weekday().number_from_sunday();

    let num = date.day0() + (day_of_week-1);
    let row = num/7;
    let column = num - (row*7);

    return (column as i32, row as i32).into()
}

fn cell_to_day<T: TimeZone>(cell : cursive::XY<i32>) -> u32{

    ((cell.y*7+(cell.x+1))-1) as u32
}

fn date_from_cell_offset<T: TimeZone>(
    date: &Date<T>,
    set_day: Option<i32>,
    x_offset: i32,
    y_offset: i32,
    month_offset: i32,
    year_offset: i32,
) -> Option<Date<T>> {
    let mut year = date.year() + year_offset;
    let mut month = date.month0() as i32;
    month += month_offset;

    // let num_days = ndays_in_month(date.year(), date.month());

    //let mut current_cell = date_to_cell(date);
    //current_cell = (current_cell.x + x_offset, current_cell.y + y_offset).into();

    let offset = y_offset*7 + x_offset;

    //

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

    // let mut day = set_day.unwrap_or_else(|| cmp::min(num_days - 1, date.day() as u32));

    // day += offset;

    // let d = date
    //     .with_day0(0)?
    //     .with_year(year)?
    //     .with_month0(month as u32)?;

    // if day < 0 {
    //     if month-1 < 0 {
    //         day += ndays_in_month(year, 12);
    //         date_from_cell_offset(&d, Some(day), 0, 0, 12, -1)
    //     }
    //     else{
    //         day += ndays_in_month(year, (month as u32)-1);
    //         date_from_cell_offset(&d, Some(day), 0, 0, -1, 0)
    //     }
    // } else if day >= num_days {
    //     day -= num_days;
    //     if month+1 >= 12 {
    //         date_from_cell_offset(&d, Some(day), 0, 0, -12, 1)
    //     }
    //     else{
    //         date_from_cell_offset(&d, Some(day), 0, 0, 1, 0)
    //     }
    // } else {
    //     d.with_day0(day as u32)
    // }
}

fn custom_theme_from_cursive(siv: &Cursive) -> Theme {
    // We'll return the current theme with a small modification.
    let mut theme = siv.current_theme().clone();

    theme
}

fn write(mut s: &HashMap<NaiveDate, Vec<TextEvent>>) {

    //let mut appended_file = new json

    // let file = File::open("test.txt").unwrap();

    // let mut string2 = String::from("{}");

    // for (key, event_list) in s {

    //     let mut string0 = String::new();

    //     for event in event_list {
    //         string0.push_str(&format!("\"{}\" \n \"{}\"", event.content, event.status));
    //     }

    //     let string1 = format!(
    //         "
    //         \"{}\": [
    //             {}
    //         ]
    //         "
    //         , key, string0);

    //     string2.insert_str(1, &string1);   
    // }

    // println!("{}", string2);

    let j = serde_json::to_string(&s).expect("could not serialize hashmap");
    fs::write("test.json", j).expect("Unable to write file");
}


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

// fn write(mut f: File, mut s: Storage<Utc>) -> std::io::Result<()> {
//     let mut buf_reader = BufReader::new(f);
//     let mut contents = String::new();
//     buf_reader.read_to_string(&mut contents)?;
//     assert_eq!(contents, "Hello, world!");
//     Ok(())
// }


fn main() {
    let utc: DateTime<Local> = Local::now();
    let year = utc.year();
    let month = utc.month();

    //let data = Rc::new(RefCell::new(Storage::new(HashMap::new())));
    let data = Rc::new(RefCell::new(Storage::new(read())));

    //println!("{}", utc.day());

    let mut siv = Cursive::default();

    siv.add_global_callback('w', |s| move_top(s, 0, -1));
    siv.add_global_callback('a', |s| move_top(s, -1, 0));
    siv.add_global_callback('s', |s| move_top(s, 0, 1));
    siv.add_global_callback('d', |s| move_top(s, 1, 0));

    siv.add_global_callback('k', |s| {
        s.call_on_name("calendar", |view: &mut CalendarView<Utc>| {
            // let temp: &mut Vec<TextEvent> = view.storage.borrow_mut().events.entry(view.view_date.clone()).or_insert(Vec::new());
            write(&view.storage.borrow_mut().events);
        });
    });

    // Request the data
    //let weather = util::weather::get_weather("Redmond,WA");

    // print it to the console
    //println!("Weather: {:?}", weather);

    let theme = custom_theme_from_cursive(&siv);
    siv.set_theme(theme);
    
    //let calendar = create_panel(year, month); 
    //let temp: &mut Vec<TextEvent> = view.events.entry(view.view_date.clone()).or_default();
    // data.calendar_data.entry(Utc.ymd(year, month, 1)).or_insert(calendar);

    siv.add_layer(create_panel(year, month, data));

    let sink = siv.cb_sink().clone();

    thread::spawn(move || {
        loop{
            thread::sleep(time::Duration::from_millis(500));
            sink.send(Box::new(|s: &mut Cursive| s.refresh()));
        }
    });



    siv.run();
}