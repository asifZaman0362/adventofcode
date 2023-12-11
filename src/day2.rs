use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

#[allow(dead_code)]
fn get_game_info(line: &str) -> (u32, HashMap<&str, u32>) {
    let mut out: HashMap<&str, u32> = HashMap::new();
    let (_, info) = line.split_once(" ").unwrap();
    let (game_num, events) = info.split_once(":").unwrap();
    let events = events.split(";");
    for event in events {
        let sets = event.split(",");
        for set in sets {
            let set = set.trim();
            let (num, color) = set.split_once(" ").unwrap();
            let num: u32 = num.parse().unwrap();
            if let Some(prev) = out.get(color) {
                if num > *prev {
                    out.insert(color, num);
                }
            } else {
                out.insert(color, num);
            }
        }
    }
    (game_num.parse().unwrap(), out)
}

#[allow(dead_code)]
pub fn soln() -> std::io::Result<(u32, u32)> {
    let mut sum = 0;
    let file = File::open("inputs/day2.txt")?;
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    let mut power_sum = 0;
    loop {
        match reader.read_line(&mut buf) {
            Ok(0) => break,
            Ok(_) => {
                let b = buf.trim();
                let mut power = 1;
                let (game_num, res) = get_game_info(b);
                let red = res.get("red").unwrap_or(&0);
                let green = res.get("green").unwrap_or(&0);
                let blue= res.get("blue").unwrap_or(&0);
                if red <= &12 && green <= &13 && blue <= &14 {
                    sum += game_num;
                }
                power = red * green * blue;
                power_sum += power;
                buf.clear();
            }
            Err(_) => break,
        }
    }
    Ok((sum, power_sum))
}
