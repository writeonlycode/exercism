use std::fmt::Display;

use time::*;

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
}

const MINUTES_IN_HOUR: i32 = 60;
const HOURS_IN_DAY: i32 = 24;
const MINUTES_IN_DAY: i32 = HOURS_IN_DAY * MINUTES_IN_HOUR;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let hours = hours * MINUTES_IN_HOUR;
        let minutes = (MINUTES_IN_DAY + ((minutes + hours) % MINUTES_IN_DAY)) % MINUTES_IN_DAY;

        Clock { minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let minutes = self.minutes + minutes;

        let hours = minutes / MINUTES_IN_HOUR;
        let minutes = minutes % MINUTES_IN_HOUR;

        Clock::new(hours, minutes)
    }
}
impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hours = self.minutes / 60;
        let minutes = self.minutes % 60;

        write!(f, "{:02}:{:02}", hours, minutes)
    }
}
