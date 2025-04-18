// External Dependencies ------------------------------------------------------
use chrono::prelude::*;
use cursive::traits::*;
use cursive::Vec2;
use cursive::Printer;
use cursive::theme::*;
use cursive::event::{Event, EventResult};


const TOP: &str=   "  .oooo.   #     .o    #  .oooo.   #   .oooo.  #      .o   #  oooooooo #   .ooo    # ooooooooo # .ooooo.   #  .ooooo.  #    ";
const M1: &str =   " d8P'`Y8b  #   o888    #.dP\"\"Y88b  #.dP\"\"Y88b  #    .d88   # dP\"\"\"\"\"\"\" #  .88'     #d\"\"\"\"\"\"\"8' #d88'   `8. #888' `Y88. #    ";
const M2: &str =r##"888    888 #    888    #      |8P' #      |8P' #  .d'888   #d88888b.   # d88'      #      .8'  #Y88..  .8' #888    888 #    "##  ;
const M3: &str =   "888    888 #    888    #    .d8P'  #    <88b.  #.d'  888   #    `Y88b  #d888P\"Ybo. #     .8'   # `88888b.  # `Vbood888 #    " ;
const M4: &str =   "888    888 #    888    #  .dP'     #     `88b. #88ooo888oo #      |88  #Y88|   |88 #    .8'    #.8'  ``88b #      888' #    ";
const M5: &str =   "`88b  d88' #    888    #.oP     .o #o.   .88P  #     888   #o.   .88P  #`Y88   88P #   .8'     #`8.   .88P #    .88P'  #    ";
const BOT: &str=   " `Y8bd8P'  #   o888o   #8888888888 #`8bd88P'   #    o888o  #`8bd88P'   # `88bod8'  #  .8'      # `boood8'  #  .oP'     #    ";

pub struct Clock{
    time: String
}

impl Clock {
    pub fn new() -> Self {
        Self {
            time: get_ascii_time(Local::now()),
        }
    }

    pub fn update_time(&mut self){
        self.time = get_ascii_time(Local::now());
    }
}

/// takes a local date time and converts it to a ascii character representation
pub fn get_ascii_time(local: DateTime<Local>) -> String{
    let top: Vec<&str> = TOP.split('#').collect();

    let m1: Vec<&str> = M1.split('#').collect();
    let m2: Vec<&str> = M2.split('#').collect();
    let m3: Vec<&str> = M3.split('#').collect();
    let m4: Vec<&str> = M4.split('#').collect();
    let m5: Vec<&str> = M5.split('#').collect();

    let bot: Vec<&str> = BOT.split('#').collect();

    // Get the current local time
    // let local: DateTime<Local> = Local::now();

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

        let time = get_ascii_time(Local::now());

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
        Vec2::new(78, 7)
    }
}
