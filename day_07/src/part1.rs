pub fn solve(input: &str) {
    let mut sum: i64 = 0;

    for line in input.lines() {
        let line_splitted: Vec<&str> = line.split(":").collect();

        let expected_result: i64 = line_splitted[0].parse().unwrap();
        let numbers: Vec<i64> = line_splitted[1]
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let mut operation_binary: u32 = 0;
        let gaps: usize = numbers.len() - 1;
        let possibilities: usize = 2usize.pow(gaps as u32);

        for _ in 0..possibilities {
            let operations: Vec<char> = create_possible_operations(operation_binary, gaps);
            operation_binary += 1;

            let mut cur_number: i64 = numbers[0];
            for index in 1..numbers.len() {
                cur_number = match operations[index - 1] {
                    '+' => cur_number + numbers[index],
                    '*' => cur_number * numbers[index],
                    _ => panic![],
                };
            }

            if cur_number == expected_result {
                sum += cur_number;
                break;
            }
        }
    }

    println!["Part 1: {}", sum];
}

fn create_possible_operations(operation_binary: u32, gaps: usize) -> Vec<char> {
    let mut operations: Vec<char> = Vec::with_capacity(gaps);

    for gap in 0..gaps {
        let cur_op = (operation_binary & (1 << gap)) >> gap;

        if cur_op == 0 {
            operations.push('+');
        } else if cur_op == 1 {
            operations.push('*');
        } else {
            panic!["cur_op too high"];
        }
    }

    return operations;
}
