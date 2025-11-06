use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let base_clock = Self {
            hours: 0,
            minutes: 0,
        };
        let minutes_to_add = (hours * 60) + minutes;
        base_clock.add_minutes(minutes_to_add)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let total_minutes = (self.hours * 60) + self.minutes + minutes;

        let new_minutes = total_minutes.rem_euclid(60);
        let new_hours = (total_minutes.div_euclid(60)).rem_euclid(24);

        Self {
            hours: new_hours,
            minutes: new_minutes,
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
