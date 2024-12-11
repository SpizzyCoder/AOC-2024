pub fn solve(input: &str) {
    let mut numbers: Vec<i64> = input
        .trim()
        .split(" ")
        .map(|x| x.parse().unwrap())
        .collect();

    for _ in 0..25 {
        let mut new_numbers: Vec<i64> = Vec::new();

        for index in 0..numbers.len() {
            if numbers[index] == 0 {
                new_numbers.push(1);
            } else if format!["{}", numbers[index]].len() % 2 == 0 {
                let number_as_string = format!["{}", numbers[index]];
                let number1: i64 = number_as_string[0..number_as_string.len() / 2]
                    .parse()
                    .unwrap();
                let number2: i64 = number_as_string[number_as_string.len() / 2..]
                    .parse()
                    .unwrap();

                new_numbers.push(number1);
                new_numbers.push(number2);
            } else {
                new_numbers.push(numbers[index] * 2024);
            }
        }

        numbers = new_numbers;
    }

    println!["{}", numbers.len()];
}
