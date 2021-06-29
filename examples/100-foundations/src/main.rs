use rand::Rng;

fn main() {
    let mut random_numbers = Vec::new();

    for _i in 0..=100 {
        random_numbers.push(rand::thread_rng().gen_range(0..=100))
    }

    println!("{:?}", random_numbers);
}
