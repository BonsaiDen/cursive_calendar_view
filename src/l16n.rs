// Internal Dependencies ------------------------------------------------------
use ::{Month, WeekDay};


/// Trait for localization of a [`CalendarView`](struct.CalendarView.html).
pub trait Locale {

    /// Method returning the localized string for a specific [`WeekDay`](enum.WeekDay.html).
    ///
    /// Both *short* e.g. `Th` and *long* translations e.g. `Thursday` are suppported.
    fn week_day(day: WeekDay, long_text: bool) -> &'static str;

    /// Method returning the localized string for a specific [`Month`](enum.Month.html).
    ///
    /// Both *short* e.g. `Dec` and *long* translations e.g. `December` are suppported.
    fn month(month: Month, long_text: bool) -> &'static str;

}


/// English locale for a [`CalendarView`](struct.CalendarView.html).
pub struct EnglishLocale;

impl Locale for EnglishLocale {

    fn week_day(day: WeekDay, long_text: bool) -> &'static str {
        if long_text {
            match day {
                WeekDay::Monday => "Monday",
                WeekDay::Tuesday => "Tuesday",
                WeekDay::Wednesday => "Wednesday",
                WeekDay::Thursday => "Thursday",
                WeekDay::Friday => "Friday",
                WeekDay::Saturday => "Saturday",
                WeekDay::Sunday => "Sunday"
            }
        } else {
            match day {
                WeekDay::Monday => "Mo",
                WeekDay::Tuesday => "Tu",
                WeekDay::Wednesday => "We",
                WeekDay::Thursday => "Th",
                WeekDay::Friday => "Fr",
                WeekDay::Saturday => "Sa",
                WeekDay::Sunday => "Su"
            }
        }
    }

    fn month(month: Month, long_text: bool) -> &'static str {
        if long_text {
            match month {
                Month::January => "January",
                Month::February => "February",
                Month::March => "March",
                Month::April => "April",
                Month::May => "May",
                Month::June => "June",
                Month::July => "July",
                Month::August => "August",
                Month::September => "September",
                Month::October => "October",
                Month::November => "November",
                Month::December => "December"
            }
        } else {
            match month {
                Month::January => "Jan",
                Month::February => "Feb",
                Month::March => "Mar",
                Month::April => "Apr",
                Month::May => "May",
                Month::June => "Jun",
                Month::July => "Jul",
                Month::August => "Aug",
                Month::September => "Sep",
                Month::October => "Oct",
                Month::November => "Nov",
                Month::December => "Dec"
            }
        }

    }

}

