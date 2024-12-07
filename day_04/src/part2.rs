const SEARCH_STR1: &'static str = "MAS";
const SEARCH_STR2: &'static str = "SAM";

pub fn solve(input: &str) {
    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        matrix.push(line.chars().collect());
    }

    let mut sum: i32 = 0;

    for lines in matrix.windows(3) {
        sum += find(lines);
    }

    println!["Part 2: {}", sum];
}

fn find(matrix: &[Vec<char>]) -> i32 {
    let mut finds: i32 = 0;

    let se_calc_y = |y: usize, index: usize| y.overflowing_add(index).0;
    let se_calc_x = |x: usize, index: usize| x.overflowing_add(index).0;
    let sw_calc_y = |y: usize, index: usize| y.overflowing_add(index).0;
    let sw_calc_x = |x: usize, index: usize| x.overflowing_sub(index).0;

    for x in 0..matrix[0].len() {
        if check2(matrix, 0, x, se_calc_y, se_calc_x)
            && check2(matrix, 0, x + 2, sw_calc_y, sw_calc_x)
        {
            finds += 1;
        }
    }

    return finds;
}

fn check2(
    matrix: &[Vec<char>],
    y: usize,
    x: usize,
    calc_y: impl Fn(usize, usize) -> usize,
    calc_x: impl Fn(usize, usize) -> usize,
) -> bool {
    let search_strings: &[&str] = &[SEARCH_STR1, SEARCH_STR2];
    let mut strings_found: [bool; 2] = [true; 2];

    for (search_string_index, search_string) in search_strings.iter().enumerate() {
        for (search_char_index, search_char) in search_string.chars().enumerate() {
            let line: &Vec<char> = match matrix.get(calc_y(y, search_char_index)) {
                Some(line) => line,
                None => {
                    strings_found[search_string_index] = false;
                    break;
                }
            };

            let cur_char: char = match line.get(calc_x(x, search_char_index)) {
                Some(character) => *character,
                None => {
                    strings_found[search_string_index] = false;
                    break;
                }
            };

            if cur_char != search_char {
                strings_found[search_string_index] = false;
                break;
            }
        }
    }

    return strings_found[0] || strings_found[1];
}
