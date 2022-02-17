#[derive(Debug, Default)]
struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    fn new(hours: i32, minutes: i32) -> Self {
        Self { hours, minutes }.normalize()
    }

    fn normalize(&mut self) -> Self {
        let mut hours = (self.hours + self.minutes / 60) % 24;
        let mut minutes = self.minutes % 60;

        if minutes < 0 {
            minutes += 60;
            hours -= 1;
        }

        if hours < 0 {
            hours += 24;
        }

        Self { hours, minutes }
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

impl std::ops::Add for Clock {
    type Output = Clock;

    fn add(self, rhs: Self) -> Self::Output {
        Clock::new(self.hours + rhs.hours, self.minutes + rhs.minutes)
    }
}

impl std::ops::Add<i32> for Clock {
    type Output = Clock;

    fn add(self, rhs: i32) -> Self::Output {
        Clock::new(self.hours, self.minutes + rhs)
    }
}

impl From<i32> for Clock {
    fn from(val: i32) -> Clock {
        Clock::new(0, val)
    }
}

fn main() {
    let clock = Clock::new(14, 55);
    let clock_2 = Clock::new(34, 155);
    let default_clock: Clock = Default::default();

    let clock = clock + clock_2;

    println!("{}", clock);

    let clock = default_clock + 1055;

    println!("{}", clock);

    let clock: Clock = 2055.into();

    println!("{}", clock);
}
