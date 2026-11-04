use rand::Rng;

fn main() {
    let mut vec = Vec::<i32>::new();

    for _ in 0..10 {
        let random_number = rand::thread_rng().gen_range(0..=50);
        if random_number % 3 != 0 {
            vec.push(random_number);
        }
    }

    println!("{:?}", vec);

    let random_range: Vec<i32> = (0..10)
        .map(|_| rand::thread_rng().gen_range(0..=50))
        .filter(|el| el % 3 != 0)
        .collect::<Vec<i32>>();

    println!("{:?}", random_range);
}
