use core::panic;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let minutes_new;
        let mut hours_new = 0;
        match minutes % 60 {
            x if x >= 0 => minutes_new = x,
            x if x < 0 => {
                minutes_new = 60 + x;
                hours_new = -1;
            }
            _ => panic!(),
        };
        match minutes / 60 {
            x if x >= 0 => hours_new += x,
            x if x < 0 => hours_new += 24 + x,
            _ => panic!(),
        }
        match (hours_new + hours) % 24 {
            x if x >= 0 => hours_new = x % 24,
            x if x < 0 => hours_new = (24 + x) % 24,
            _ => panic!(),
        }
        Clock {
            hours: hours_new,
            minutes: minutes_new,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let minutes_total = self.minutes + minutes;
        let minutes_new;
        let mut hours_new = 0;
        match minutes_total % 60 {
            x if x >= 0 => minutes_new = x,
            x if x < 0 => {
                minutes_new = 60 + x;
                hours_new = -1;
            }
            _ => panic!(),
        };
        match minutes_total / 60 {
            x if x >= 0 => hours_new += x,
            x if x < 0 => hours_new += 24 + x,
            _ => panic!(),
        }
        match (self.hours + hours_new) % 24 {
            x if x >= 0 => hours_new = x % 24,
            x if x < 0 => hours_new = (24 + x) % 24,
            _ => panic!(),
        }
        // let minutes_rem = minutes_diff % 60;
        // let hours_rem = minutes_diff / 60;

        Clock {
            hours: hours_new,
            minutes: minutes_new,
        }
    }
}

// impl Clock {
//     pub fn new(hours: i32, minutes: i32) -> Self {
//         Clock {
//             hours: (hours + minutes.div_euclid(60)).rem_euclid(24),
//             minutes: minutes.rem_euclid(60),
//         }
//     }
//     pub fn add_minutes(&self, minutes: i32) -> Self {
//         Clock::new(self.hours, self.minutes + minutes)
//     }
// }

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
