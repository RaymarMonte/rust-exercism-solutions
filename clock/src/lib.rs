use std::fmt;

#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32, minutes: i32,
}

impl Clock {

    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock::get_normalized_value(hours, minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::get_normalized_value(self.hours, self.minutes + minutes)
    }

    pub fn get_normalized_value(mut hours: i32, mut minutes: i32) -> Self {
        hours += minutes.div_euclid(60);
        hours = hours.rem_euclid(24);
        minutes = minutes.rem_euclid(60);
        Self {hours: hours, minutes: minutes}
    }

}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}