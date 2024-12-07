#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(PartialEq, Clone, Copy, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

pub fn solve(input: &str) {
    let mut matrix: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let mut guard_pos: (Coord, Direction) = (Coord { x: 0, y: 0 }, Direction::North);

    // Search the guard
    for (y, line) in matrix.iter().enumerate() {
        for (x, &char) in line.iter().enumerate() {
            if char == '^' {
                guard_pos = (Coord { x, y }, Direction::North);
                break;
            } else if char == '>' {
                guard_pos = (Coord { x, y }, Direction::East);
                break;
            } else if char == 'v' {
                guard_pos = (Coord { x, y }, Direction::South);
                break;
            } else if char == '<' {
                guard_pos = (Coord { x, y }, Direction::West);
                break;
            }
        }
    }

    let mut sum: i32 = 0;
    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            if matrix[y][x] == '.' {
                matrix[y][x] = 'O';
            } else {
                continue; // I honestly don't know why I *need* this, otherwise it doesn't work
            }

            if check_loop(&matrix, guard_pos.clone()) {
                sum += 1;
            }

            matrix[y][x] = '.';
        }
    }

    println!["Part 2: {}", sum];
}

fn check_loop(matrix: &Vec<Vec<char>>, mut guard_pos: (Coord, Direction)) -> bool {
    // Walk the guard and count her steps
    let mut hit_o: i32 = 0;
    let mut hit_hashtag_history: Vec<Coord> = Vec::new();

    loop {
        let next_step: Coord = match guard_pos.1 {
            Direction::North => Coord {
                x: guard_pos.0.x,
                y: guard_pos.0.y.overflowing_sub(1).0,
            },
            Direction::East => Coord {
                x: guard_pos.0.x.overflowing_add(1).0,
                y: guard_pos.0.y,
            },
            Direction::South => Coord {
                x: guard_pos.0.x,
                y: guard_pos.0.y.overflowing_add(1).0,
            },
            Direction::West => Coord {
                x: guard_pos.0.x.overflowing_sub(1).0,
                y: guard_pos.0.y,
            },
        };

        // Check the next step
        if let Some(y) = matrix.get(next_step.y) {
            if let Some(&x) = y.get(next_step.x) {
                if x == '#' || x == 'O' {
                    // Turn right
                    guard_pos.1 = match guard_pos.1 {
                        Direction::North => Direction::East,
                        Direction::East => Direction::South,
                        Direction::South => Direction::West,
                        Direction::West => Direction::North,
                    };
                    if x == 'O' {
                        if hit_o >= 3 {
                            return true;
                        }
                        hit_o += 1;
                    } else {
                        hit_hashtag_history.push(guard_pos.0);

                        if hit_hashtag_history.len() > 8 {
                            if hit_hashtag_history
                                [hit_hashtag_history.len() - 8..hit_hashtag_history.len() - 4]
                                == hit_hashtag_history[hit_hashtag_history.len() - 4..]
                            {
                                return true;
                            }
                        }
                    }

                    continue;
                }
            } else {
                break;
            }
        } else {
            break;
        }

        // Make step
        guard_pos.0 = next_step;
    }

    return false;
}
