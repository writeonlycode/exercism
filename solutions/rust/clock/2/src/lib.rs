use std::fmt::Display;

use time::*;

#[derive(Debug, PartialEq)]
pub struct Clock {
    time: Time,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut minutes = minutes;
        let mut hours = hours;

        while minutes < 0 {
            hours -= 1;
            minutes = 60 + minutes;
        }

        while minutes >= 60 {
            hours += 1;
            minutes = minutes - 60;
        }

        while hours < 0 {
            hours = 24 + hours;
        }

        while hours >= 24 {
            hours = hours - 24;
        }

        Clock {
            time: Time::from_hms(
                (hours % 24).try_into().unwrap(),
                (minutes % 60).try_into().unwrap(),
                0,
            )
            .unwrap(),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            time: self.time + Duration::new((minutes * 60).into(), 0),
        }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.time.hour(), self.time.minute())
    }
}
