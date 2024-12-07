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
    let matrix: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
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

    // Walk the guard and count her steps
    let mut visited_coords: Vec<Coord> = Vec::new();

    visited_coords.push(guard_pos.0); // Push the first pos at beginning

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
                if x == '#' {
                    // Turn right
                    guard_pos.1 = match guard_pos.1 {
                        Direction::North => Direction::East,
                        Direction::East => Direction::South,
                        Direction::South => Direction::West,
                        Direction::West => Direction::North,
                    };

                    continue;
                } else {
                    // Make step
                    guard_pos.0 = next_step;
                    if !visited_coords.contains(&next_step) {
                        visited_coords.push(next_step);
                    }
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }

    println!["Part 1: {}", visited_coords.len()];
}
