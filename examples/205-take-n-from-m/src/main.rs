use rand::seq::SliceRandom;
use std::env;

fn take_n_from_m(take: usize, from: u64) -> Vec<u64> {
    let mut rng = rand::rng();
    let mut nums: Vec<u64> = (1..=from).collect();
    nums.shuffle(&mut rng);
    let mut result = nums[0..take].to_vec();
    result.sort_unstable();
    result
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    for chunk in args.chunks(2) {
        if chunk.len() == 2 {
            let take = chunk[0].parse::<usize>();
            let from = chunk[1].parse::<u64>();
            match (take, from) {
                (Ok(t), Ok(f)) => {
                    println!("take {} from {}", t, f);
                    println!("-> numbers: {:?}", take_n_from_m(t, f));
                }
                (_, _) => {
                    println!("Error parsing arguments");
                }
            }
        } else {
            println!("Missing arguments, take {} from what?", chunk[0]);
        }
    }
}

#[test]
fn test_take_n_from_m() {
    let mut result = take_n_from_m(6, 45);
    result.sort_by(|a, b| b.partial_cmp(a).unwrap());

    assert_eq!(result.len(), 6);
    assert!(result[0] < 45);
}
