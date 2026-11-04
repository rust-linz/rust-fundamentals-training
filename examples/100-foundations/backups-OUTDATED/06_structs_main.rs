use std::fmt;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self { hours, minutes }.normalize()
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self {
            hours: self.hours,
            minutes: self.minutes + minutes,
        }
        .normalize()
    }

    fn normalize(&mut self) -> Self {
        let mut hours = (self.hours + self.minutes / 60) % 24;
        let mut minutes = self.minutes % 60;
        if minutes < 0 {
            minutes += 60;
            hours -= 1;
        }
        if hours < 0 {
            hours = (hours + 24) % 24;
        }
        Self { hours, minutes }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

fn main() {
    let c = Clock::new(24, 0);
    let d = Clock::new(0, 0);

    if c == d {
        println!("It's the same!");
    }
}
