use core::fmt;

struct MaintenanceHours {
    hours: f32,
    rate: f32,
}

pub trait Billable {
    fn bill(&self) -> f32 {
        0f32
    }
}

impl Billable for MaintenanceHours {
    fn bill(&self) -> f32 {
        self.hours * self.rate
    }
}

fn print_bill(bill: &impl Billable) {
    println!("{}", bill.bill());
}

fn main() {
    let hours = MaintenanceHours {
        hours: 30f32,
        rate: 200f32,
    };
    print_bill(&hours);
}
