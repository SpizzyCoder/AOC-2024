#[derive(Debug)]
struct Number {
    number: i64,
    amount: usize,
}

pub fn solve(input: &str) {
    let mut numbers: Vec<Number> = input
        .trim()
        .split(" ")
        .map(|x| Number { number: x.parse().unwrap(), amount: 1 })
        .collect();

    for _ in 0..75 {
        let mut new_numbers: Vec<Number> = Vec::new();

        for cur_number in numbers.iter_mut() {
            if cur_number.number == 0 {
                new_numbers.push(Number { number: 1, amount: cur_number.amount });
                cur_number.amount = 0;
                continue;
            }

            let digits: i32 = count_digits(cur_number.number);
            if digits % 2 == 0 {
                let split_number: (i64, i64) = split_digit(cur_number.number, digits);

                new_numbers.push(Number { number: split_number.0, amount: cur_number.amount });
                new_numbers.push(Number { number: split_number.1, amount: cur_number.amount });
                cur_number.amount = 0;
                continue;
            }

            new_numbers.push(Number { number: cur_number.number * 2024, amount: cur_number.amount });
            cur_number.amount = 0;
        }

        // The numbers with the amount 0 are processed here unnecessarily, but that doesn't matter
        // Better this than to filter the whole array before this processing
        for cur_index in 0..new_numbers.len() {
            for check_index in (cur_index + 1)..new_numbers.len() {
                if new_numbers[cur_index].number == new_numbers[check_index].number {
                    new_numbers[cur_index].amount += new_numbers[check_index].amount;
                    new_numbers[check_index].amount = 0;
                }
            }
        }
        
        numbers = new_numbers.into_iter().filter(|x| x.amount != 0).collect();
    }

    let mut sum: usize = 0;
    for i in numbers.iter() {
        sum += i.amount;
    }
    println!["{}", sum];
}

fn count_digits(n: i64) -> i32 {
    let mut count = 0;
    let mut num = n;

    while num > 0 {
        num /= 10;
        count += 1;
    }

    return count;
}

fn split_digit(n: i64, digits: i32) -> (i64, i64) {
    let half = digits / 2;

    let divider = 10i64.pow(half as u32);

    let left = n / divider;
    let right = n % divider;

    return (left, right);
}

#[cfg(test)]
mod tests {
    use crate::part2::{count_digits, split_digit};

    #[test]
    fn test_count_digits() {
        assert_eq!(count_digits(1), 1);
        assert_eq!(count_digits(12), 2);
        assert_eq!(count_digits(123), 3);
        assert_eq!(count_digits(1234), 4);
        assert_eq!(count_digits(12345), 5);
        assert_eq!(count_digits(123456), 6);
        assert_eq!(count_digits(1234567), 7);
        assert_eq!(count_digits(12345678), 8);
        assert_eq!(count_digits(123456789), 9);
        assert_eq!(count_digits(1234567890), 10);
        assert_eq!(count_digits(12345678901), 11);
    }

    #[test]
    fn test_split_digits() {
        assert_eq!(split_digit(12, 2), (1, 2));
        assert_eq!(split_digit(1234, 4), (12, 34));
        assert_eq!(split_digit(123456, 6), (123, 456));
        assert_eq!(split_digit(12345678, 8), (1234, 5678));
    }
}
