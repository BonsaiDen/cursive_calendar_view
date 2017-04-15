/// Enumeration of all weekdays.
#[derive(Copy, Clone)]
pub enum WeekDay {
    /// Monday.
    Monday,
    /// Tuesday.
    Tuesday,
    /// Wednesday.
    Wednesday,
    /// Thursday.
    Thursday,
    /// Friday.
    Friday,
    /// Saturday.
    Saturday,
    /// Sunday.
    Sunday
}


// Statics --------------------------------------------------------------------
static WEEK_DAY_LIST: [WeekDay; 7] = [
    WeekDay::Monday,
    WeekDay::Tuesday,
    WeekDay::Wednesday,
    WeekDay::Thursday,
    WeekDay::Friday,
    WeekDay::Saturday,
    WeekDay::Sunday
];


// Conversions ----------------------------------------------------------------
impl From<i32> for WeekDay {
    fn from(index: i32) -> Self {
        WEEK_DAY_LIST[((index + 7) % 7) as usize]
    }
}

impl Into<i32> for WeekDay {
    fn into(self) -> i32 {
        match self {
            WeekDay::Monday => 0,
            WeekDay::Tuesday => 1,
            WeekDay::Wednesday => 2,
            WeekDay::Thursday => 3,
            WeekDay::Friday => 4,
            WeekDay::Saturday => 5,
            WeekDay::Sunday => 6
        }
    }
}

