/// Day 9 - Part 1 : 57195069
///     generator: 32.927µs,
///     runner: 15.16952ms
///
/// Day 9 - Part 2 : 7409241
///     generator: 23.365µs,
///     runner: 67.567µs

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input.lines().filter_map(|line| line.parse().ok()).collect()
}

fn sum_preamble(preample: &[u32]) -> Vec<u32> {
    use itertools::Itertools;

    let mut res = Vec::new();
    res.reserve(preample.len() * preample.len());

    for num in preample.iter().permutations(2) {
        res.push(num[0] + num[1]);
    }

    res.sort();
    res.dedup();

    return res;
}

#[aoc(day9, part1)]
pub fn solver_part1(input: &Vec<u32>) -> Option<u32> {
    let len = input.len();
    for i in 0..len {
        #[cfg(not(test))]
        if i == len - 25 {
            break;
        } else {
            if !sum_preamble(&input[i..i + 25]).contains(&input[i + 25]) {
                return Some(input[i + 25]);
            }
        }
        #[cfg(test)]
        if i == len - 5 {
            break;
        } else {
            if !sum_preamble(&input[i..i + 5]).contains(&input[i + 5]) {
                return Some(input[i + 5]);
            }
        }
    }

    return None;
}
#[cfg(not(test))]
const TARGET: u32 = 57195069;
#[cfg(test)]
const TARGET: u32 = 127;

#[aoc(day9, part2)]
pub fn solver_part2(input: &Vec<u32>) -> Option<u32> {
    let len = input.len();

    let mut nums = Vec::new();
    let mut curr = 0;

    for i in 0..len {
        let mut pos = i;

        while curr < TARGET && pos <= len - 1 {
            nums.push(input[pos]);
            curr += input[pos];

            if curr == TARGET {
                nums.sort();
                return Some(nums.first().unwrap() + nums.last().unwrap());
            }

            pos += 1;
        }

        nums.clear();
        curr = 0;
    }

    return None;
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
";

    #[test]
    fn test_input_generator() {
        assert_eq!(
            (vec![
                35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277,
                309, 576
            ]),
            input_generator(TEST_INPUT)
        )
    }

    #[test]
    fn test_sum_preample() {
        let input = input_generator(TEST_INPUT);

        assert_eq!(
            vec![35, 40, 45, 50, 55, 60, 62, 67, 72, 82],
            sum_preamble(&input[0..5])
        )
    }

    #[test]
    fn test_part1() {
        let input = input_generator(TEST_INPUT);

        assert_eq!(Some(127), solver_part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = input_generator(TEST_INPUT);

        assert_eq!(Some(62), solver_part2(&input));
    }
}
