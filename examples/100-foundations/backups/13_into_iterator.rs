use rand::{prelude::IteratorRandom, thread_rng};

#[derive(Default)]
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
    let lotto = Lotto::new(6, 45);
    for i in lotto {
        println!("{}", i);
    }
}
