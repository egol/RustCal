/// Contains all of the logic necessary to draw the main calendar window with days

// STD Dependencies -----------------------------------------------------------
use std::cmp;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// External Dependencies ------------------------------------------------------
use chrono::prelude::*;
use chrono::Datelike;
use cursive::traits::*;
use cursive::Cursive;
use cursive::Vec2;
use cursive::Printer;
use cursive::direction::Direction;
use cursive::view::CannotFocus;
use cursive::theme::*;
use cursive::views::*;
use cursive::views::NamedView;
use cursive::event::{Callback, Event, EventResult, Key, MouseButton, MouseEvent};

// Internal Dependencies ------------------------------------------------------
use crate::util::storage::*;
use crate::util;
use crate::util::textevent::*;
use crate::tasklist::*;
use crate::util::month::*;
use crate::todolist::*;

pub struct CalendarView<T: TimeZone> {
    enabled: bool,
    /// date that is selected
    pub view_date: DateTime<T>,
    // on_select: Option<DateCallback<T>>,
    size: Vec2,
    earliest_date: Option<DateTime<T>>,
    latest_date: Option<DateTime<T>>,
    date: DateTime<chrono::Local>,
    focused: Option<Vec2>,
    /// todays date
    current_date: cursive::XY<i32>,
    pub storage: Arc<Mutex<Storage>>,
}

// type DateCallback<T> = Arc<dyn Fn(&mut Cursive, &DateTime<T>) + Send + Sync>;

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
            // on_select: None,
            earliest_date: None,
            latest_date: None,
            focused: None,
            current_date: (0,0).into(),
            storage: s,
        }
    }

    /// Sets a callback to be used when an a new date is visually selected.
    // pub fn set_on_select<F>(&mut self, cb: F)
    // where
    //     F: Fn(&mut Cursive, &DateTime<T>) + Send + Sync + 'static,
    // {
    //     self.on_select = Some(Arc::new(move |s, date| cb(s, date)));
    // }

    /// Sets a callback to be used when an a new date is visually selected.
    // pub fn on_select<F>(self, cb: F) -> Self
    // where
    //     F: Fn(&mut Cursive, &DateTime<T>) + Send + Sync + 'static,
    // {
    //     self.with(|v| v.set_on_select(cb))
    // }

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
                
                let mut totals_incomplete = vec![0,0,0];
                let mut totals_complete = vec![0,0,0];

                // totals up uncompleted event status
                for a in events.iter(){
                    if !a.completed {
                        totals_incomplete[a.status as usize] += 1;
                    }
                }
                // totals up completed event status
                for a in events.iter(){
                    if a.completed {
                        totals_complete[a.status as usize] += 1;
                    }
                }

                self.draw_cell(printer, x as u8, y as u8, format!("{:>2}", day_number + 1),
                    color, totals_incomplete, totals_complete, past);
            }
        }

    }

    fn draw_cell (&self, p: &Printer, offset_x : u8, offset_y : u8, day : String, color : ColorStyle,
        nums_incomplete: Vec<i32>, nums_complete: Vec<i32>, past : bool) {
        // sets the size of one calendar cell
        let x_max : u8 = 11;
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

                // draw current/future event totals
                else if x == 1 && !past {

                    let mut content: String = "".to_string();
                    let mut color = ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(90, 190, 90));

                    if y == 1 && nums_incomplete[0] > 0 {
                        content = format!("{:>2}", nums_incomplete[0]);
                    }
                    else if y == 2 && nums_incomplete[1] > 0 {
                        color = ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(190, 190, 90));
                        content = format!("{:>2}", nums_incomplete[1]);
                    }
                    else if y == 3 && nums_incomplete[2] > 0 {
                        color = ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(190, 90, 90));
                        content = format!("{:>2}", nums_incomplete[2]);
                    }

                    if y >= 1 && y <= 3 && content.len() > 0{
                        p.with_color(color, |printer| {
                            printer.print((x + offset_x, y + offset_y), &content);
                        });
                    }

                }
                // draw completed current and future events
                else if x == 3 && !past {

                    let mut color = ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(140, 240, 140));
                    let mut content: String = "".to_string();

                    if y == 1 && nums_complete[0] > 0 {
                        content = format!("{:>2}", nums_complete[0]);
                    }
                    else if y == 2 && nums_complete[1] > 0 {
                        color = ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(240, 240, 140));
                        content = format!("{:>2}", nums_complete[1]);
                    }
                    else if y == 3 && nums_complete[2] > 0 {
                        color = ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(240, 140, 140));
                        content = format!("{:>2}", nums_complete[2]);
                    }

                    if y >= 1 && y <= 3 && content.len() > 0 {
                        p.with_color(color, |printer| {
                            printer.print((x + offset_x, y + offset_y), &content);
                        });
                    }

                }
                // draw past event totals (greyed out)
                else if x == 1 && past {

                    let color = ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(70, 70, 70));
                    let mut content: String = "".to_string();

                    if y == 1 && nums_incomplete[0] > 0 {
                        content = format!("{:>2}", nums_incomplete[0]);
                    }
                    else if y == 2 && nums_incomplete[1] > 0 {
                        content = format!("{:>2}", nums_incomplete[1]);
                    }
                    else if y == 3 && nums_incomplete[2] > 0 {
                        content = format!("{:>2}", nums_incomplete[2]);
                    }

                    if y >= 1 && y <= 3 && content.len() > 0 {
                        p.with_color(color, |printer| {
                            printer.print((x + offset_x, y + offset_y), &content);
                        });
                    }

                }
                // draw completed past events
                else if x == 3 && past {

                    let color = ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(110, 110, 110));
                    let mut content: String = "".to_string();

                    if y == 1 && nums_complete[0] > 0 {
                        content = format!("{:>2}", nums_complete[0]);
                    }
                    else if y == 2 && nums_complete[1] > 0 {
                        content = format!("{:>2}", nums_complete[1]);
                    }
                    else if y == 3 && nums_complete[2] > 0 {
                        content = format!("{:>2}", nums_complete[2]);
                    }

                    if y >= 1 && y <= 3 && content.len() > 0 {
                        p.with_color(color, |printer| {
                            printer.print((x + offset_x, y + offset_y), &content);
                        });
                    }

                }
                else if x == 7 && y == 1 {
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

    // TODO: Causing crash after removing element and then trying to edit
    EditView::new()
        .on_edit(move |s: &mut Cursive, text, _cursor| {

            let mut events_clone: HashMap<NaiveDate, Vec<TextEvent>> = Default::default();

            s.call_on_name("calendar", |view: &mut CalendarView<T>| {

                let mut storage_ref_mut = view.storage.lock().unwrap();

                storage_ref_mut.events.entry(NaiveDate::from_ymd_opt(view.view_date.year(), view.view_date.month(), view.view_date.day()).unwrap())
                    .or_insert(Vec::new())[i].content = String::from(text);

                events_clone = storage_ref_mut.events.clone();
            
            });

            update_task_list_view(s, events_clone);

        })
        .content(content)
        .fixed_width(25)
}

/// Creates the actual calendar panel
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

fn date_to_cell<T: TimeZone>(date: &DateTime<T>) -> cursive::XY<i32>{
    let cd = Utc.with_ymd_and_hms(date.year(), date.month(), 1, 0, 0, 0).unwrap();
    let day_of_week = cd.weekday().number_from_sunday();

    let num = date.day0() + (day_of_week-1);
    let row = num/7;
    let column = num - (row*7);

    return (column as i32, row as i32).into()
}