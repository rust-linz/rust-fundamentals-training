fn print_numbers(numbers: &Vec<i32>) {
    println!("{:?}", numbers);
}

fn create_numbers(numbers: &mut Vec<i32>) {
    for i in 0..10 {
        numbers.push(i);
    }
}

fn main() {
    let inital_numbers = vec![1i32, 2, 3, 4, 5];

    let mut numbers = inital_numbers;
    numbers.push(6);
    numbers.push(7);

    print_numbers(&numbers);
    // What happens here?
    print_numbers(&numbers);

    let mut created_numbers = Vec::new();
    create_numbers(&mut created_numbers);
    print_numbers(&created_numbers);
}
