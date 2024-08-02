// Crate Dependencies ---------------------------------------------------------

use cursive;

// STD Dependencies -----------------------------------------------------------
use std::sync::{Arc, Mutex};

// External Dependencies ------------------------------------------------------
use chrono::prelude::*;
use cursive::traits::*;
use cursive::views::{Dialog, TextView};
use cursive::Cursive;

// Modules --------------------------------------------------------------------
use cursive_calendar_view::{CalendarView, EnglishLocale, ViewMode};

// Example --------------------------------------------------------------------
fn main() {
    let mut siv = cursive::default();

    let stored_date: Arc<Mutex<Date<Utc>>> = Arc::new(Mutex::new(Utc.ymd(2020, 12, 31)));
    siv.add_layer(
        Dialog::around(TextView::new("-").with_name("text_box"))
            .button("Choose Date...", move |s| {
                let mut calendar =
                    CalendarView::<Utc, EnglishLocale>::new(*stored_date.lock().unwrap());

                //calendar.set_highest_view_mode(ViewMode::Year);
                calendar.set_view_mode(ViewMode::Year);
                calendar.set_earliest_date(Some(Utc.ymd(2020, 1, 1)));
                calendar.set_latest_date(Some(Utc.ymd(2040, 12, 31)));
                calendar.set_show_iso_weeks(true);

                let inner_date = stored_date.clone();
                calendar.set_on_submit(move |siv: &mut Cursive, date: &Date<Utc>| {
                    siv.call_on_name("text_box", |view: &mut TextView| {
                        *inner_date.lock().unwrap() = *date;
                        view.set_content(format!("{}", date));
                    });
                    siv.pop_layer();
                });

                s.add_layer(Dialog::around(calendar.with_name("calendar")).title("Select Date"));
            })
            .title("Calendar View Demo"),
    );

    siv.run();
}
