/// Day 2 - Part 1 : 458
///     generator: 643.257µs,
///     runner: 147.253µs
///
/// Day 2 - Part 2 : 342
///     generator: 450.359µs,
///     runner: 60.31µs

#[derive(Debug, PartialEq)]
pub struct Policy {
    pub min: u8,
    pub max: u8,
    pub letter: char,
    pub password: String,
}

impl From<&str> for Policy {
    fn from(input: &str) -> Self {
        let split: Vec<&str> = input.split_ascii_whitespace().collect();
        let split_num: Vec<&str> = split[0].split('-').collect();

        Policy {
            min: split_num[0].parse::<u8>().unwrap(),
            max: split_num[1].parse::<u8>().unwrap(),
            letter: split[1].trim_end_matches('-').chars().nth(0).unwrap(),
            password: split[2].to_string(),
        }
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Policy> {
    input.lines().map(|line| Policy::from(line)).collect()
}

#[aoc(day2, part1)]
fn solver_part1(input: &Vec<Policy>) -> u32 {
    let mut count = 0;
    for i in input {
        let c = i.password.matches(i.letter).count();
        if !(c > i.max.into()) && !(c < i.min.into()) {
            count += 1;
        }
    }

    return count;
}

#[aoc(day2, part2)]
fn solver_part2(input: &Vec<Policy>) -> u32 {
    let mut count = 0;
    for i in input {
        if (i.password.chars().nth((i.max - 1).into()).unwrap() == i.letter)
            ^ (i.password.chars().nth((i.min - 1).into()).unwrap() == i.letter)
        {
            count += 1;
        }
    }

    return count;
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
";

    #[test]
    fn test_policy_parse() {
        let output: Vec<Policy> = vec![
            Policy {
                min: 1,
                max: 3,
                letter: 'a',
                password: "abcde".to_string(),
            },
            Policy {
                min: 1,
                max: 3,
                letter: 'b',
                password: "cdefg".to_string(),
            },
            Policy {
                min: 2,
                max: 9,
                letter: 'c',
                password: "ccccccccc".to_string(),
            },
        ];
        assert_eq!(output, input_generator(TEST_INPUT));
    }

    #[test]
    fn test_policy_count() {
        let input: Vec<Policy> = vec![
            Policy {
                min: 1,
                max: 3,
                letter: 'a',
                password: "abcde".to_string(),
            },
            Policy {
                min: 1,
                max: 3,
                letter: 'b',
                password: "cdefg".to_string(),
            },
            Policy {
                min: 2,
                max: 9,
                letter: 'c',
                password: "ccccccccc".to_string(),
            },
        ];
        assert_eq!(2, solver_part1(&input))
    }
}
