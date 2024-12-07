// I knew I have to do this enum...
#[derive(PartialEq, Debug, Clone)]
enum Operation {
    Addition,
    Multiplication,
    Concatination,
}

impl Operation {
    fn upgrade(&mut self) -> bool {
        *self = match self {
            Operation::Addition => Operation::Multiplication,
            Operation::Multiplication => Operation::Concatination,
            Operation::Concatination => Operation::Addition,
        };

        return *self == Operation::Addition;
    }
}

fn upgrade_vec(operations: &mut Vec<Operation>) {
    for item in operations.iter_mut() {
        if !item.upgrade() {
            break;
        }
    }
}

pub fn solve(input: &str) {
    let mut sum: i64 = 0;

    for line in input.lines() {
        let line_splitted: Vec<&str> = line.split(":").collect();

        let expected_result: i64 = line_splitted[0].parse().unwrap();
        let numbers: Vec<i64> = line_splitted[1]
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let gaps: usize = numbers.len() - 1;
        let possibilities: usize = 3usize.pow(gaps as u32);
        let mut operations: Vec<Operation> = vec![Operation::Addition; gaps];

        for _ in 0..possibilities {
            let mut cur_number: i64 = numbers[0];
            for index in 1..numbers.len() {
                cur_number = match operations[index - 1] {
                    Operation::Addition => cur_number + numbers[index],
                    Operation::Multiplication => cur_number * numbers[index],
                    Operation::Concatination => {
                        // This gives the right answer, but is wayy to inefficient
                        // format!["{}{}", cur_number, numbers[index]].parse().unwrap()

                        let b_digits: u32 = (numbers[index] as f64).log10().floor() as u32 + 1;
                        cur_number * 10_i64.pow(b_digits) + numbers[index]
                    }
                };
            }

            if cur_number == expected_result {
                sum += cur_number;
                break;
            }

            upgrade_vec(&mut operations);
        }
    }

    println!["Part 2: {}", sum];
}
