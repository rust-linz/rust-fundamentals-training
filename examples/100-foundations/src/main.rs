fn add(i: i32, j: i32) -> i32 {
    i + j
}

fn main() {
    let a = 32;
    let b: i32 = 64;

    let c = add(a, b);

    println!("{}", c);
}
