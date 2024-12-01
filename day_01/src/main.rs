use std::fs;

fn main() {
    let input: String = fs::read_to_string("./input.txt").unwrap();

    // part1(&input);
    part2(&input);
}

fn part2(input: &str) {
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for line in input.split("\n") {
        let line_splitted: Vec<&str> = line.split("   ").collect();

        left_list.push(line_splitted[0].parse().unwrap());
        right_list.push(line_splitted[1].parse().unwrap());
    }

    let mut sum: i32 = 0;
    for num in left_list.iter() {
        sum += ((right_list.iter().filter(|&&x| x == *num).count()) as i32 * *num) as i32;
    }

    println!["{}", sum];
}

fn part1(input: &str) {
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for line in input.split("\n") {
        let line_splitted: Vec<&str> = line.split("   ").collect();

        left_list.push(line_splitted[0].parse().unwrap());
        right_list.push(line_splitted[1].parse().unwrap());
    }

    left_list.sort_unstable();
    right_list.sort_unstable();

    let mut sum: u32 = 0;
    for (l, r) in left_list.iter().zip(right_list.iter()) {
        println!["{} {} = {}", l, r, l.abs_diff(*r)];
        sum += l.abs_diff(*r);
    }

    println!["{}", sum];
}
