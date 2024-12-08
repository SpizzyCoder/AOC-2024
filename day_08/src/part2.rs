#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct Antenna {
    identifier: char,
    coord: Coord,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
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
                let mut antinodes_append: Vec<Coord> =
                    calc_antinode(antenna.coord, cur_antenna.coord, x_bound, y_bound);

                antinodes.push(cur_antenna.coord);
                antinodes.push(antenna.coord);

                antinodes.append(&mut antinodes_append);
            }
        }
    }

    antinodes.sort_unstable();
    antinodes.dedup();

    println!["Part 2: {}", antinodes.len()];
}

fn calc_antinode(mut from: Coord, mut to: Coord, x_bound: usize, y_bound: usize) -> Vec<Coord> {
    let mut antennas: Vec<Coord> = Vec::new();

    loop {
        let antinode_x: isize = (to.x.wrapping_sub(from.x.wrapping_sub(to.x))) as isize;
        let antinode_y: isize = (to.y.wrapping_sub(from.y.wrapping_sub(to.y))) as isize;

        if (antinode_x >= 0 && antinode_x <= x_bound as isize)
            && (antinode_y >= 0 && antinode_y <= y_bound as isize)
        {
            antennas.push(Coord {
                x: antinode_x as usize,
                y: antinode_y as usize,
            });
        } else {
            break;
        }

        from = to;
        to = Coord {
            x: antinode_x as usize,
            y: antinode_y as usize,
        };
    }

    return antennas;
}
