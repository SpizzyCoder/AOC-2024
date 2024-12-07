pub fn solve(input: &str) {
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for line in input.lines() {
        let line_splitted: Vec<&str> = line.split("   ").collect();

        left_list.push(line_splitted[0].parse().unwrap());
        right_list.push(line_splitted[1].parse().unwrap());
    }

    let mut sum: i32 = 0;
    for num in left_list.iter() {
        sum += ((right_list.iter().filter(|&&x| x == *num).count()) as i32 * *num) as i32;
    }

    println!["Part 2: {}", sum];
}
