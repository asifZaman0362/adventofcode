use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;

fn get_card_points(line: &str) -> (u32, u32) {
    let (_, numbers) = line.split_once(":").unwrap();
    let (winning, you_have) = numbers.split_once("|").unwrap();
    let winning = winning.trim().split(" ");
    let you_have = you_have.trim().split(" ");
    let mut w_nums: HashSet<u32> = HashSet::new();
    let mut matches = 0;
    for w_num in winning {
        if let Ok(num) = w_num.parse::<u32>() {
            w_nums.insert(num);
        }
    }
    for y_num in you_have {
        if let Ok(num) = y_num.parse::<u32>() {
            if w_nums.get(&num).is_some() {
                matches += 1;
            }
        }
    }
    let score: u32 = 2;
    if matches == 0 {
        (0, 0)
    } else {
        (score.pow(matches - 1), matches)
    }
}

pub fn soln() -> std::io::Result<(u32, usize)> {
    let file = File::open("inputs/day4.txt")?;
    let mut buf_reader = std::io::BufReader::new(file);
    let mut buf = String::new();
    let mut score = 0;
    let mut matches: Vec<u32> = Vec::new();
    let mut copies: Vec<usize> = Vec::new();
    loop {
        buf.clear();
        match buf_reader.read_line(&mut buf) {
            Ok(0) => break,
            Ok(_) => {
                let (card_points, n_matches) = get_card_points(buf.trim());
                score += card_points;
                matches.push(n_matches);
                copies.push(1);
            }
            Err(_) => break,
        }
    }
    for (idx, n_matches) in matches[..matches.len() - 1].iter().enumerate() {
        let min = copies.len().min(idx + *n_matches as usize + 1);
        for next in idx + 1..min {
            copies[next] += copies[idx];
        }
    }
    let total_copies = copies.iter().fold(0, |sum, x| sum + x);
    Ok((score, total_copies))
}
