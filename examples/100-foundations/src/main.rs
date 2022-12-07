use rand::{prelude::IteratorRandom, thread_rng};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    sync::{Arc, Mutex},
    thread,
};

#[derive(Default, Debug)]
struct Lotto {
    numbers: Vec<usize>,
}

impl Lotto {
    pub fn new(amount: usize, max: usize) -> Self {
        let pot = 1..=max;
        let mut rng = thread_rng();
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
    let lottos = Mutex::new(Vec::<Lotto>::new());
    let lottos = Arc::new(lottos);
    let pairs = [(6, 45), (5, 50), (2, 12)];
    let mut handles = vec![];

    for (take, from) in pairs {
        let lottos = Arc::clone(&lottos);
        let handle = thread::spawn(move || {
            let lotto = Lotto::new(take, from);
            lottos.lock().unwrap().push(lotto);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    for lotto in lottos.lock().unwrap().iter() {
        println!("{:?}", lotto);
    }
}

fn isTruthy() -> bool {
    true
}

/*
// 2
fn main() {
    let pairs = [(6, 45), (5, 50), (2, 12)];
    for (take, from) in pairs {
        let handle = thread::spawn(move || {
            let lotto = Lotto::new(take, from);
            println!("{:?}", lotto);
        });

        handle.join().unwrap();
    }
}
 */

/*
//1
fn main() {
    let pairs = [(6, 45), (5, 50), (2, 12)];
    for (_take, _from) in pairs {
        let handle = thread::spawn(|| {
            let lotto = Lotto::new(6, 45);
            println!("{:?}", lotto);
        });

        handle.join().unwrap();
    }
}

 */
