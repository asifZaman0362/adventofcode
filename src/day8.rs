use std::collections::HashMap;

#[allow(dead_code)]
fn get_lcm(n1: u64, n2: u64) -> u64 {
    let mut rem: u64= 0;
    let mut x: u64  = 0;
    let mut y: u64  = 0;
    
    if n1 > n2 {
        x = n1;
        y = n2;
    }
    else {
        x = n2;
        y = n1;
    }

    rem = x % y;

    while rem != 0 {
        x = y;
        y = rem;
        rem = x % y;
    }
    n1 * n2 / y
}

#[allow(dead_code)]
pub fn soln(lines: Vec<String>) -> u64 {
    let mut steps = 0;
    let mut direction_iter = lines[0].trim().as_bytes().iter();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut current = vec![];
    let mut steps = Vec::new();
    for line in &lines[2..] {
        let (from, to) = line.trim().split_once(" = ").unwrap();
        let (left, right) = to.trim().split_once(", ").unwrap();
        map.insert(from, (&left[1..], &right[..right.len() - 1]));
        if from.ends_with("A") {
            current.push(from);
        }
    }
    for starting in current {
        let start = starting.to_owned();
        let mut count = 0;
        let mut node = starting;
        loop {
            if let Some(dir) = direction_iter.next() {
                count += 1;
                let to = map.get(node).unwrap();
                let next = match dir {
                    b'L' => to.0,
                    b'R' => to.1,
                    _ => panic!("oh noooo!"),
                };
                if next.ends_with("Z") {
                    steps.push(count);
                    break;
                }
                node = next;
            } else {
                direction_iter = lines[0].trim().as_bytes().iter();
            }
        }
    }
    let mut lcm = 1;
    for step in steps {
        lcm = get_lcm(step, lcm);
    }
    lcm
}
