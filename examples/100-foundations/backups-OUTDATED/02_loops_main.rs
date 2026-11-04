use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/*
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let f = File::open("stairway.md").unwrap();
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    loop {
        let len = reader.read_line(&mut line).unwrap();

        if len == 0 {
            break;
        }
        println!("{} ({} bytes long)", line, len);

        line.truncate(0);
    }
}

    loop {
        if let Some(line) = lines.next() {
            let line = line.unwrap();
            println!("{} ({} bytes long)", line, line.len());
        } else {
            break;
        }
    }

    let mut lines = BufReader::new(f).lines();
    while let Some(line) = lines.next() {
        let line = line.unwrap();
        println!("{} ({} bytes long)", line, line.len());
    }

*/

fn main() {
    let f = File::open("stairway.md").unwrap();
    let lines = BufReader::new(f).lines();
    for line in lines {
        let line = line.unwrap();
        println!("{} ({} bytes long)", line, line.len());
    }
}
