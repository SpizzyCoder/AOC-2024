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

        if line_ok {
            sum += updateset[updateset.len() / 2];
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
