struct Fibonacci {
    curr: u128,
    next: u128,
}

impl Iterator for Fibonacci {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr.checked_add(self.next)?;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

fn main() {
    let fibb = Fibonacci { curr: 0, next: 1 };

    for (idx, i) in fibb.enumerate() {
        println!("{}: {}", idx, i)
    }
}
