use std::fs::File;
use std::io::{BufRead, BufReader};
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() -> std::io::Result<()> {
    const INPUT: &str = "inputs/day5.txt";
    let mut lines = Vec::new();
    let mut buf = String::new();
    let mut reader = BufReader::new(File::open(INPUT)?);
    loop {
        match reader.read_line(&mut buf) {
            Ok(0) => break,
            Ok(_) => {
                lines.push(buf.trim().to_owned());
                buf.clear();
            }
            Err(_) => break
        }
    }
    let res = day5::soln(lines);
    println!("{res:?}");
    Ok(())
}
