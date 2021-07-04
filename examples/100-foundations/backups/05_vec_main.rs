fn main() {
    let mut vecc = vec![1, 2, 3, 4];

    let mut vecc_collected: Vec<i32> = (1..5).collect();

    vecc_collected.push(5);

    println!("{:?}", vecc_collected);

    // Turbofish
    let mut vecc_collected = (1..5).collect::<Vec<i32>>();

    vecc.push(5);

    vecc_collected.push(6);

    for i in vecc_collected {
        println!("> {}", i);
    }
    let mut i = 0;
    while i < vecc.len() {
        println!(": {}", vecc[i]);
        i += 1;
    }
}
