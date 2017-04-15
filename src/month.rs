/// Enumeration of all months in a year.
#[derive(Copy, Clone)]
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
    December
}

impl Month {

    #[doc(hidden)]
    pub fn prev(&self) -> Self {
        let index: i32 = self.into();
        MONTH_LIST[(((index - 1) + 12) % 12) as usize]
    }

    #[doc(hidden)]
    pub fn number_of_days(&self, year: i32) -> i32 {
        match *self {
            Month::February => {
                if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
                    29

                } else {
                    28
                }
            },
            Month::January | Month::March | Month::May | Month::July | Month::August | Month::October | Month::December => 31,
            Month::April | Month::June | Month::September | Month::November => 30
        }
    }

    #[doc(hidden)]
    pub fn prev_number_of_days(&self, year: i32) -> i32 {
        match *self {
            Month::January => self.prev().number_of_days(year - 1),
            _ => self.prev().number_of_days(year)
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
    Month::December
];


// Conversions ----------------------------------------------------------------
impl From<u32> for Month {
    fn from(index: u32) -> Self {
        MONTH_LIST[index as usize]
    }
}

impl<'a> Into<i32> for &'a Month {
    fn into(self) -> i32 {
        match *self {
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
            Month::December => 11
        }
    }
}

