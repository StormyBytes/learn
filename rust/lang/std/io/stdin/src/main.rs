use std::io::{self, BufRead as _};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        println!("{}", line);
    }
}
