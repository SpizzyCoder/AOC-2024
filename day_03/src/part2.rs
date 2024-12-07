use regex::Regex;

pub fn solve(input: &str) {
    let mul_regex: Regex =
        Regex::new(r"(?<statement>(mul\((?<num1>\d{1,3}),(?<num2>\d{1,3})\))|do\(\)|don't\(\))")
            .unwrap();

    let mut sum: i32 = 0;
    let mut enabled: bool = true;

    for mul in mul_regex.captures_iter(input) {
        if mul["statement"].starts_with("mul") {
            if enabled {
                sum += mul["num1"].parse::<i32>().unwrap() * mul["num2"].parse::<i32>().unwrap();
            }
        } else if &mul["statement"] == "do()" {
            enabled = true;
        } else if &mul["statement"] == "don't()" {
            enabled = false;
        } else {
            panic![];
        }
    }

    println!["Part 2: {}", sum];
}
