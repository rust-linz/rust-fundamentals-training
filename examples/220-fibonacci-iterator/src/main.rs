use std::iter;

struct Fibonacci {
    curr: u128,
    next: u128,
}

// Implement iterator for our Fibonacci struct
impl Iterator for Fibonacci {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        // Note that checked_add returns None if the addition overflows.
        // That will end our iterator.
        let new_next = self.curr.checked_add(self.next)?;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

fn main() {
    // Use Fibonacci struct as an iterator
    let fibb = Fibonacci { curr: 0, next: 1 };
    for (idx, i) in fibb.enumerate() {
        println!("{}: {}", idx, i)
    }

    // Build iterator using from_fn and closure
    let mut fibb_state = (0u128, 1u128);
    let fibb = iter::from_fn(move || {
        let new_next = fibb_state.0.checked_add(fibb_state.1)?;
        fibb_state.0 = fibb_state.1;
        fibb_state.1 = new_next;
        Some(fibb_state.0)
    });
    for (idx, i) in fibb.enumerate() {
        println!("{}: {}", idx, i)
    }
}
