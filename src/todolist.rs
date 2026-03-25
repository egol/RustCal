/// Contains all of the helpers and struct definition of the todo list popup
// External Dependencies ------------------------------------------------------
use chrono::prelude::*;
use cursive::direction::Direction;
use cursive::event::Key;
use cursive::event::{Callback, Event, EventResult, MouseButton, MouseEvent};
use cursive::theme::*;
use cursive::traits::*;
use cursive::view::CannotFocus;
use cursive::views::NamedView;
use cursive::views::*;
use cursive::Cursive;
use cursive::Printer;
use cursive::Vec2;

// Internal dependencies ------------------------------------------------------
use crate::calendarview::*;
use crate::tasklist::*;
use crate::util::file::*;
use crate::util::textevent::*;

pub struct TodoList {
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

    fn get_cell(&mut self, mouse_pos: Vec2, offset: Vec2) -> Option<Vec2> {
        let diff = mouse_pos.map(|v| v as i64) - offset.map(|v| v as i64);
        let pos: cursive::XY<i32> = ((diff.x / 1) as i32, (diff.y / 1) as i32).into();

        Some((pos.x, pos.y).into())
    }

    fn draw_list(&self, p: &Printer) {
        if self.events_list.len() > 0 {
            for y in 1..self.events_list.len() + 1 {
                for x in 0..self.size.x {
                    if x == 0 {
                        if self
                            .focused
                            .map(|focused| focused.x == 0 && focused.y == y)
                            .unwrap_or(false)
                        {
                            p.with_color(ColorStyle::highlight(), |printer| {
                                printer.print((x, y), "X");
                            });
                        } else {
                            p.with_color(ColorStyle::primary(), |printer| {
                                printer.print((x, y), "x");
                            });
                        }
                    }
                    if x == 2 || x == 3 {
                        if self.events_list[y - 1].status == 1 {
                            p.with_color(
                                ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(190, 190, 90)),
                                |printer| {
                                    printer.print((x, y), " ");
                                },
                            );
                        } else if self.events_list[y - 1].status == 2 {
                            p.with_color(
                                ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(190, 90, 90)),
                                |printer| {
                                    printer.print((x, y), " ");
                                },
                            );
                        } else {
                            p.with_color(
                                ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(90, 190, 90)),
                                |printer| {
                                    printer.print((x, y), " ");
                                },
                            );
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
                                if (pos.x > 0 && pos.x < 4)
                                    && pos.y > 0
                                    && !self.events_list.is_empty()
                                    && pos.y <= self.events_list.len()
                                {
                                    let Some(event) = self.events_list.get(pos.y - 1) else {
                                        self.focused = None;
                                        return EventResult::Ignored;
                                    };
                                    let status = if event.status < 2 { event.status + 1 } else { 0 };
                                    let task_id = event.id;

                                    return EventResult::Consumed(Some(Callback::from_fn(
                                        move |s| {
                                            s.call_on_name(
                                                "calendar",
                                                |view: &mut CalendarView<Utc>| {
                                                    let mut storage_ref_mut =
                                                        view.storage.lock().unwrap();
                                                    let _ = storage_ref_mut.set_status(
                                                        view.view_date.date_naive(),
                                                        task_id,
                                                        status,
                                                    );
                                                },
                                            );

                                            refresh_task_list_view::<Utc>(s);
                                            refresh_todo_views::<Utc>(s);
                                        },
                                    )));
                                }
                                // If x button is pressed on todo list
                                else if pos.x == 0
                                    && pos.y > 0
                                    && !self.events_list.is_empty()
                                    && pos.y <= self.events_list.len()
                                {
                                    let Some(event) = self.events_list.get(pos.y - 1) else {
                                        self.focused = None;
                                        return EventResult::Ignored;
                                    };
                                    let task_id = event.id;

                                    return EventResult::Consumed(Some(Callback::from_fn(
                                        move |s| {
                                            s.call_on_name(
                                                "calendar",
                                                |view: &mut CalendarView<Utc>| {
                                                    let mut storage_ref_mut =
                                                        view.storage.lock().unwrap();

                                                    let _ = storage_ref_mut.remove_event(
                                                        view.view_date.date_naive(),
                                                        task_id,
                                                    );
                                                },
                                            );

                                            refresh_task_list_view::<Utc>(s);
                                            refresh_todo_views::<Utc>(s);
                                        },
                                    )));
                                }
                            }
                            _ => (),
                        }
                    }
                    self.focused = None;
                }
            }
            // exit todo list on popup as well
            Event::Key(Key::Enter) => {
                return EventResult::Consumed(Some(Callback::from_fn(move |s| {
                    s.call_on_name("calendar", |view: &mut CalendarView<Utc>| {
                        let mut_storage_ref = view.storage.lock().unwrap();
                        save_data(&mut_storage_ref.events);
                    });

                    s.pop_layer();
                })))
            }
            _ => (),
        }
        return EventResult::Ignored;
    }
}

fn sync_editor_list<T>(view: &mut ListView, events: &[TextEvent])
where
    T: TimeZone + Send + Sync + 'static,
    CalendarView<T>: View,
{
    while view.len() > 1 {
        view.remove_child(1);
    }

    for event in events {
        view.add_child("", create_event_editor::<T>(event.content.clone(), event.id));
    }
}

pub fn refresh_todo_views<T>(s: &mut Cursive)
where
    T: TimeZone + Send + Sync + 'static,
    CalendarView<T>: View,
{
    let mut events = Vec::new();

    s.call_on_name("calendar", |view: &mut CalendarView<T>| {
        events = view
            .storage
            .lock()
            .unwrap()
            .events_for_date(view.view_date.date_naive());
    });

    s.call_on_name("list", |view: &mut NamedView<TodoList>| {
        view.get_mut().sync_events(events.clone());
    });

    s.call_on_name("todo", |view: &mut NamedView<ListView>| {
        let mut list = view.get_mut();
        sync_editor_list::<T>(&mut list, &events);
    });
}

// creates the todo list popup
pub fn create_todo_list<T: TimeZone + Send + Sync + 'static>(s: &mut Cursive)
where
    T: TimeZone + Send + Sync + 'static,
    CalendarView<T>: View,
{
    s.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(NamedView::new("list", TodoList::new()))
                .child(
                    LinearLayout::vertical()
                        .child(
                            NamedView::new(
                                "todo",
                                ListView::new()
                                    .delimiter(),
                            )
                            .scrollable(),
                        )
                        .child(Button::new_raw("<+>", |s| {
                            s.call_on_name("calendar", |view: &mut CalendarView<T>| {
                                let mut storage_ref_mut = view.storage.lock().unwrap();
                                storage_ref_mut.add_event(view.view_date.date_naive());
                            });

                            refresh_task_list_view::<T>(s);
                            refresh_todo_views::<T>(s);
                        })), //.max_height(15)
                ),
        )
        .title("Todo")
        .button("Ok", |s| {
            // this writes the current content of the storage to the json save file
            // saves on exiting the todo menu
            s.call_on_name("calendar", |view: &mut CalendarView<Utc>| {
                let mut_storage_ref = view.storage.lock().unwrap();
                save_data(&mut_storage_ref.events);
            });

            s.pop_layer();
        }),
    );

    refresh_todo_views::<T>(s);
}
