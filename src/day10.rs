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

fn traverse(x: usize, y: usize, lines: &Vec<String>, dir: Direction) -> usize {
    let mut from = dir;
    let mut steps = 0;
    let mut x = x;
    let mut y = y;
    let mut this = lines[x].as_bytes()[y];
    loop {
        steps += 1;
        match this {
            b'S' => {
                return steps / 2;
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
    }
}

pub fn soln(lines: Vec<String>) -> usize {
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
                    return traverse(x, y, &lines, -dir);
                }
            }
            b'7' => {
                if dir == Direction::Right || dir == Direction::Up {
                    return traverse(x, y, &lines, -dir);
                }
            }
            b'F' => {
                if dir == Direction::Left || dir == Direction::Up {
                    return traverse(x, y, &lines, -dir);
                }
            }
            b'J' => {
                if dir == Direction::Right || dir == Direction::Down {
                    return traverse(x, y, &lines, -dir);
                }
            }
            b'|' => {
                if dir == Direction::Up || dir == Direction::Down {
                    return traverse(x, y, &lines, -dir);
                }
            }
            b'L' => {
                if dir == Direction::Down || dir == Direction::Left {
                    return traverse(x, y, &lines, -dir);
                }
            }
            _ => {
                continue
            }
        }
    }
    0
}
