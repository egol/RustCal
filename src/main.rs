// TODO:
// https://docs.rs/cursive/0.14.1/cursive/view/trait.View.html
// implement the needs re-layout function to improve performance?

// TODO:
// 3. Add more priorities?
// 4. Rework Readme for release
// 5. Add todo list functionality?
// 6. Add in pomodoro timer button
// 7. at 12:37am time shows as 00:37
// 8. Allow shifting of the task list time frame via drop down menu
//    Look at previous 7 days or two weeks from now ect.
// 9. Fix bug involving deleting tasks causing crashes or inability to interact with text field

// STD Dependencies -----------------------------------------------------------
use std::collections::HashMap;
use std::fs::File;
use std::sync::{Arc, Mutex};

// External Dependencies ------------------------------------------------------
use chrono::prelude::*;
use chrono::Datelike;
use cursive::traits::*;
use cursive::Cursive;
#[macro_use] extern crate serde_derive;
use cursive::theme::*;
use cursive::views::*;
use cursive::view::Position;
use cursive::views::NamedView;
use cursive::views::LayerPosition;

// Debug dependencies ---------------------------------------------------------

use log::{info, LevelFilter};
use simplelog::{Config, WriteLogger};

// Internal Dependencies ------------------------------------------------------
mod util;
mod tasklist;
mod calendarview;
mod todolist;
mod clock;

use util::textevent::*;
use util::storage::*;
use tasklist::*;
use calendarview::*;
use util::month::*;
use util::file::*;

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

    let events_clone: HashMap<NaiveDate, Vec<TextEvent>>;
    {   
        let st_clone_panel5: Arc<Mutex<Storage>> = Arc::clone(&st);
        events_clone = st_clone_panel5.lock().unwrap().events.clone();
    }

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
            )).min_width(27))

            .child(NamedView::new("clock", 
            Panel::new(PaddedView::lrtb(3, 0, 0, 0, clock::Clock::new())).max_width(80)
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

                                    row.add_child(TextView::new("  "));

                                    // generate the children with a for loop
                                    for _columns in 0..num_rows {

                                        let st_clone = Arc::clone(&st);
                                        
                                        row.add_child(Panel::new(Button::new(abbr_month_to_string(month), move |s| {
                                            s.pop_layer();
                                            s.add_layer(create_panel(year, month as u32, Arc::clone(&st_clone)));
                                        })));

                                        month += 1;
                                    }

                                    row.add_child(TextView::new("  "));
                                })
                            )

                        }
                    })
                    // spacer
                    .child(Layer::new(TextView::new(" ")))

                    // Task list
                    .child(
                    LinearLayout::vertical().child(
                    Panel::new(
                            // DummyView
                            create_task_list_view(events_clone)
                            .scrollable()
                        ).title("Next 7 Days").max_height(25)).with_name("tasklist")
                    )

                    // spacer
                    // .child(Layer::new(TextView::new(" ")))

                    // TODO: Create Pomodoro timer
                    .child(Panel::new(Button::new("Pomodoro Timer", move|s| {
                        
                        
        
                    })))

                    ).title(year.to_string()).max_width(27)
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

// placeholder method for future theme managment
fn custom_theme_from_cursive(siv: &Cursive) -> Theme {
    let theme = siv.current_theme().clone();
    theme
}

/// main function that initializes storage and launches app
fn main() {
    let utc: DateTime<Local> = Local::now();
    let year = utc.year();
    let month = utc.month();

    // Create a log file
    let log_file = File::create("app.log").unwrap();

    // Initialize the logger to write to the file
    WriteLogger::init(LevelFilter::Info, Config::default(), log_file).unwrap();

    let data = Arc::new(Mutex::new(Storage::new(read())));

    let mut siv = cursive::default();

    // temp solution for allowing the movement of the current selected layer/view
    siv.add_global_callback('w', |s| move_top(s, 0, -1));
    siv.add_global_callback('a', |s| move_top(s, -1, 0));
    siv.add_global_callback('s', |s| move_top(s, 0, 1));
    siv.add_global_callback('d', |s| move_top(s, 1, 0));

    // save button 'k'
    siv.add_global_callback('k', |s| {
        s.call_on_name("calendar", |view: &mut CalendarView<Utc>| {

            let mut_storage_ref = view.storage.lock().unwrap();
            save_data(&mut_storage_ref.events);

        });
    });

    let theme = custom_theme_from_cursive(&siv);
    siv.set_theme(theme);

    siv.add_layer(create_panel(year, month, data));

    siv.set_autorefresh(true);

    siv.run();
}