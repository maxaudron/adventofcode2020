impl Adapter for u64 {
    fn diff(self: &Self, next: &u64) -> u64 {
        match next - self {
            1 | 2 | 3 => return next - self,
            _ => unimplemented!(),
        }
    }
}

trait Adapter {
    fn diff(self: &Self, next: &u64) -> u64;
}

#[aoc_generator(day10)]
fn input_generator(input: &str) -> Vec<u64> {
    let mut input = input
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect::<Vec<u64>>();

    input.sort();
    input
}

#[aoc(day10, part1)]
fn solver_part1(input: &Vec<u64>) -> u64 {
    let diff = input.iter().fold((0, 0, 0, 0), |mut acc, x| {
        match acc.0.diff(x) {
            1 => acc.1 += 1,
            2 => acc.2 += 1,
            3 => acc.3 += 1,
            _ => unimplemented!(),
        }
        acc.0 = *x;
        return acc;
    });

    return diff.1 * (diff.3 + 1);
}

#[aoc(day10, part2)]
fn solver_part2(input: &Vec<u64>) -> u64 {
    let (_, mut diff): (_, Vec<u64>) =
        input
            .iter()
            .fold((0, Vec::with_capacity(input.len())), |mut acc, x| {
                acc.1.push(acc.0.diff(x));
                acc.0 = *x;
                return acc;
            });

    diff.insert(0, input[0]);
    diff.push(3);
    diff.remove(0);

    let (res, _) = diff.iter().fold((1, 0), |(permutations, ones), diff| {
        if *diff == 1 {
            (permutations, ones + 1)
        } else {
            match ones {
                0 => (permutations, 0),
                1 => (permutations, 0),
                2 => (permutations * 2, 0),
                3 => (permutations * 4, 0),
                _ => (permutations * ((1 << (ones - 1)) - 1), 0),
            }
        }
    });

    return res;
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: [u64; 11] = [1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19];
    const TEST_INPUT2: [u64; 31] = [
        1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31, 32, 33, 34, 35, 38,
        39, 42, 45, 46, 47, 48, 49,
    ];

    #[test]
    fn test_solver_part1() {
        assert_eq!(7 * 5, solver_part1(&TEST_INPUT.to_vec()));
    }

    #[test]
    fn test_solver_part2() {
        assert_eq!(8, solver_part2(&TEST_INPUT.to_vec()));
    }

    #[test]
    fn test_solver_part2_2() {
        assert_eq!(19208, solver_part2(&TEST_INPUT2.to_vec()));
    }
}
