use std::rc::Rc;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Status {
    Error,
    Walking,
    ReachedEnd,
}

#[derive(Debug)]
struct PathWalker {
    map: Rc<Map>,
    cur_pos: Coord,
    cur_status: Status,
}

impl PathWalker {
    fn new(starting_pos: Coord, map: Rc<Map>) -> Self {
        return Self {
            map,
            cur_pos: starting_pos,
            cur_status: Status::Walking,
        };
    }

    fn walk(&mut self) -> Vec<Coord> {
        if self.cur_status != Status::Walking {
            return Vec::new();
        }

        let cur_num: u32 = self.map.get_num(self.cur_pos).unwrap(); // Unwrap is safe

        if cur_num == 9 {
            self.cur_status = Status::ReachedEnd;
            return Vec::new();
        }

        let mut possible_coords: Vec<Coord> = Vec::new();
        let expected_num: u32 = cur_num + 1;

        for i in [
            Coord::new(self.cur_pos.x, self.cur_pos.y.wrapping_sub(1)),
            Coord::new(self.cur_pos.x + 1, self.cur_pos.y),
            Coord::new(self.cur_pos.x, self.cur_pos.y + 1),
            Coord::new(self.cur_pos.x.wrapping_sub(1), self.cur_pos.y),
        ] {
            if let Some(step) = self.map.get_num(i) {
                if step == expected_num {
                    possible_coords.push(i);
                }
            }
        }

        if possible_coords.len() == 0 {
            self.cur_status = Status::Error;
            return Vec::new();
        } else {
            self.cur_pos = possible_coords[0];
            if self.map.get_num(self.cur_pos).unwrap() == 9 {
                self.cur_status = Status::ReachedEnd;
            }
            return possible_coords[1..].to_vec();
        }
    }
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<char>>,
}

impl Map {
    fn new(input: &str) -> Self {
        return Self {
            map: input.lines().map(|x| x.chars().collect()).collect(),
        };
    }

    fn get_num(&self, coords: Coord) -> Option<u32> {
        return self.map.get(coords.y)?.get(coords.x)?.to_digit(10);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        return Self { x, y };
    }
}

pub fn solve(input: &str) {
    let map: Rc<Map> = Rc::new(Map::new(input));

    let mut sum: usize = 0;
    for (cur_y, line) in map.map.iter().enumerate() {
        for (cur_x, char) in line.iter().enumerate() {
            if *char != '0' {
                continue;
            }

            let mut pathwalkers: Vec<PathWalker> = Vec::new();

            pathwalkers.push(PathWalker::new(Coord::new(cur_x, cur_y), map.clone()));

            loop {
                if pathwalkers.len() == 0 {
                    break;
                }

                let possible_coords = pathwalkers[0].walk();

                for possible_coord in possible_coords.iter() {
                    pathwalkers.push(PathWalker::new(*possible_coord, map.clone()));
                }

                if pathwalkers[0].cur_status != Status::Walking {
                    if pathwalkers[0].cur_status == Status::ReachedEnd {
                        sum += 1;
                    }

                    pathwalkers.remove(0);

                    continue;
                }
            }
        }
    }

    println!["Part 2: {}", sum];
}
