use std::fmt::Display;
use chrono::{Datelike, Timelike, Utc};


pub enum MONTH {
    JANUARY = 1,
    FEBRUARY = 2,
    MARCH = 3,
    APRIL = 4,
    MAY = 5,
    JUNE = 6,
    JULY = 7,
    AUGUST = 8,
    SEPTEMBER = 9,
    OCTOBER = 10,
    NOVEMBER = 11,
    DECEMBER = 12
}

impl Display for MONTH {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MONTH::JANUARY => { write!(f, "Jan") },
            MONTH::FEBRUARY => { write!(f, "Feb") },
            MONTH::MARCH => { write!(f, "Mar") },
            MONTH::APRIL => { write!(f, "Apr") },
            MONTH::MAY => { write!(f, "May") },
            MONTH::JUNE => { write!(f, "Jun") },
            MONTH::JULY => { write!(f, "Jul") },
            MONTH::AUGUST => { write!(f, "Aug") },
            MONTH::SEPTEMBER => { write!(f, "Sep") },
            MONTH::OCTOBER => { write!(f, "Oct") },
            MONTH::NOVEMBER => { write!(f, "Nov") },
            MONTH::DECEMBER => { write!(f, "Dec") },
        }
    }
}




// To follow RFC 7231 Section 7.1.1.1
// https://datatracker.ietf.org/doc/html/rfc7231#section-7.1.1.1
pub struct HttpDate {
    day_of_week: chrono::Weekday,
    day_of_month: u32,
    month: MONTH,
    year: i32,
    hour: u32,
    minute: u32,
    second: u32
}

impl HttpDate {
    pub fn get_current() -> HttpDate {
        let now = Utc::now();
        
        HttpDate { 
            day_of_week: now.weekday(),
            day_of_month: now.month(),
            month: MONTH::APRIL,
            year: now.year(),
            hour: now.hour(),
            minute: now.minute(),
            second: now.second()
        }
    }
}


impl Display for HttpDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Format: `Tue, 15 Nov 1994 08:12:31 GMT`
        write!(f, 
            "{}, {:02} {} {:04} {:02}:{:02}:{:02} GMT",
            self.day_of_week,
            self.day_of_month, 
            self.month,
            self.year,
            self.hour,
            self.minute,
            self.second
        )
    }
}