#[derive(PartialEq, Debug)]
enum State {
    Increasing,
    Decreasing,
}

impl State {
    fn get(num1: i32, num2: i32) -> Option<Self> {
        let calc: i32 = num1 - num2;

        if calc > 0 && calc < 4 {
            return Some(State::Decreasing);
        } else if calc < 0 && calc > -4 {
            return Some(State::Increasing);
        } else {
            return None;
        }
    }
}

pub fn solve(input: &str) {
    let mut sum: i32 = 0;

    for line in input.split("\n") {
        let array: Vec<i32> = line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();

        match part2_check(&array) {
            Ok(_) => {
                sum += 1;
            },
            Err((_index1, _index2)) => {
                // ngl, this is probably the worst way to do it, but it works
                for i in 0..array.len() {
                    let mut array_clone: Vec<i32> = array.clone();

                    array_clone.remove(i);

                    if part2_check(&array_clone).is_ok() {
                        sum += 1;
                        break;
                    }
                }
            },
        };
    }

    println!["{}", sum];
}

fn part2_check(array: &[i32]) -> Result<(), (usize, usize)> {
    let mut prev_state: Option<State> = None;

    for (index, window) in array.windows(2).enumerate() {
        let state: State = match State::get(window[0], window[1]) {
            Some(state) => state,
            None => return Err((index, index + 1)),
        };

        if prev_state.is_none() {
            prev_state = Some(state);
        } else {
            if prev_state != Some(state) {
                return Err((index, index + 1));
            }
        }
    }

    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::part2_check;

    #[test]
    fn test_part2_check() {
        assert_eq![part2_check(&[7, 6, 4, 2, 1]), Ok(())];
        assert_eq![part2_check(&[1, 2, 7, 8, 9]), Err((1, 2))];
        assert_eq![part2_check(&[9, 7, 6, 2, 1]), Err((2, 3))];
        assert_eq![part2_check(&[1, 3, 2, 4, 5]), Err((1, 2))];
        assert_eq![part2_check(&[8, 6, 4, 4, 1]), Err((2, 3))];
        assert_eq![part2_check(&[1, 3, 6, 7, 9]), Ok(())];

        assert_eq![part2_check(&[1, 1, 6, 7, 9]), Err((0, 1))];
        assert_eq![part2_check(&[1, 2, 6, 7, 9]), Err((1, 2))];

        assert_eq![part2_check(&[35, 35, 38, 40, 40, 38]), Err((0, 1))];
        assert_eq![part2_check(&[35, 38, 40, 40, 38]), Err((2, 3))];

        assert_eq![part2_check(&[26, 26, 27, 27, 27]), Err((0, 1))];
        assert_eq![part2_check(&[26, 27, 27, 27]), Err((1, 2))];


        assert_eq![part2_check(&[1, 1, 1, 3, 7]), Err((0, 1))];
        assert_eq![part2_check(&[1, 1, 3, 7]), Err((0, 1))];

        assert_eq![part2_check(&[1, 1, 4, 4, 7, 10, 11, 18]), Err((0, 1))];
        assert_eq![part2_check(&[1, 4, 4, 7, 10, 11, 18]), Err((1, 2))];
        
        assert_eq![part2_check(&[25, 29, 31, 32, 34, 35]), Err((0, 1))];
        assert_eq![part2_check(&[29, 31, 32, 34, 35]), Ok(())];
        assert_eq![part2_check(&[25, 31, 32, 34, 35]), Err((0, 1))];
    }
}
