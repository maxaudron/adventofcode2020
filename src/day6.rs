/// Day 6 - Part 1 : 6799
///     generator: 281ns,
///     runner: 110.086µs
///
/// Day 6 - Part 2 : 3354
///     generator: 140ns,
///     runner: 111.79µs

fn get_group_answers(input: &str) -> [u8; 26] {
    let mut res: [u8; 26] = [0; 26];
    for cha in input.as_bytes() {
        if &b'\n' == cha {
            continue;
        } else {
            res[(cha - 97) as usize] = 1;
        }
    }

    return res;
}

#[aoc(day6, part1)]
fn get_sum_answers(input: &str) -> u32 {
    let mut res: u32 = 0;
    for group in input.split("\n\n") {
        res += get_group_answers(group).iter().sum::<u8>() as u32
    }

    return res;
}

fn get_group_answers_2(input: &str) -> u8 {
    let mut res: [u8; 26] = [0; 26];
    for cha in input.as_bytes() {
        if &b'\n' == cha {
            continue;
        } else {
            res[(cha - 97) as usize] += 1;
        }
    }

    let target = input.as_bytes().iter().filter(|&&c| c == b'\n').count() + 1;
    let sum = res.iter().filter(|&&c| c == target as u8).count();

    return sum as u8;
}

#[aoc(day6, part2)]
fn get_sum_answers_2(input: &str) -> u32 {
    let mut res: u32 = 0;
    for group in input.split("\n\n") {
        res += get_group_answers_2(group.trim_end()) as u32
    }

    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_group_answers() {
        let input = "abc\nabc";

        assert_eq!(3 as u8, get_group_answers(input).iter().sum());
    }
}
