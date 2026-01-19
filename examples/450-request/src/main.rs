#![allow(dead_code)]

use std::{
    fmt::Display,
    io::{Read, Write},
    net::{Shutdown, TcpStream},
    num::ParseIntError,
    sync::{Arc, Mutex},
    thread,
};

mod fibonacci;

// Error Handling in 4 tiers
// 1. You have something, that might reasonably be absent.  --> Option
// 2. Something goes wrong, but you can reasonably handle it --> Result
// 3. Something goes wrong, but you can't reasonably handle it --> panic --> Thread
// 4. Catastrophic circumstances, abort the program

// 1. I want to write on the happy path
// 2. I want to have tier 2 error safety

fn request(host: &str, port: u16, path: &str) -> Result<String, std::io::Error> {
    let mut socket = TcpStream::connect((host, port))?;

    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);
    socket.write_all(request.as_bytes())?;
    socket.shutdown(Shutdown::Write)?;

    let mut response_buffer = String::new();
    socket.read_to_string(&mut response_buffer)?;

    Ok(response_buffer)
}

#[derive(Debug)]
enum MyError {
    ParseError(ParseIntError),
    IoError(std::io::Error),
}

impl std::error::Error for MyError {}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::ParseError(err) => write!(f, "Parse Error: {}", err),
            MyError::IoError(err) => write!(f, "IO Error: {}", err),
        }
    }
}

impl From<std::io::Error> for MyError {
    fn from(err: std::io::Error) -> Self {
        MyError::IoError(err)
    }
}

impl From<ParseIntError> for MyError {
    fn from(err: ParseIntError) -> Self {
        MyError::ParseError(err)
    }
}

fn main() {
    let urls = vec![
        (String::from("de.wikipedia.org"), 80, String::from("/")),
        (String::from("fr.wikipedia.org"), 80, String::from("/")),
        (String::from("pt.wikipedia.org"), 80, String::from("/")),
        (String::from("it.wikipedia.org"), 80, String::from("/")),
        (String::from("en.wikipedia.org"), 80, String::from("/")),
    ];
    let results = Mutex::new(Vec::<String>::new());
    let results = Arc::new(results);

    let mut handles = Vec::new();

    for url in urls {
        let results = results.clone();
        let handle = thread::spawn(move || {
            let _guard = results.lock();
            request(&url.0, url.1, &url.2).unwrap()

            /*match guard {
                Ok(mut guard) => guard.push(response),
                Err(_) => eprintln!("Access to Mutex not possible"),
            }*/
        });
        handles.push(handle);
    }

    for handle in handles {
        if let Ok(value) = handle.join() {
            println!("{value}");
        }
    }

    /*let guard = results.lock();
    if let Ok(results) = guard {
        let results = results.to_owned().into_iter();
        for result in results {
            println!("{result}");
        }
    }*/
}
