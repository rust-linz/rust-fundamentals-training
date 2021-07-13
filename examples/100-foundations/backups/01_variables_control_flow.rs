use rand::Rng;

fn add(i: i32, j: i32) -> i32 {
    i + j
}

fn main() {
    let a = 32;
    let b: i32 = 64;
    let c = add(a, b);

    println!("{}", c);

    let d = 0i32;
    let c = a + d;

    println!("{}", c);

    let mut c = b + d;
    c += a;

    println!("{}", c);

    let num = rand::thread_rng().gen_range(0..10);

    let msg = match num {
        5 => "So close",
        _n if _n < 6 => "Win!",
        _ => "Lost! :-(",
    };

    println!("Draw {}, {}", num, msg);
    // Maybe show freezing and block scopes
}
