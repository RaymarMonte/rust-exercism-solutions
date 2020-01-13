use std::fmt;

pub struct Clock {
    hours: i32, minutes: i32,
}

impl Clock {

    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {hours: hours, minutes: minutes}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self {hours: self.hours, minutes: self.minutes + minutes}
    }

    pub fn get_normalized_value(&self) -> Self {
        let mut hours = self.hours;
        let mut minutes = self.minutes;
        hours += minutes.div_euclid(60);
        hours = hours.rem_euclid(24);
        minutes = minutes.rem_euclid(60);
        Self {hours: hours, minutes: minutes}
    }

}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let normalized = self.get_normalized_value();
        write!(f, "{:02}:{:02}", normalized.hours, normalized.minutes)
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let normalized = self.get_normalized_value();
        write!(f, "Clock {{ hours: {:02}, minutes: {:02} }}", normalized.hours, normalized.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        let normalized_self = self.get_normalized_value();
        let normalized_other = other.get_normalized_value();
        (normalized_self.hours == normalized_other.hours) && (normalized_self.minutes == normalized_other.minutes)
    }
}