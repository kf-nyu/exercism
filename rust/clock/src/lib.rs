use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}   
impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {

        let total_minutes = (hours * 60 + minutes).rem_euclid(24 * 60);

        Self { hours: total_minutes / 60, minutes: total_minutes % 60 }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        
        Self::new(self.hours, self.minutes + minutes)
    }
}
