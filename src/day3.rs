use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
fn get_symbols(string: &str) -> (Vec<usize>, HashMap<usize, usize>) {
    let mut numbers = HashMap::new();
    let mut symbols = Vec::new();
    let mut newlines = 0;
    let mut prev = 0;
    let mut flag = false;
    for (idx, char) in string.as_bytes().iter().enumerate() {
        match char {
            b'0'..=b'9' => {
                if flag {
                    numbers.insert(prev, ((idx - newlines) - prev) + 1);
                } else {
                    numbers.insert(idx - newlines, 1);
                    prev = idx - newlines;
                }
                flag = true;
            }
            b'.' => {
                flag = false;
            }
            b'\n' => {
                newlines += 1;
                flag = false;
            }
            _ => {
                flag = false;
                symbols.push(idx - newlines);
            }
        }
    }
    (symbols, numbers)
}

#[allow(dead_code)]
pub fn soln() -> std::io::Result<(u32, u64)> {
    let mut ex = String::new();
    let file = File::open("inputs/day3.txt")?;
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    loop {
        match reader.read_line(&mut buf) {
            Ok(0) => break,
            Ok(_) => {
                ex += buf.trim();
                buf.clear();
            }
            Err(_) => break,
        }
    }
    let length = 140;
    let mut result = 0;
    let mut part_numbers = Vec::new();
    let (symbols, numbers) = get_symbols(&ex);
    let mut keys: Vec<_> = numbers.keys().collect();
    keys.sort();
    for key in keys.clone() {
        let start = *key;
        let end = start + *numbers.get(key).unwrap();
        for symbol in &symbols {
            let key_line = key / length;
            let symbol_line = symbol / length;
            if (key_line as i32).abs_diff(symbol_line as i32) <= 1 {
                let symbol_col = symbol % length;
                let key_left = key % length;
                let key_right = (*key + *numbers.get(key).unwrap()) % length;
                let l = if key_left == 0 { 0 } else { key_left - 1 };
                let r = if key_right == 0 { length } else { key_right };
                if symbol_col <= r && symbol_col >= l {
                    part_numbers.push(&ex[start..end]);
                    break;
                }
            }
        }
    }
    let mut ratios = 0;
    for symbol in &symbols {
        let sym = ex.as_bytes()[*symbol];
        if sym == b'*' {
            let mut gears = Vec::new();
            for key in keys.clone() {
                let start = *key;
                let end = start + *numbers.get(key).unwrap();
                let key_line = key / length;
                let symbol_line = symbol / length;
                if (key_line as i32).abs_diff(symbol_line as i32) <= 1 {
                    let symbol_col = symbol % length;
                    let key_left = key % length;
                    let key_right = (*key + *numbers.get(key).unwrap()) % length;
                    let l = if key_left == 0 { 0 } else { key_left - 1 };
                    let r = if key_right == 0 { length } else { key_right };
                    if symbol_col <= r && symbol_col >= l {
                        gears.push(&ex[start..end]);
                    }
                }
            }
            if gears.len() == 2 {
                let ratio = gears[0].parse::<u64>().unwrap() * gears[1].parse::<u64>().unwrap();
                ratios += ratio;
            }
        }
    }
    for part_number in part_numbers {
        result += part_number.parse::<u32>().unwrap();
    }
    Ok((result, ratios))
}
