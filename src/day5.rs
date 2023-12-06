use core::ops::Range;

type Ran = Range<i64>;

struct Map {
    src: i64,
    dst: i64,
    len: i64,
}

fn split_ranges(seeds: Ran, maps: &Vec<Map>) -> Vec<Ran> {
    let mut ranges = vec![seeds];
    let mut out = vec![];
    for map in maps {
        let src = map.src..map.src + map.len;
        let diff = map.dst - map.src;
        let mut remaining = vec![];
        for range in &ranges {
            if range.contains(&src.start) {
                if range.contains(&src.end) {
                    out.push(src.start + diff..src.end + diff);
                    remaining.push(range.start..src.start);
                    remaining.push(src.end..range.end);
                } else {
                    out.push(src.start + diff..range.end + diff);
                    remaining.push(range.start..src.start);
                }
            } else if src.contains(&range.start) {
                if src.contains(&range.end) {
                    return vec![range.start + diff..range.end + diff];
                } else {
                    out.push(range.start + diff..src.end + diff);
                    remaining.push(src.end..range.end);
                }
            } else {
                remaining.push(range.clone());
            }
        }
        ranges = remaining;
    }
    out.append(&mut ranges);
    out
}

pub fn soln(lines: Vec<String>) -> (i64, i64) {
    let mut maps: Vec<Vec<Map>> = vec![];
    let mut seeds = vec![];
    let mut idx = 0;
    for line in lines {
        if line.trim().len() == 0 {
            continue;
        } else if line.starts_with("seeds") {
            let (_, y) = line.trim().split_once(" ").unwrap();
            let splits = y.trim().split(" ");
            for split in splits {
                seeds.push(split.parse::<i64>().unwrap());
            }
        } else if line.starts_with("seed")
            || line.starts_with("soil")
            || line.starts_with("fert")
            || line.starts_with("water")
            || line.starts_with("light")
            || line.starts_with("temp")
            || line.starts_with("humid")
        {
            if maps.get(idx).is_some() {
                idx += 1;
            }
            maps.push(vec![]);
            continue;
        } else {
            let (dst, rem) = line.trim().split_once(" ").unwrap();
            let (src, len) = rem.split_once(" ").unwrap();
            let dst = dst.parse().unwrap();
            let src = src.parse().unwrap();
            let len = len.parse().unwrap();
            maps[idx].push(Map { dst, src, len });
        }
    }
    let mut ranges = seeds
        .chunks_exact(2)
        .into_iter()
        .map(|x| x[0]..x[0] + x[1])
        .collect::<Vec<_>>();
    for map in maps {
        let mut new_ranges = vec![];
        for range in ranges {
            let mut out = split_ranges(range, &map);
            new_ranges.append(&mut out);
        }
        ranges = new_ranges;
    }
    let mut lowest = i64::MAX;
    let mut total = 0;
    for range in &ranges {
        if range.start < lowest {
            lowest = range.start;
        } else if range.end < lowest {
            lowest = range.end;
        }
        total += range.end - range.start;
    }
    let mut t = 0;
    for seed in seeds.chunks(2) {
        t += seed[1];
    }
    assert_eq!(&total, &t);
    (lowest, 0)
}
