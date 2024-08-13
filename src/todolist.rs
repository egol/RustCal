// STD Dependencies -----------------------------------------------------------
use std::collections::HashMap;

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
use cursive::event::{Callback, Event, EventResult, MouseButton, MouseEvent};

// Debug dependencies ---------------------------------------------------------

use log::info;

// Internal dependencies ------------------------------------------------------
use crate::util::textevent::*;
use crate::tasklist::*;
use crate::calendarview::*;
use crate::util::file::*;

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
                                // If the status button is pressed on the todo
                                if (pos.x > 0 && pos.x < 4) && pos.y > 0 && self.events_list.len() > 0 && pos.y <= self.events_list.len(){
                                    if self.events_list[pos.y-1].status < 2 {
                                        self.events_list[pos.y-1].status += 1;
                                    }
                                    else {
                                        self.events_list[pos.y-1].status = 0;
                                    }

                                    let status = self.events_list[pos.y-1].status;

                                    return EventResult::Consumed(Some(Callback::from_fn(move |s| {

                                        let mut events_clone: HashMap<NaiveDate, Vec<TextEvent>> = Default::default();

                                        s.call_on_name("calendar", |view: &mut CalendarView<Utc>| {

                                            let mut storage_ref_mut = view.storage.lock().unwrap();

                                            storage_ref_mut.events.entry(
                                                NaiveDate::from_ymd_opt(view.view_date.year(), view.view_date.month(), view.view_date.day()).unwrap())
                                                .or_insert(Vec::new())[pos.y-1].status = status;

                                            events_clone = storage_ref_mut.events.clone();

                                        });

                                        update_task_list_view(s, events_clone);

                                    })));
                                }
                                // If x button is pressed on todo list
                                else if pos.x == 0 && pos.y > 0 && self.events_list.len() > 0 && pos.y <= self.events_list.len(){
                                    self.events_list.remove(pos.y-1);

                                    return EventResult::Consumed(Some(Callback::from_fn(move |s| {

                                        // TODO: has to be a cleaner way of updating the task list based off of storage update
                                        // TODO: cant edit other tasks after deleting a task??
                                        let mut events_clone: HashMap<NaiveDate, Vec<TextEvent>> = Default::default();

                                        s.call_on_name("calendar", |view: &mut CalendarView<Utc>| {

                                            let mut storage_ref_mut = view.storage.lock().unwrap();

                                            info!("{:?}", storage_ref_mut.events);

                                            storage_ref_mut.events.entry(NaiveDate::from_ymd_opt(view.view_date.year(),
                                                view.view_date.month(), view.view_date.day()).unwrap())
                                                .or_default().remove(pos.y-1);

                                            events_clone = storage_ref_mut.events.clone();

                                            info!("{:?}", storage_ref_mut.events);

                                        });

                                        s.call_on_name("todo", |view: &mut NamedView<ListView>| {
                                            let mut view = view.get_mut();
                                            view.remove_child(pos.y);
                                        });

                                        update_task_list_view(s, events_clone);
                                        
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

                        let mut events_clone: HashMap<NaiveDate, Vec<TextEvent>> = Default::default();

                        // creates the entries in the calendar cells + storage
                        s.call_on_name("calendar", |view: &mut CalendarView<T>| {

                            let mut storage_ref_mut = view.storage.lock().unwrap();
                            
                            // create a new event in the storage if there is none at the hash map date specified
                            storage_ref_mut.events.entry(
                                NaiveDate::from_ymd_opt(view.view_date.year(), view.view_date.month(), view.view_date.day()).unwrap())
                                .or_insert(Vec::new()).push(TextEvent::new(String::from("")));

                            events = storage_ref_mut.events.entry(
                                NaiveDate::from_ymd_opt(view.view_date.year(), view.view_date.month(), view.view_date.day()).unwrap()).or_default().clone();
                            
                            events_clone = storage_ref_mut.events.clone()

                        });

                        update_task_list_view(s, events_clone);

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
        
        .button("Ok", |s| {

            // this writes the current content of the storage to the json save file
            // saves on exiting the todo menu
            s.call_on_name("calendar", |view: &mut CalendarView<Utc>| {

                let mut_storage_ref = view.storage.lock().unwrap();
                save_data(&mut_storage_ref.events);
    
            });

            s.pop_layer();

        }));

}