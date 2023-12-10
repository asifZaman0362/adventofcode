use std::collections::HashSet;

#[derive(PartialEq)]
#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Down,
    Up,
}

impl std::ops::Neg for Direction {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left
        }
    }
}

fn get_next(x: usize, y: usize, map: &Vec<String>, dir: Direction) -> (u8, usize, usize) {
    let (nx, ny) = match dir {
        Direction::Left => (x, y - 1),
        Direction::Right => (x, y + 1),
        Direction::Up => (x - 1, y),
        Direction::Down => (x + 1, y),
    };
    (map[nx].as_bytes()[ny], nx, ny)
}

fn traverse(x: usize, y: usize, lines: &Vec<String>, dir: Direction, s: char) -> (usize, usize) {
    let mut from = dir;
    let mut steps = 0;
    let mut x = x;
    let mut y = y;
    let mut this = lines[x].as_bytes()[y];
    let mut loop_pipes = HashSet::new();
    let steps = loop {
        loop_pipes.insert((x, y));
        steps += 1;
        match this {
            b'S' => {
                break steps / 2;
            }
            b'L' => {
                if from == Direction::Right {
                    from = Direction::Down;
                    (this, x, y) = get_next(x, y, lines, Direction::Up);
                } else {
                    from = Direction::Left;
                    (this, x, y) = get_next(x, y, lines, Direction::Right);
                }
            }
            b'J' => {
                if from == Direction::Up {
                    from = Direction::Right;
                    (this, x, y) = get_next(x, y, lines, Direction::Left);
                } else {
                    from = Direction::Down;
                    (this, x, y) = get_next(x, y, lines, Direction::Up);
                }
            }
            b'-' => {
                if from == Direction::Left {
                    (this, x, y) = get_next(x, y, lines, Direction::Right);
                } else {
                    (this, x, y) = get_next(x, y, lines, Direction::Left);
                }
            }
            b'F' => {
                if from == Direction::Right {
                    from = Direction::Up;
                    (this, x, y) = get_next(x, y, lines, Direction::Down);
                } else {
                    from = Direction::Left;
                    (this, x, y) = get_next(x, y, lines, Direction::Right);
                }
            }
            b'7' => {
                if from == Direction::Down {
                    from = Direction::Right;
                    (this, x, y) = get_next(x, y, lines, Direction::Left);
                } else {
                    from = Direction::Up;
                    (this, x, y) = get_next(x, y, lines, Direction::Down);
                }
            }
            b'|' => {
                if from == Direction::Down {
                    (this, x, y) = get_next(x, y, lines, Direction::Up);
                } else {
                    (this, x, y) = get_next(x, y, lines, Direction::Down);
                }
            }
            _ => {}
        }
    };
    let mut total_enclosed = 0;
    for (l_idx, line) in lines.iter().enumerate() {
        let mut num_crossings: f64 = 0.0;
        let mut inside = 0;
        for (c_idx, char) in line.chars().enumerate() {
            let mut char = char;
            if char == 'S' {
                char = s;
            }
            if loop_pipes.contains(&(l_idx, c_idx)) {
                match char {
                    'L' => num_crossings += -0.5,
                    'J' => num_crossings += 0.5,
                    'F' => num_crossings += 0.5,
                    '7' => num_crossings += -0.5,
                    '|' => num_crossings += 1.0,
                    _ => {}
                }
            } else {
                if (num_crossings.floor() as i64).abs() % 2 == 1 {
                    inside += 1;
                }
            }
        }
        total_enclosed += inside;
    }
    (steps, total_enclosed)
}

pub fn soln(lines: Vec<String>) -> (usize, usize) {
    let mut start = (0usize, 0usize);
    let w = lines[0].len();
    let h = lines.len();
    for (l_idx, line) in lines.iter().enumerate() {
        for (c_idx, char) in line.chars().enumerate() {
            if char == 'S' {
                start = (l_idx, c_idx);
            }
        }
    }
    let dir: [Direction; 4] = [
        Direction::Left,
        Direction::Right,
        Direction::Down,
        Direction::Up,
    ];
    let (mut up, mut down, mut left, mut right) = (false, false, false, false);
    if start.0 > 0 {
        let above = lines[start.0 - 1].as_bytes()[start.1];
        if above == b'7' || above == b'F' || above == b'|' {
            up = true;
        }
    }
    if start.0 < h - 1 {
        let bottom = lines[start.0 + 1].as_bytes()[start.1];
        if bottom == b'L' || bottom == b'J' || bottom == b'|' {
            down = true;
        }
    }
    if start.1 > 0 {
        let l = lines[start.0].as_bytes()[start.1 - 1];
        if l == b'F' || l == b'L' || l == b'-' {
            left = true;
        }
    }
    if start.1 < w - 1 {
        let r = lines[start.0].as_bytes()[start.1 + 1];
        if r == b'7' || r == b'J' || r == b'-' {
            right = true;
        }
    }
    let s = match (left, right, up, down) {
        (true, false, true, false) => 'J',
        (true, true, false, false) => '-',
        (false, true, true, false) => 'L',
        (true, false, false, true) => '7',
        (false, true, false, true) => 'F',
        (false, false, true, true) => '|',
        _ => {
            unreachable!("never")
        }
    };
    for d in dir {
        let dir = d;
        let (x, y) = match dir {
            Direction::Right => {
                if start.1 == w - 1 {
                    continue;
                } else {
                    (start.0, start.1 + 1)
                }
            }
            Direction::Left => {
                if start.1 == 0 {
                    continue;
                } else {
                    (start.0, start.1 - 1)
                }
            }
            Direction::Up => {
                if start.0 == 0 {
                    continue;
                } else {
                    (start.0 - 1, start.1)
                }
            }
            Direction::Down => {
                if start.0 == h - 1 {
                    continue;
                } else {
                    (start.0 + 1, start.1)
                }
            }
        };
        match lines[x].as_bytes()[y] {
            b'-' => {
                if dir == Direction::Left || dir == Direction::Right {
                    return traverse(x, y, &lines, -dir, s);
                }
            }
            b'7' => {
                if dir == Direction::Right || dir == Direction::Up {
                    return traverse(x, y, &lines, -dir, s);
                }
            }
            b'F' => {
                if dir == Direction::Left || dir == Direction::Up {
                    return traverse(x, y, &lines, -dir, s);
                }
            }
            b'J' => {
                if dir == Direction::Right || dir == Direction::Down {
                    return traverse(x, y, &lines, -dir, s);
                }
            }
            b'|' => {
                if dir == Direction::Up || dir == Direction::Down {
                    return traverse(x, y, &lines, -dir, s);
                }
            }
            b'L' => {
                if dir == Direction::Down || dir == Direction::Left {
                    return traverse(x, y, &lines, -dir, s);
                }
            }
            _ => {
                continue
            }
        }
    }
    (0, 0)
}
