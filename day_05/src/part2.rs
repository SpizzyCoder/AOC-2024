pub fn solve(input: &str) {
    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    let mut parse_rules: bool = true;
    for line in input.lines() {
        if parse_rules {
            if line.is_empty() {
                parse_rules = false;
                continue;
            }

            let splitted_line: Vec<i32> = line
                .split("|")
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

            rules.push((splitted_line[0], splitted_line[1]));
        } else {
            updates.push(line.split(",").map(|x| x.parse::<i32>().unwrap()).collect());
        }
    }

    let mut sum: i32 = 0;
    for updateset in updates.iter() {
        let mut line_ok: bool = true;

        for (index, update) in updateset.iter().enumerate() {
            if !check(&rules, &updateset[0..index], *update) {
                line_ok = false;
                break;
            }
        }

        if !line_ok {
            let correct_array: Vec<i32> = fix(&rules, &updateset);
            sum += correct_array[updateset.len() / 2];
        }
    }

    println!["{}", sum];
}

fn check(rules: &[(i32, i32)], before: &[i32], current: i32) -> bool {
    for i in before {
        if !rules.contains(&(*i, current)) {
            return false;
        }
    }

    return true;
}

fn fix(rules: &[(i32, i32)], faulty_array: &[i32]) -> Vec<i32> {
    let mut fixed_array: Vec<i32> = Vec::new();

    for num in faulty_array.iter() {
        let mut insert_pos: usize = 0;

        for fixed_num in fixed_array.iter() {
            if rules.contains(&(*fixed_num, *num)) {
                insert_pos += 1;
            } else {
                break;
            }
        }

        fixed_array.insert(insert_pos, *num);
    }

    return fixed_array;
}
