use std::{
    sync::{Arc, Mutex},
    thread,
};

use rand::prelude::*;

#[derive(Default, Debug)]
struct Lotto {
    numbers: Vec<usize>,
}

impl Lotto {
    pub fn new(amount: usize, max: usize) -> Self {
        let pot = 1..=max;
        let mut rng = rand::rng();
        Self {
            numbers: pot.choose_multiple(&mut rng, amount),
        }
    }
}

impl IntoIterator for Lotto {
    type Item = <Vec<usize> as IntoIterator>::Item;
    type IntoIter = <Vec<usize> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.numbers.into_iter()
    }
}

fn main() {
    let mut handles = Vec::new();
    let mutex = Arc::new(Mutex::new(3));

    let pairs = [(6, 45), (5, 50), (2, 12)];
    for (take, from) in pairs {
        let mutex = mutex.clone();
        let handle = thread::spawn(move || {
            match mutex.lock() {
                Ok(res) => res.to_owned(),
                Err(_) => 0,
            };
            if take == 5 {
                panic!("at the disco");
            }
            Lotto::new(take, from)
        });
        handles.push(handle);
    }

    for handle in handles {
        if let Ok(result) = handle.join() {
            println!("{:?}", result);
        }
    }
}
