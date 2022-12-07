pub struct Fibonacci {
    curr: u128,
    next: u128,
}

impl Iterator for Fibonacci {
    type Item = u128; // Associated Type

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr.checked_add(self.next)?;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

impl Default for Fibonacci {
    fn default() -> Self {
        Self { curr: 0, next: 1 }
    }
}
