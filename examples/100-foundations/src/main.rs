fn get_biggest<'a, 'b, 'c>(n: &'a i32, m: &'b i32) -> &'c i32
where
    'a: 'c,
    'b: 'c,
{
    if n > m {
        n
    } else {
        m
    }
}

fn main() {
    let a = 5;
    let b = 6;

    let biggest = get_biggest(&a, &b);
    println!("{}", biggest);
}
