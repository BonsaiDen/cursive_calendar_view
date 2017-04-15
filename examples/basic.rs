// Crate Dependencies ---------------------------------------------------------
extern crate cursive;
extern crate chrono;
extern crate cursive_calendar_view;


// STD Dependencies -----------------------------------------------------------
use std::rc::Rc;
use std::cell::RefCell;


// External Dependencies ------------------------------------------------------
use chrono::prelude::*;
use cursive::Cursive;
use cursive::traits::*;
use cursive::views::{Dialog, TextView};


// Modules --------------------------------------------------------------------
use cursive_calendar_view::{CalendarView, EnglishLocale, ViewMode};


// Example --------------------------------------------------------------------
fn main() {

    let mut siv = Cursive::new();

    let stored_date: Rc<RefCell<Date<UTC>>> = Rc::new(RefCell::new(UTC.ymd(2017, 12, 31)));
    siv.add_layer(
        Dialog::around(TextView::new("-").with_id("text_box")).button("Choose Date...", move |s| {

            let mut calendar = CalendarView::<UTC, EnglishLocale>::new(
                *stored_date.borrow()
            );

            calendar.set_highest_view_mode(ViewMode::Year);
            calendar.set_view_mode(ViewMode::Year);
            calendar.set_earliest_date(Some(UTC.ymd(2017, 1, 1)));
            calendar.set_latest_date(Some(UTC.ymd(2017, 12, 31)));
            calendar.set_show_iso_weeks(true);

            let inner_date = stored_date.clone();
            calendar.set_on_submit(move |siv: &mut Cursive, date: &Date<UTC>| {
                siv.call_on_id("text_box", |view: &mut TextView| {
                    *inner_date.borrow_mut() = *date;
                    view.set_content(format!("{}", date));
                });
                siv.pop_layer();
            });

            s.add_layer(
                Dialog::around(calendar.with_id("calendar")).title("Select Date")
            );

        }).title("Calendar View Demo")
    );

    siv.run();

}

