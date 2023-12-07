use regex::*;

fn solve_quadratic(a: f64, b: f64, c: f64) -> (i64, i64) {
    let disc = b.powi(2) - 4.0 * a * c;
    if disc < 0.0 {
        panic!("no real roots!");
    } else {
        let d = disc.sqrt();
        let first_root = (-1.0 * b + d) / (2.0 * a);
        let second_root = (-1.0 * b - d) / (2.0 * a);
        let x = first_root.ceil() as i64;
        let y = second_root.floor() as i64;
        (x, y)
    }
}

pub fn soln(lines: Vec<String>) -> (i64, i64) {
    let re = Regex::new("([0-9]+)").unwrap();
    let times = re
        .captures_iter(&lines[0])
        .map(|c| c.extract::<1>())
        .map(|x| x.0);
    let distances = re
        .captures_iter(&lines[1])
        .map(|c| c.extract::<1>())
        .map(|x| x.0);
    let entries = times.zip(distances);
    let mut num_ways = 1;
    let mut total_time = String::new();
    let mut total_dist = String::new();
    for (time, dist) in entries {
        total_dist += dist;
        total_time += time;
        let time = time.parse::<i64>().unwrap();
        let dist = dist.parse::<i64>().unwrap();
        let res = solve_quadratic(-1.0, time as f64, -1.0 * (dist + 1) as f64);
        dbg!(&res);
        num_ways *= res.1 - res.0 + 1;
    }
    let total_time = total_time.parse::<i64>().unwrap();
    let total_dist = total_dist.parse::<i64>().unwrap();
    let res = solve_quadratic(-1.0, total_time as f64, -1.0 * (total_dist + 1) as f64);
    (num_ways, res.1 - res.0 + 1)
}
