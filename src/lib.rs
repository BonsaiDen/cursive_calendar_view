//! A calendar implementation for [cursive](https://crates.io/crates/cursive).
#![deny(
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications
)]

// Crate Dependencies ---------------------------------------------------------
extern crate chrono;
extern crate cursive;

// STD Dependencies -----------------------------------------------------------
use std::cmp;
use std::marker::PhantomData;
use std::rc::Rc;

// External Dependencies ------------------------------------------------------
use chrono::offset::TimeZone;
use chrono::prelude::*;

use cursive::direction::Direction;
use cursive::event::{Callback, Event, EventResult, Key};
use cursive::theme::ColorStyle;
use cursive::vec::Vec2;
use cursive::view::View;
use cursive::With;
use cursive::{Cursive, Printer};

// Modules --------------------------------------------------------------------
mod l16n;
mod month;
mod week_day;

// Re-Exports -----------------------------------------------------------------
pub use l16n::{EnglishLocale, Locale};
pub use month::Month;
pub use week_day::WeekDay;

/// Enumeration of all view modes supported by a [`CalendarView`](struct.CalendarView.html).
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub enum ViewMode {
    /// View of a specific month, allowing selection of individual days.
    Month,
    /// View of a specific year, allowing selection of individual months.
    Year,
    /// View of a specific decade, allowing selection of individual years.
    Decade,
}

/// A callback taking a date as parameter.
///
/// This is an internal type used to improve readability.
type DateCallback<T> = Rc<Fn(&mut Cursive, &Date<T>)>;

/// View for selecting a date, supporting different modes for day, month or
/// year based selection.
///
/// View modes can be navigated via `Backspace` and `Enter`.
///
/// Custom localization is possible by providing an implementation of the
/// [`Locale`](trait.Locale.html) trait.
///
/// # Examples
///
/// ```
/// # extern crate cursive;
/// # extern crate cursive_calendar_view;
/// # extern crate chrono;
/// # use chrono::prelude::*;
/// # use cursive_calendar_view::{CalendarView, EnglishLocale, ViewMode};
/// # fn main() {
/// // Allow selection a date within the year of 2017.
/// let mut calendar = CalendarView::<Utc, EnglishLocale>::new(Utc::today());
///
/// calendar.set_highest_view_mode(ViewMode::Year);
/// calendar.set_earliest_date(Some(Utc.ymd(2017, 1, 1)));
/// calendar.set_latest_date(Some(Utc.ymd(2017, 12, 31)));
/// calendar.set_show_iso_weeks(true);
/// # }
/// ```
pub struct CalendarView<T: TimeZone, L: Locale> {
    enabled: bool,
    show_iso_weeks: bool,
    week_start: WeekDay,

    highest_view_mode: ViewMode,
    lowest_view_mode: ViewMode,

    view_mode: ViewMode,
    view_date: Date<T>,

    earliest_date: Option<Date<T>>,
    latest_date: Option<Date<T>>,
    date: Date<T>,
    on_submit: Option<DateCallback<T>>,
    on_select: Option<DateCallback<T>>,

    size: Vec2,

    _localization: PhantomData<L>,
}

impl<T: TimeZone, L: Locale + 'static> CalendarView<T, L> {
    /// Creates new `CalendarView`.
    pub fn new(today: Date<T>) -> Self {
        Self {
            enabled: true,
            highest_view_mode: ViewMode::Decade,
            lowest_view_mode: ViewMode::Month,
            show_iso_weeks: false,
            week_start: WeekDay::Monday,
            date: today.clone(),
            earliest_date: None,
            latest_date: None,
            view_mode: ViewMode::Month,
            view_date: today,
            size: (0, 0).into(),
            on_submit: None,
            on_select: None,
            _localization: PhantomData,
        }
    }

    /// Disables this view.
    ///
    /// A disabled view cannot be selected.
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Re-enables this view.
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Enable or disable this view.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Returns `true` if this view is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Returns the currently selected date of this view.
    pub fn date(&self) -> Date<T> {
        self.date.clone()
    }

    /// Sets the currently selected date of this view.
    pub fn set_selected_date(&mut self, mut date: Date<T>) {
        if let Some(ref earliest) = self.earliest_date {
            if date < *earliest {
                date = earliest.clone();
            }
        }

        if let Some(ref latest) = self.latest_date {
            if date > *latest {
                date = latest.clone();
            }
        }

        self.date = date;
    }

    /// Sets the currently selected date of this view.
    ///
    /// Chainable variant.
    pub fn selected_date(self, date: Date<T>) -> Self {
        self.with(|v| v.set_selected_date(date))
    }

    /// Sets the visually selected date of this view.
    pub fn set_view_date(&mut self, mut date: Date<T>) {
        if let Some(ref earliest) = self.earliest_date {
            if date < *earliest {
                date = earliest.clone();
            }
        }

        if let Some(ref latest) = self.latest_date {
            if date > *latest {
                date = latest.clone();
            }
        }

        self.view_date = date;
    }

    /// Sets the visually selected date of this view.
    ///
    /// Chainable variant.
    pub fn view_date(self, date: Date<T>) -> Self {
        self.with(|v| v.set_view_date(date))
    }

    /// Sets the currently active view mode of this view.
    pub fn set_view_mode(&mut self, mode: ViewMode) {
        if mode >= self.lowest_view_mode && mode <= self.highest_view_mode {
            self.view_mode = mode;
        }
    }

    /// Sets the currently active view mode of this view.
    ///
    /// Chainable variant.
    pub fn view_mode(self, mode: ViewMode) -> Self {
        self.with(|v| v.set_view_mode(mode))
    }

    /// Sets the lowest view mode this calendar can be in.
    ///
    /// Can be used conjunction with
    /// [`CalendarView::set_highest_view_mode`](struct.CalendarView.html#method.set_highest_view_mode)
    /// to limit a `CalendarView` to only allow selection of days, months or years.
    pub fn set_lowest_view_mode(&mut self, mode: ViewMode) {
        if mode < self.highest_view_mode {
            self.lowest_view_mode = mode;
            if self.view_mode < self.lowest_view_mode {
                self.view_mode = self.lowest_view_mode;
            }
        }
    }

    /// Sets the lowest view mode this calendar can be in.
    ///
    /// Can be used conjunction with
    /// [`CalendarView::set_highest_view_mode`](struct.CalendarView.html#method.set_highest_view_mode)
    /// to limit a `CalendarView` to only allow selection of days, months or years.
    ///
    /// Chainable variant.
    pub fn lowest_view_mode(self, mode: ViewMode) -> Self {
        self.with(|v| v.set_lowest_view_mode(mode))
    }

    /// Sets the highest view mode this calendar can be in.
    ///
    /// Can be used conjunction with
    /// [`CalendarView::set_lowest_view_mode`](struct.CalendarView.html#method.set_lowest_view_mode)
    /// to limit a `CalendarView` to only allow selection of days, months or years.
    pub fn set_highest_view_mode(&mut self, mode: ViewMode) {
        if mode > self.lowest_view_mode {
            self.highest_view_mode = mode;
            if self.view_mode > self.highest_view_mode {
                self.view_mode = self.highest_view_mode;
            }
        }
    }

    /// Sets the highest view mode this calendar can be in.
    ///
    /// Can be used conjunction with
    /// [`CalendarView::set_lowest_view_mode`](struct.CalendarView.html#method.set_lowest_view_mode)
    /// to limit a `CalendarView` to only allow selection of days, months or years.
    ///
    /// Chainable variant.
    pub fn highest_view_mode(self, mode: ViewMode) -> Self {
        self.with(|v| v.set_highest_view_mode(mode))
    }

    /// Sets and limits the earliest date selectable by this view.
    pub fn set_earliest_date(&mut self, date: Option<Date<T>>) {
        self.earliest_date = date;

        if let Some(ref date) = self.earliest_date {
            if self.date < *date {
                self.date = date.clone();
            }
        }
    }

    /// Sets and limits the earliest date selectable by this view.
    ///
    /// Chainable variant.
    pub fn earliest_date(self, date: Option<Date<T>>) -> Self {
        self.with(|v| v.set_earliest_date(date))
    }

    /// Sets and limits the latest date selectable by this view.
    pub fn set_latest_date(&mut self, date: Option<Date<T>>) {
        self.latest_date = date;

        if let Some(ref date) = self.latest_date {
            if self.date > *date {
                self.date = date.clone();
            }
        }
    }

    /// Sets and limits the latest date selectable by this view.
    ///
    /// Chainable variant.
    pub fn latest_date(self, date: Option<Date<T>>) -> Self {
        self.with(|v| v.set_latest_date(date))
    }

    /// Allows to change the default week start day of `WeekDay::Monday` to any other
    /// [`WeekDay`](struct.WeekDay.html).
    pub fn set_week_start(&mut self, day: WeekDay) {
        self.week_start = day;
    }

    /// Allows to change the default week start day of `WeekDay::Monday` to any other
    /// [`WeekDay`](struct.WeekDay.html).
    ///
    /// Chainable variant.
    pub fn week_start(self, day: WeekDay) -> Self {
        self.with(|v| v.set_week_start(day))
    }

    /// Show or hide ISO week numbers in the `ViewMode::Month` view mode.
    ///
    /// ISO week numbers only make sense with a week start day of `WeekDay::Monday`.
    pub fn set_show_iso_weeks(&mut self, show: bool) {
        self.show_iso_weeks = show;
    }

    /// Show or hide ISO week numbers in the `ViewMode::Month` view mode.
    ///
    /// ISO week numbers only make sense with a week start day of `WeekDay::Monday`.
    ///
    /// Chainable variant.
    pub fn show_iso_weeks(self, show: bool) -> Self {
        self.with(|v| v.set_show_iso_weeks(show))
    }

    /// Sets a callback to be used when `<Enter>` is pressed to select a date.
    pub fn set_on_submit<F>(&mut self, cb: F)
    where
        F: Fn(&mut Cursive, &Date<T>) + 'static,
    {
        self.on_submit = Some(Rc::new(move |s, date| cb(s, date)));
    }

    /// Sets a callback to be used when `<Enter>` is pressed to select a date.
    ///
    /// Chainable variant.
    pub fn on_submit<F>(self, cb: F) -> Self
    where
        F: Fn(&mut Cursive, &Date<T>) + 'static,
    {
        self.with(|v| v.set_on_submit(cb))
    }

    /// Sets a callback to be used when an a new date is visually selected.
    pub fn set_on_select<F>(&mut self, cb: F)
    where
        F: Fn(&mut Cursive, &Date<T>) + 'static,
    {
        self.on_select = Some(Rc::new(move |s, date| cb(s, date)));
    }

    /// Sets a callback to be used when an a new date is visually selected.
    ///
    /// Chainable variant.
    pub fn on_select<F>(self, cb: F) -> Self
    where
        F: Fn(&mut Cursive, &Date<T>) + 'static,
    {
        self.with(|v| v.set_on_select(cb))
    }
}

impl<T: TimeZone, L: Locale + 'static> CalendarView<T, L> {
    fn draw_month(&self, printer: &Printer) {
        let year = self.view_date.year();
        let month: Month = self.view_date.month0().into();
        let month_start = self.view_date.with_day0(0).unwrap();

        let active_day = self.date.day0() as i32;
        let view_day = self.view_date.day0() as i32;

        let d_month = self.date.month0() as i32 - self.view_date.month0() as i32;
        let d_year = self.date.year() - year;

        let month_days = month.number_of_days(year);
        let prev_month_days = month.prev_number_of_days(year);
        let first_week_day: WeekDay = (month_start.weekday() as i32).into();

        // Draw Month Name
        printer.print(
            (0, 0),
            &format!(
                "{:^width$}",
                format!("{} {}", L::month(month, true), year),
                width = self.size.x
            ),
        );

        // Draw Weekdays
        let h_offset = if self.show_iso_weeks { 3 } else { 0 };
        let w_offset: i32 = self.week_start.into();
        for i in 0..7 {
            let week_day: WeekDay = (i + w_offset).into();
            printer.print((h_offset + i * 3, 1), L::week_day(week_day, false));
        }

        // Draw days
        let d_shift = ((WeekDay::Monday as i32 - w_offset) + 7) % 7;
        let d_offset = ((first_week_day as i32) + d_shift) % 7;

        for (index, i) in (-d_offset..-d_offset + 42).enumerate() {
            let (day_number, month_offset) = if i < 0 {
                (prev_month_days + i, -1)
            } else if i > month_days - 1 {
                (i - month_days, 1)
            } else {
                (i, 0)
            };

            if let Some(exact_date) =
                date_from_day_and_offsets(&self.view_date, Some(day_number), 0, month_offset, 0)
            {
                let color = if !self.date_available(&exact_date) {
                    ColorStyle::tertiary()
                } else if i < 0 {
                    if active_day == prev_month_days + i && d_month == -1 && d_year == 0 {
                        if self.enabled && printer.focused {
                            ColorStyle::highlight_inactive()
                        } else {
                            ColorStyle::secondary()
                        }
                    } else {
                        ColorStyle::secondary()
                    }
                } else if i > month_days - 1 {
                    if active_day == i - month_days && d_month == 1 && d_year == 0 {
                        if self.enabled && printer.focused {
                            ColorStyle::highlight_inactive()
                        } else {
                            ColorStyle::secondary()
                        }
                    } else {
                        ColorStyle::secondary()
                    }
                } else if view_day == i {
                    if self.enabled && printer.focused {
                        ColorStyle::highlight()
                    } else {
                        ColorStyle::highlight_inactive()
                    }
                } else if active_day == i && d_month == 0 && d_year == 0 {
                    if self.enabled {
                        ColorStyle::highlight_inactive()
                    } else {
                        ColorStyle::primary()
                    }
                } else {
                    ColorStyle::primary()
                };

                // Draw day number
                let (x, y) = (h_offset + (index as i32 % 7) * 3, 2 + (index as i32 / 7));
                printer.with_color(color, |printer| {
                    printer.print((x, y), &format!("{:>2}", day_number + 1));
                });

                // Draw ISO Weeks (Only makes sense when start_of_week is Monday)
                if self.show_iso_weeks && index as i32 % 7 == 0 {
                    let iso_week = exact_date.iso_week().week();
                    printer.with_color(ColorStyle::title_secondary(), |printer| {
                        printer.print((0, y), &format!("{:>2}", iso_week));
                    });
                }
            }
        }
    }

    fn draw_year(&self, printer: &Printer) {
        let active_month = self.date.month0();
        let view_month = self.view_date.month0();
        let year = self.view_date.year();
        let d_year = self.date.year() - year;

        // Draw Year
        printer.print(
            (0, 0),
            &format!("{:^width$}", format!("{}", year), width = self.size.x),
        );

        // Draw Month Names
        let h_offset = if self.show_iso_weeks { 2 } else { 0 };
        for i in 0..12 {
            let color = if !self.month_available(i, year) {
                ColorStyle::tertiary()
            } else if view_month == i {
                if self.enabled && printer.focused {
                    ColorStyle::highlight()
                } else {
                    ColorStyle::highlight_inactive()
                }
            } else if active_month == i && d_year == 0 {
                if self.enabled && printer.focused {
                    ColorStyle::highlight_inactive()
                } else {
                    ColorStyle::primary()
                }
            } else {
                ColorStyle::primary()
            };

            let (x, y) = (h_offset + (i as i32 % 4) * 5, 2 + (i as i32 / 4) * 2);
            printer.with_color(color, |printer| {
                printer.print((x, y), &format!("{:>4}", L::month(i.into(), false)));
            });
        }
    }

    fn draw_decade(&self, printer: &Printer) {
        let active_year = self.date.year();
        let view_year = self.view_date.year();
        let decade = view_year - (view_year % 10);

        // Draw Year Range
        printer.print(
            (0, 0),
            &format!(
                "{:^width$}",
                format!("{} - {}", decade, decade + 9),
                width = self.size.x
            ),
        );

        // Draw Years
        let h_offset = if self.show_iso_weeks { 2 } else { 0 };
        for (index, i) in (-1..12).enumerate() {
            let year = decade + i;
            let color = if !self.year_available(year) {
                ColorStyle::tertiary()
            } else if i < 0 || i > 9 {
                if active_year == year {
                    if self.enabled && printer.focused {
                        ColorStyle::highlight_inactive()
                    } else {
                        ColorStyle::secondary()
                    }
                } else {
                    ColorStyle::secondary()
                }
            } else if view_year == year {
                if self.enabled && printer.focused {
                    ColorStyle::highlight()
                } else {
                    ColorStyle::highlight_inactive()
                }
            } else if active_year == year {
                if self.enabled {
                    ColorStyle::highlight_inactive()
                } else {
                    ColorStyle::primary()
                }
            } else {
                ColorStyle::primary()
            };

            let (x, y) = (
                h_offset + (index as i32 % 4) * 5,
                2 + (index as i32 / 4) * 2,
            );

            printer.with_color(color, |printer| {
                printer.print((x, y), &format!("{:>4}", year));
            });
        }
    }

    fn date_available(&self, date: &Date<T>) -> bool {
        if let Some(ref earliest) = self.earliest_date {
            if *date < *earliest {
                return false;
            }
        }

        if let Some(ref latest) = self.latest_date {
            if *date > *latest {
                return false;
            }
        }

        true
    }

    fn month_available(&self, month: u32, year: i32) -> bool {
        if !self.year_available(year) {
            return false;
        }

        if let Some(ref earliest) = self.earliest_date {
            if year == earliest.year() && month < earliest.month0() {
                return false;
            }
        }

        if let Some(ref latest) = self.latest_date {
            if year == latest.year() && month > latest.month0() {
                return false;
            }
        }

        true
    }

    fn year_available(&self, year: i32) -> bool {
        if let Some(ref earliest) = self.earliest_date {
            if year < earliest.year() {
                return false;
            }
        }

        if let Some(ref latest) = self.latest_date {
            if year > latest.year() {
                return false;
            }
        }

        true
    }
}

impl<T: TimeZone + 'static, L: Locale + 'static> View for CalendarView<T, L> {
    fn draw(&self, printer: &Printer) {
        match self.view_mode {
            ViewMode::Month => self.draw_month(printer),
            ViewMode::Year => self.draw_year(printer),
            ViewMode::Decade => self.draw_decade(printer),
        }
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        self.size = if self.show_iso_weeks {
            (23, 8).into()
        } else {
            (20, 8).into()
        };
        self.size
    }

    fn take_focus(&mut self, _: Direction) -> bool {
        self.enabled
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        if !self.enabled {
            return EventResult::Ignored;
        }

        let last_view_date = self.view_date.clone();
        let offsets = match event {
            Event::Key(Key::Up) => Some(match self.view_mode {
                ViewMode::Month => (-7, 0, 0),
                ViewMode::Year => (0, -4, 0),
                ViewMode::Decade => (0, 0, -4),
            }),
            Event::Key(Key::Down) => Some(match self.view_mode {
                ViewMode::Month => (7, 0, 0),
                ViewMode::Year => (0, 4, 0),
                ViewMode::Decade => (0, 0, 4),
            }),
            Event::Key(Key::Right) => Some(match self.view_mode {
                ViewMode::Month => (1, 0, 0),
                ViewMode::Year => (0, 1, 0),
                ViewMode::Decade => (0, 0, 1),
            }),
            Event::Key(Key::Left) => Some(match self.view_mode {
                ViewMode::Month => (-1, 0, 0),
                ViewMode::Year => (0, -1, 0),
                ViewMode::Decade => (0, 0, -1),
            }),
            Event::Key(Key::PageUp) => Some(match self.view_mode {
                ViewMode::Month => (0, 1, 0),
                ViewMode::Year => (0, 0, 1),
                ViewMode::Decade => (0, 0, 10),
            }),
            Event::Key(Key::PageDown) => Some(match self.view_mode {
                ViewMode::Month => (0, -1, 0),
                ViewMode::Year => (0, 0, -1),
                ViewMode::Decade => (0, 0, -10),
            }),
            Event::Key(Key::Backspace) => {
                if self.view_mode < self.highest_view_mode {
                    self.view_mode = match self.view_mode {
                        ViewMode::Month => ViewMode::Year,
                        ViewMode::Year | ViewMode::Decade => ViewMode::Decade,
                    };
                }
                None
            }
            Event::Key(Key::Enter) => {
                if self.view_mode == self.lowest_view_mode {
                    self.date = self.view_date.clone();

                    if self.on_submit.is_some() {
                        let cb = self.on_submit.clone().unwrap();
                        let date = self.date.clone();
                        return EventResult::Consumed(Some(Callback::from_fn(move |s| {
                            cb(s, &date)
                        })));
                    }
                } else {
                    self.view_mode = match self.view_mode {
                        ViewMode::Month | ViewMode::Year => ViewMode::Month,
                        ViewMode::Decade => ViewMode::Year,
                    };
                }
                None
            }
            _ => return EventResult::Ignored,
        };

        if let Some((day, month, year)) = offsets {
            if let Some(date) = date_from_day_and_offsets(&last_view_date, None, day, month, year) {
                self.set_view_date(date);
            }
        }

        if self.view_date != last_view_date {
            let date = self.view_date.clone();
            EventResult::Consumed(
                self.on_select
                    .clone()
                    .map(|cb| Callback::from_fn(move |s| cb(s, &date))),
            )
        } else {
            EventResult::Consumed(None)
        }
    }
}

// Helpers --------------------------------------------------------------------
fn date_from_day_and_offsets<T: TimeZone>(
    date: &Date<T>,
    set_day: Option<i32>,
    day_offset: i32,
    month_offset: i32,
    year_offset: i32,
) -> Option<Date<T>> {
    let mut year = date.year() + year_offset;
    let mut month = date.month0() as i32;

    month += month_offset;

    if month < 0 {
        year -= 1;
        month += 12;
    } else if month > 11 {
        year += 1;
        month -= 12;
    }

    date.with_day0(0).unwrap().with_year(year).and_then(|d| {
        d.with_month0(month as u32).and_then(|d| {
            let month: Month = d.month0().into();
            let number_of_days = month.number_of_days(year);

            let mut day =
                set_day.unwrap_or_else(|| cmp::min(number_of_days - 1, date.day0() as i32));

            if day_offset < 0 {
                day += day_offset;
                if day < 0 {
                    day += month.prev_number_of_days(year);
                    return date_from_day_and_offsets(&d, Some(day), 0, -1, 0);
                }
            } else if day_offset > 0 {
                day += day_offset;
                if day > number_of_days - 1 {
                    day -= number_of_days;
                    return date_from_day_and_offsets(&d, Some(day), 0, 1, 0);
                }
            }

            d.with_day0(day as u32)
        })
    })
}
