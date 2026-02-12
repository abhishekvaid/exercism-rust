#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes_since_midnight: u16,
}

const MINUTES_IN_DAY: i32 = 24 * 60;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            minutes_since_midnight: (hours * 60 + minutes).rem_euclid(MINUTES_IN_DAY) as u16,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0, self.minutes_since_midnight as i32 + minutes)
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02}:{:02}",
            self.minutes_since_midnight / 60,
            self.minutes_since_midnight % 60
        )
    }
}
