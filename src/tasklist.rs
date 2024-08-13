/// Contains all methods related to the task list as well as
/// The tasklist struct which contains the custom rendering of the list entries

// STD Dependencies -----------------------------------------------------------
use std::collections::HashMap;

// External Dependencies ------------------------------------------------------
use chrono::{prelude::*, Duration};
use chrono::Datelike;
use cursive::traits::*;
use cursive::Cursive;
use cursive::Vec2;
use cursive::Printer;
use cursive::direction::Direction;
use cursive::view::CannotFocus;
use cursive::theme::*;
use cursive::views::*;
use cursive::event::{Callback, Event, EventResult, MouseButton, MouseEvent};
use cursive::utils::markup::StyledString;

// Debug dependencies ---------------------------------------------------------

use log::info;

// Internal dependencies ------------------------------------------------------
use crate::util::textevent::*;
use crate::calendarview::*;
use crate::util::month::*;

// #[derive(Clone, Serialize, Deserialize, Debug)]
// pub struct TextEvent{
//     content: String,
//     status: i8,
//     completed: bool,
// }

// impl TextEvent {
//     pub fn new(s: String) -> Self {
//         Self {
//             content: s,
//             status: 0,
//             completed: false,
//         }
//     }

// }

pub struct TaskList {
    enabled: bool,
    size: Vec2,
    // focused: Option<Vec2>,
    text_event: TextEvent,
    date: DateTime<Local>,
    index: usize,
    id: i32,
}

impl TaskList {

    // Initializes the struct
    pub fn new(date: DateTime<Local>, index: usize, event: TextEvent, id: i32) -> Self {
        Self {
            enabled: true,
            size: (0, 0).into(),
            // focused: None,
            text_event: event,
            date: date,
            index: index,
            id: id,
        }
    }

    fn draw_list(&self, p: &Printer) {
        // info!("{}", self.text_event.content);
        if self.text_event.content.len() > 0{
            for x in 0..self.size.x {

                let mut background_color;

                if self.text_event.status == 1{
                    background_color = ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(190, 190, 90));
                    if self.text_event.completed {
                        background_color = ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(240, 240, 140));
                    }
                }
                else if self.text_event.status == 2{
                    background_color = ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(190, 90, 90));
                    if self.text_event.completed {
                        background_color = ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(240, 140, 140));
                    }
                }
                else{
                    background_color = ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(90, 190, 90));
                    if self.text_event.completed {
                        background_color = ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(140, 240, 140));
                    }
                }

                if x == 1 {
                    if self.text_event.completed {
                        p.with_color(background_color, |printer| {
                            printer.print((x, 0), "x");
                        });
                    }
                    else {
                        p.with_color(background_color, |printer| {
                            printer.print((x, 0), ".");
                        });
                    }
                }
                else {
                    p.with_color(background_color, |printer| {
                        printer.with_style(Style::primary().combine(Effect::Bold),|printer| {
                            printer.print(
                            (x, 0), 
                            format!("{}", self.index+1).as_str());
                        })
                    });
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
        self.size = (2, 1).into();
        (2, 1).into()
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
                offset: _,
                position: _,
                event: MouseEvent::Release(btn),
            } => {
                // if self.focused == Some(pos) {
                match btn {
                    MouseButton::Left => { 
                        self.text_event.completed = !self.text_event.completed;
                        let completed_status = self.text_event.completed;

                        let text_event_date = self.date;

                        let text_event_index = self.index;
                        let id = self.id;

                        info!("{:?}", self.text_event);

                        return EventResult::Consumed(Some(Callback::from_fn(move |s| {

                            s.call_on_name("calendar", |view: &mut CalendarView<Utc>| {

                                let mut storage_ref_mut = view.storage.lock().unwrap();

                                storage_ref_mut.events.entry(
                                    NaiveDate::from_ymd_opt(text_event_date.year(), text_event_date.month(), text_event_date.day()).unwrap())
                                    .or_insert(Vec::new())[text_event_index].completed = completed_status;
                                
                                info!("{:?}", storage_ref_mut.events.entry(
                                    NaiveDate::from_ymd_opt(text_event_date.year(), text_event_date.month(), text_event_date.day()).unwrap())
                                    .or_insert(Vec::new())[text_event_index]);

                            });

                            // need to refresh the TaskView in order to render the changes
                            if completed_status {
                                s.call_on_name(&format!("{id}"), |view: &mut TextView| {
                                    view.set_style(Style::primary().combine(Effect::Strikethrough));
                                });
                            }
                            else {
                                s.call_on_name(&format!("{id}"), |view: &mut TextView| {
                                    // this line is necessary to force refresh the text style
                                    view.set_content(view.get_content().source());

                                    view.set_style(Style::primary().combine(Effect::Simple));
                                });
                            }

                        })));
                    }
                    _ => (),
                }
                // }
                // self.focused = None;
        }
            _ => (),
        }

        return EventResult::Ignored;
    }

}

/// creates the Vec of "coming up" tasks taken from the todo list
fn create_task_list(mut events: HashMap<NaiveDate, Vec<TextEvent>>) -> Vec<Vec<TextEvent>> {

    // A list of all events in the next 7 days
    let mut week_events: Vec<Vec<TextEvent>> = Vec::new();

    // get current date
    let utc: DateTime<Local> = Local::now();

    // need to go through the next 7 days where the first day is the present
    for i in 0..8 {
        week_events.push(events.entry(
            NaiveDate::from_ymd_opt(utc.year(), utc.month(), utc.day() + i).unwrap()).or_default().clone());
    }

    // TODO: This sorting step makes identifying the events difficult 
    // readd in the future
    // start creating the list
    // sort the tasks by days, then priorities, then alphabetical
    // for events in &mut week_events {
    //     events.sort_by(|a, b| {

    //         // First, compare by status in descending order
    //         let status_cmp = b.status.cmp(&a.status);
            
    //         // If status is equal, compare alphabetically by content
    //         if status_cmp == std::cmp::Ordering::Equal {
    //             a.content.cmp(&b.content)
    //         } else {
    //             status_cmp
    //         }
    //     });
    // }

    return week_events

}

pub fn create_task_list_view(events: HashMap<NaiveDate, Vec<TextEvent>>) -> ListView {

    ListView::new()
        .delimiter()
        .with(|list| {

            let global_tasks = create_task_list(events);

            let utc: DateTime<Local> = Local::now();

            let mut id_counter = 0;

            for (day, tasks) in global_tasks.iter().enumerate() { 

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

                        if task.content.len() > 0 {

                            let task_name: &str = &task.content.clone().to_owned();

                            // TODO: code could be less redundant
                            if task.completed {

                                list.add_child(
                                    "",
                                    LinearLayout::horizontal()
                                            .child(TaskList::new(task_date, task_counter, task.clone(), id_counter))
                                            .child(TextView::new(" "))
                                            .child(TextView::new(StyledString::styled(
                                                format!("{}", task_name), Style::primary().combine(Effect::Strikethrough)))
                                                .with_name(format!("{id_counter}"))
                                            )
                                    );

                            }
                            else {

                                list.add_child(
                                "",
                                LinearLayout::horizontal()
                                        .child(TaskList::new(task_date, task_counter, task.clone(), id_counter))
                                        .child(TextView::new(" "))
                                        .child(TextView::new(StyledString::styled(
                                            format!("{}", task_name), Style::primary().combine(Effect::Simple)))
                                            .with_name(format!("{id_counter}"))
                                        )
                                );
                            }

                            task_counter += 1;
                            id_counter += 1;
                        }
                    }
                }
            }
        })

}

/// update tasklist by recreating it
pub fn update_task_list_view(s: &mut Cursive, events: HashMap<NaiveDate, Vec<TextEvent>>) {

    s.call_on_name("tasklist", |view: &mut LinearLayout| {
        view.remove_child(0);
        view.add_child(Panel::new(
            create_task_list_view(events)
            .scrollable()
        ).title("Next 7 Days").max_height(25))
    });

}
