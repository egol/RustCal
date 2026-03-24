use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use chrono::prelude::*;
use chrono::Datelike;
use chrono::{DateTime, Duration, Local};
use cursive::traits::*;
use cursive::theme::Theme;
use cursive::view::Position;
use cursive::views::LayerPosition;
use cursive::views::NamedView;
use cursive::views::*;
use cursive::Cursive;

#[macro_use]
extern crate serde_derive;

pub mod backend;
pub mod calendarview;
pub mod clock;
pub mod pomodoro;
pub mod tasklist;
pub mod todolist;
pub mod util;
#[cfg(target_family = "wasm")]
pub mod wasm;

use calendarview::*;
use tasklist::*;
use util::file::{read, save_data};
use util::month::{abbr_month_to_string, month_to_string};
use util::storage::Storage;
use util::textevent::TextEvent;
use util::timer::PomodoroTimer;

fn create_panel(
    year: i32,
    month: u32,
    st: Arc<Mutex<Storage>>,
    timer: Arc<Mutex<PomodoroTimer>>,
) -> Panel<LinearLayout> {
    let utc: DateTime<Local> = Local::now();
    let c_year = utc.year();
    let c_month = utc.month();
    let c_day = utc.day();

    let st_clone_panel = Arc::clone(&st);
    let st_clone_panel2 = Arc::clone(&st);
    let st_clone_panel3 = Arc::clone(&st);
    let st_clone_panel4 = Arc::clone(&st);

    let events_clone: HashMap<NaiveDate, Vec<TextEvent>>;
    {
        let st_clone_panel5: Arc<Mutex<Storage>> = Arc::clone(&st);
        events_clone = st_clone_panel5.lock().unwrap().events.clone();
    }

    let tm_clone = Arc::clone(&timer);
    let tm_clone2 = Arc::clone(&timer);
    let tm_clone3 = Arc::clone(&timer);
    let tm_clone4 = Arc::clone(&timer);

    Panel::new(
        LinearLayout::vertical()
            .child(
                LinearLayout::horizontal()
                    .child(Panel::new(Layer::new(
                        TextView::new(" ").center().fixed_width(4),
                    )))
                    .child(NamedView::new(
                        "clock",
                        Panel::new(PaddedView::lrtb(2, 0, 0, 0, clock::Clock::new())).max_width(80),
                    ))
                    .child(Panel::new(Layer::new(
                        TextView::new(" ").center().fixed_width(4),
                    ))),
            )
            .child(
                LinearLayout::horizontal()
                    .child(
                        NamedView::new(
                            "view1",
                            Panel::new(
                                LinearLayout::vertical()
                                    .with(|column| {
                                        let num_rows = 3;
                                        let mut month = 1;

                                        for _rows in 0..(12 / num_rows) {
                                            column.add_child(LinearLayout::horizontal().with(
                                                |row| {
                                                    row.add_child(TextView::new("  "));

                                                    for _columns in 0..num_rows {
                                                        let st_clone = Arc::clone(&st);
                                                        let tm_clone = Arc::clone(&tm_clone2);

                                                        row.add_child(Panel::new(Button::new(
                                                            abbr_month_to_string(month),
                                                            move |s| {
                                                                s.pop_layer();
                                                                s.add_layer(create_panel(
                                                                    year,
                                                                    month as u32,
                                                                    Arc::clone(&st_clone),
                                                                    Arc::clone(&tm_clone),
                                                                ));
                                                            },
                                                        )));

                                                        month += 1;
                                                    }

                                                    row.add_child(TextView::new("  "));
                                                },
                                            ))
                                        }
                                    })
                                    .child(Panel::new(Button::new(
                                        format!("{}/{}/{}", c_month, c_day, c_year),
                                        move |s| {
                                            s.pop_layer();

                                            let st_clone = Arc::clone(&st_clone_panel4);
                                            let tm_clone = Arc::clone(&tm_clone);

                                            s.add_layer(create_panel(
                                                c_year,
                                                c_month,
                                                Arc::clone(&st_clone),
                                                tm_clone,
                                            ));
                                        },
                                    )))
                                    .child(Layer::new(TextView::new(" ")))
                                    .child(
                                        LinearLayout::vertical()
                                            .child(
                                                Panel::new(
                                                    create_task_list_view(events_clone).scrollable(),
                                                )
                                                .title("Next 7 Days")
                                                .max_height(15),
                                            )
                                            .with_name("tasklist"),
                                    )
                                    .child(Panel::new(Button::new("Pomodoro Timer", move |s| {
                                        let tm_clone = Arc::clone(&timer);
                                        let pm = pomodoro::Pomodoro::new(tm_clone);
                                        pomodoro::create_pomodoro_timer(s, pm);
                                    }))),
                            )
                            .title(year.to_string()),
                        )
                        .max_width(27),
                    )
                    .child(NamedView::new(
                        "view2",
                        Panel::new(
                            create_calendar(year, month, Arc::clone(&st_clone_panel)).child(
                                PaddedView::lrtb(
                                    ((7 * 9) / 2) - ((4 + 4 + 4) / 2),
                                    0,
                                    0,
                                    0,
                                    LinearLayout::horizontal()
                                        .child(Button::new("Prev", move |s| {
                                            s.pop_layer();

                                            let st_clone = Arc::clone(&st_clone_panel2);
                                            let tm_clone = Arc::clone(&tm_clone3);

                                            if month - 1 == 0 {
                                                s.add_layer(create_panel(
                                                    year - 1,
                                                    12,
                                                    Arc::clone(&st_clone),
                                                    tm_clone,
                                                ));
                                            } else {
                                                s.add_layer(create_panel(
                                                    year,
                                                    month - 1,
                                                    Arc::clone(&st_clone),
                                                    tm_clone,
                                                ));
                                            }
                                        }))
                                        .child(Button::new("Next", move |s| {
                                            s.pop_layer();

                                            let st_clone = Arc::clone(&st_clone_panel3);
                                            let tm_clone = Arc::clone(&tm_clone4);

                                            if month + 1 == 13 {
                                                s.add_layer(create_panel(
                                                    year + 1,
                                                    1,
                                                    Arc::clone(&st_clone),
                                                    tm_clone,
                                                ));
                                            } else {
                                                s.add_layer(create_panel(
                                                    year,
                                                    month + 1,
                                                    Arc::clone(&st_clone),
                                                    tm_clone,
                                                ));
                                            }
                                        })),
                                ),
                            ),
                        )
                        .title(month_to_string(month as i32)),
                    )),
            ),
    )
}

fn move_top(c: &mut Cursive, x_in: isize, y_in: isize) {
    let s = c.screen_mut();
    let l = LayerPosition::FromFront(0);
    let pos = s
        .layer_offset(LayerPosition::FromFront(0))
        .unwrap()
        .saturating_add((x_in, y_in));
    let p = Position::absolute(pos);
    s.reposition_layer(l, p);
}

fn custom_theme_from_cursive(siv: &Cursive) -> Theme {
    siv.current_theme().clone()
}

pub fn build_app() -> Cursive {
    let utc: DateTime<Local> = Local::now();
    let year = utc.year();
    let month = utc.month();

    let data = Arc::new(Mutex::new(Storage::new(read())));
    let timer = Arc::new(Mutex::new(PomodoroTimer::new(
        Duration::minutes(25),
        Duration::minutes(5),
        Duration::minutes(15),
    )));

    let mut siv = Cursive::new();

    siv.add_global_callback('w', |s| move_top(s, 0, -1));
    siv.add_global_callback('a', |s| move_top(s, -1, 0));
    siv.add_global_callback('s', |s| move_top(s, 0, 1));
    siv.add_global_callback('d', |s| move_top(s, 1, 0));

    if std::env::var_os("RUSTCAL_DISABLE_SAVE").is_none() {
        siv.add_global_callback('k', |s| {
            s.call_on_name("calendar", |view: &mut CalendarView<Utc>| {
                let mut_storage_ref = view.storage.lock().unwrap();
                save_data(&mut_storage_ref.events);
            });
        });
    }

    let theme = custom_theme_from_cursive(&siv);
    siv.set_theme(theme);
    siv.add_layer(create_panel(year, month, data, timer));
    siv.set_autorefresh(true);
    siv
}
