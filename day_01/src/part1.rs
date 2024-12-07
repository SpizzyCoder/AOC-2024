pub fn solve(input: &str) {
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for line in input.lines() {
        let line_splitted: Vec<&str> = line.split("   ").collect();

        left_list.push(line_splitted[0].parse().unwrap());
        right_list.push(line_splitted[1].parse().unwrap());
    }

    left_list.sort_unstable();
    right_list.sort_unstable();

    let mut sum: u32 = 0;
    for (l, r) in left_list.iter().zip(right_list.iter()) {
        sum += l.abs_diff(*r);
    }

    println!["Part 1: {}", sum];
}
