use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_num_from_line(line: &str) -> i32 {
    if let Some((first, pos)) = find_first_digit(&line) {
        if let Some(last) = find_last_digit(&line[pos..]) {
            first * 10 + last
        } else {
            first * 10 + first
        }
    } else {
        0
    }
}

fn find_first_digit(line: &str) -> Option<(i32, usize)> {
    let mut iter = line.as_bytes().iter().enumerate();
    while let Some((idx, c)) = iter.next() {
        match c {
            b'0'..=b'9' => {
                return Some(((c - b'0') as i32, idx + 1));
            }
            b'z' => {
                if line[idx..].starts_with("zero") {
                    return Some((0, idx+4));
                }
            }
            b'o' => {
                if line[idx..].starts_with("one") {
                    return Some((1, idx+3));
                }
            }
            b't' => {
                if line[idx..].starts_with("two") {
                    return Some((2, idx+3));
                } else if line[idx..].starts_with("three") {
                    return Some((3, idx+5));
                }
            }
            b'f' => {
                if line[idx..].starts_with("four") {
                    return Some((4, idx+4));
                } else if line[idx..].starts_with("five") {
                    return Some((5, idx+4));
                }
            }
            b's' => {
                if line[idx..].starts_with("six") {
                    return Some((6, idx+3));
                } else if line[idx..].starts_with("seven") {
                    return Some((7, idx+5));
                }
            }
            b'e' => {
                if line[idx..].starts_with("eight") {
                    return Some((8, idx+5));
                }
            }
            b'n' => {
                if line[idx..].starts_with("nine") {
                    return Some((9, idx+4));
                }
            }
            _ => {}
        }
    }
    None
}

fn find_last_digit(line: &str) -> Option<i32> {
    let iter = line.as_bytes().iter().enumerate();
    let mut iter = iter.rev();
    while let Some((idx, c)) = iter.next() {
        match c {
            b'0'..=b'9' => {
                return Some((c - b'0') as i32);
            }
            b'o' => {
                if line[idx..].starts_with("orez") {
                    return Some(0);
                }
            }
            b'e' => {
                if line[idx..].starts_with("eno") {
                    return Some(1);
                } else if line[idx..].starts_with("eerht") {
                    return Some(3);
                } else if line[idx..].starts_with("evif") {
                    return Some(5);
                } else if line[idx..].starts_with("enin") {
                    return Some(9);
                }
            }
            b'x' => {
                if line[idx..].starts_with("xis") {
                    return Some(6);
                }
            }
            b't' => {
                if line[idx..].starts_with("thgie") {
                    return Some(8);
                }
            }
            b'o' => {
                if line[idx..].starts_with("owt") {
                    return Some(2);
                }
            }
            b'r' => {
                if line[idx..].starts_with("ruof") {
                    return Some(4);
                }
            }
            b'n' => {
                if line[idx..].starts_with("neves") {
                    return Some(7);
                }
            }
            _ => {}
        }
    }
    None
}

pub fn soln() -> std::io::Result<()> {
    let mut res = 0;
    let file = File::open("inputs/day1-1.txt")?;
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    loop {
        match reader.read_line(&mut buf) {
            Ok(0) => break,
            Ok(_) => {
                let num = get_num_from_line(&buf);
                res += num;
                buf.clear();
            }
            Err(_) => break,
        }
    }
    println!("{res}");
    Ok(())
}
