use regex::Regex;

pub fn solve(input: &str) {
    let mul_regex: Regex = Regex::new(r"mul\((?<num1>\d{1,3}),(?<num2>\d{1,3})\)").unwrap();

    let mut sum: i32 = 0;

    for mul in mul_regex.captures_iter(input) {
        sum += mul["num1"].parse::<i32>().unwrap() * mul["num2"].parse::<i32>().unwrap();
    }

    println!["Part 1: {}", sum];
}
