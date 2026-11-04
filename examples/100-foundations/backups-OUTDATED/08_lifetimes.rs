fn get_biggest<'a>(n: &'a i32, m: &'a i32) -> &'a i32 {
    if n > m {
        n
    } else {
        m
    }
}

fn main() {
    let a = 5;
    let biggest;
    let b = 6;
    biggest = get_biggest(&a, &b);

    println!("{}", biggest);
}
