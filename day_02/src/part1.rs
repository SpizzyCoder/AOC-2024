#[derive(PartialEq, Debug)]
enum State {
    Increasing,
    Decreasing,
}

pub fn solve(input: &str) {
    let mut sum: i32 = 0;

    for line in input.lines() {
        let array: Vec<i32> = line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();

        let state: State;
        if (array[0] - array[1]) < 0 {
            state = State::Increasing;
        } else if (array[0] - array[1]) > 0 {
            state = State::Decreasing;
        } else {
            continue;
        }

        let mut valid: bool = true;
        for window in array.windows(2) {
            let calc: i32 = window[0] - window[1];

            if state == State::Increasing && (calc < -3 || calc > -1) {
                valid = false;
                break;
            } else if state == State::Decreasing && (calc > 3 || calc < 1) {
                valid = false;
                break;
            }
        }

        if valid {
            sum += 1;
        }
    }

    println!["Part 1: {}", sum];
}
