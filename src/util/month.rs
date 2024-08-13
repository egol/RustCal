use chrono::prelude::*;


/// Enumeration of all months in a year.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Month {
    /// The month of January.
    January,
    /// The month of February.
    February,
    /// The month of March.
    March,
    /// The month of April.
    April,
    /// The month of May.
    May,
    /// The month of June.
    June,
    /// The month of July.
    July,
    /// The month of August.
    August,
    /// The month of September.
    September,
    /// The month of October.
    October,
    /// The month of November.
    November,
    /// The month of December.
    December,
}

impl Month {
    #[doc(hidden)]
    pub fn prev(self) -> Self {
        let index: i32 = self.into();
        MONTH_LIST[(((index - 1) + 12) % 12) as usize]
    }

    #[doc(hidden)]
    pub fn number_of_days(self, year: i32) -> i32 {
        match self {
            Month::February => {
                if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
                    29
                } else {
                    28
                }
            }
            Month::January
            | Month::March
            | Month::May
            | Month::July
            | Month::August
            | Month::October
            | Month::December => 31,
            Month::April | Month::June | Month::September | Month::November => 30,
        }
    }

    #[doc(hidden)]
    pub fn prev_number_of_days(self, year: i32) -> i32 {
        match self {
            Month::January => self.prev().number_of_days(year - 1),
            _ => self.prev().number_of_days(year),
        }
    }
}

// Statics --------------------------------------------------------------------
static MONTH_LIST: [Month; 12] = [
    Month::January,
    Month::February,
    Month::March,
    Month::April,
    Month::May,
    Month::June,
    Month::July,
    Month::August,
    Month::September,
    Month::October,
    Month::November,
    Month::December,
];

// Conversions ----------------------------------------------------------------
impl From<u32> for Month {
    fn from(index: u32) -> Self {
        MONTH_LIST[index as usize]
    }
}

impl<'a> Into<i32> for Month {
    fn into(self) -> i32 {
        match self {
            Month::January => 0,
            Month::February => 1,
            Month::March => 2,
            Month::April => 3,
            Month::May => 4,
            Month::June => 5,
            Month::July => 6,
            Month::August => 7,
            Month::September => 8,
            Month::October => 9,
            Month::November => 10,
            Month::December => 11,
        }
    }
}

pub fn month_to_string(num : i32) -> String{
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

/// Outputs a custom date fromat given a DateTime
/// 8/9/2024 becomes: Friday, 9th
pub fn format_task_date(date: DateTime<Local>) -> String {
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

pub fn abbr_month_to_string(num : i32) -> String{
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