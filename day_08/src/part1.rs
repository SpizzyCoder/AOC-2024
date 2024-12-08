#[derive(Debug, Clone, Copy)]
struct Antenna {
    identifier: char,
    coord: Coord,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

pub fn solve(input: &str) {
    let mut antennas: Vec<Antenna> = Vec::new();
    let x_bound: usize = input.lines().next().unwrap().len() - 1;
    let y_bound: usize = input.lines().count() - 1;

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char != '.' {
                antennas.push(Antenna {
                    identifier: char,
                    coord: Coord { x, y },
                });
            }
        }
    }

    let mut antinodes: Vec<Coord> = Vec::new();
    for index in 0..antennas.len() {
        let mut antennas_clone: Vec<Antenna> = antennas.clone();
        let antenna: Antenna = antennas_clone.remove(index);

        for cur_antenna in antennas_clone.iter() {
            if cur_antenna.identifier == antenna.identifier {
                if let Some(antinode_coords) =
                    calc_antinode(antenna, *cur_antenna, x_bound, y_bound)
                {
                    if !antinodes.contains(&antinode_coords) {
                        antinodes.push(antinode_coords);
                    }
                }
            }
        }
    }

    println!["Part 1: {}", antinodes.len()];
}

fn calc_antinode(from: Antenna, to: Antenna, x_bound: usize, y_bound: usize) -> Option<Coord> {
    let antinode_x: isize = (to
        .coord
        .x
        .wrapping_sub(from.coord.x.wrapping_sub(to.coord.x))) as isize;
    let antinode_y: isize = (to
        .coord
        .y
        .wrapping_sub(from.coord.y.wrapping_sub(to.coord.y))) as isize;

    if (antinode_x >= 0 && antinode_x <= x_bound as isize)
        && (antinode_y >= 0 && antinode_y <= y_bound as isize)
    {
        return Some(Coord {
            x: antinode_x as usize,
            y: antinode_y as usize,
        });
    }

    return None;
}
