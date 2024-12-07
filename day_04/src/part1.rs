const SEARCH_STR: &'static str = "XMAS";

pub fn solve(input: &str) {
    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        matrix.push(line.chars().collect());
    }

    let mut sum: i32 = 0;
    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            sum += find(&matrix, y, x);
        }
    }

    println!["Part 1: {}", sum];
}

fn check2(
    matrix: &Vec<Vec<char>>,
    y: usize,
    x: usize,
    calc_y: impl Fn(usize, usize) -> usize,
    calc_x: impl Fn(usize, usize) -> usize,
) -> bool {
    for (index, search_char) in SEARCH_STR.chars().enumerate() {
        let line: &Vec<char> = match matrix.get(calc_y(y, index)) {
            Some(line) => line,
            None => return false,
        };

        let cur_char: char = match line.get(calc_x(x, index)) {
            Some(character) => *character,
            None => return false,
        };

        if cur_char != search_char {
            return false;
        }
    }

    return true;
}

fn find(matrix: &Vec<Vec<char>>, y: usize, x: usize) -> i32 {
    let mut finds: i32 = 0;

    let forward_calc_y = |y: usize, _index: usize| y;
    let forward_calc_x = |x: usize, index: usize| x.overflowing_add(index).0;
    let backward_calc_y = |y: usize, _index: usize| y;
    let backward_calc_x = |x: usize, index: usize| x.overflowing_sub(index).0;
    let ne_calc_y = |y: usize, index: usize| y.overflowing_sub(index).0;
    let ne_calc_x = |x: usize, index: usize| x.overflowing_add(index).0;
    let up_calc_y = |y: usize, index: usize| y.overflowing_sub(index).0;
    let up_calc_x = |x: usize, _index: usize| x;
    let nw_calc_y = |y: usize, index: usize| y.overflowing_sub(index).0;
    let nw_calc_x = |x: usize, index: usize| x.overflowing_sub(index).0;
    let sw_calc_y = |y: usize, index: usize| y.overflowing_add(index).0;
    let sw_calc_x = |x: usize, index: usize| x.overflowing_sub(index).0;
    let down_calc_y = |y: usize, index: usize| y.overflowing_add(index).0;
    let down_calc_x = |x: usize, _index: usize| x;
    let se_calc_y = |y: usize, index: usize| y.overflowing_add(index).0;
    let se_calc_x = |x: usize, index: usize| x.overflowing_add(index).0;

    if check2(matrix, y, x, forward_calc_y, forward_calc_x) {
        finds += 1
    }
    if check2(matrix, y, x, backward_calc_y, backward_calc_x) {
        finds += 1
    }
    if check2(matrix, y, x, ne_calc_y, ne_calc_x) {
        finds += 1
    }
    if check2(matrix, y, x, up_calc_y, up_calc_x) {
        finds += 1
    }
    if check2(matrix, y, x, nw_calc_y, nw_calc_x) {
        finds += 1
    }
    if check2(matrix, y, x, sw_calc_y, sw_calc_x) {
        finds += 1
    }
    if check2(matrix, y, x, down_calc_y, down_calc_x) {
        finds += 1
    }
    if check2(matrix, y, x, se_calc_y, se_calc_x) {
        finds += 1
    }

    return finds;
}
