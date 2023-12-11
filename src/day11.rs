use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
fn galatic_expand(space: Vec<String>) -> Vec<(usize, usize)> {
    let expansion_factor = 1_000_000;
    let mut columns = HashMap::new();
    for (l_idx, line) in space.iter().enumerate() {
        for (c_idx, char) in line.trim().chars().enumerate() {
            if char == '#' {
                columns.insert(c_idx, l_idx);
            }
        }
    }
    let mut l_units = 0;
    let mut galaxies = vec![];
    for line in space {
        let mut empty = true;
        let mut c_units = 0;
        for (c_idx, char) in line.trim().chars().enumerate() {
            if char == '#' {
                galaxies.push((l_units, c_units));
                empty = false;
            }
            if let None = columns.get(&c_idx) {
                c_units += expansion_factor;
            } else {
                c_units += 1;
            }
        }
        if empty {
            l_units += expansion_factor;
        } else {
            l_units += 1;
        }
    }
    galaxies
}

#[allow(dead_code)]
fn get_distance_sum(galaxies: Vec<(usize, usize)>) -> u64 {
    let mut sum = 0;
    let mut paired = HashSet::new();
    for (x, y) in &galaxies {
        for (ox, oy) in &galaxies {
            if x == ox && y == oy {
                continue;
            } else {
                if paired.contains(&(x, y, ox, oy)) || paired.contains(&(ox, oy, x, y)) {
                    continue;
                } else {
                    paired.insert((x, y, ox, oy));
                    let dist = x.abs_diff(*ox) + y.abs_diff(*oy);
                    sum += dist;
                }
            }
        }
    }
    sum as u64
}

#[allow(dead_code)]
pub fn soln(lines: Vec<String>) -> (u64, u64) {
    let galaxies = galatic_expand(lines);
    let dist = get_distance_sum(galaxies);
    (dist, 0)
}
