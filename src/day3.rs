/// Day 3 - Part 1 : 162
///     generator: 139.595µs,
///     runner: 902ns
///
/// Day 3 - Part 2 : 3064612320
///     generator: 112.422µs,
///     runner: 3.177µs

#[aoc_generator(day3)]
fn parse_input(input: &str) -> Vec<Vec<u8>> {
    let mut result: Vec<Vec<u8>> = Vec::new();

    for line in input.lines() {
        let mut inner: Vec<u8> = Vec::new();

        for cha in line.chars() {
            if cha == '.' {
                inner.push(0)
            } else if cha == '#' {
                inner.push(1)
            }
        }

        result.push(inner);
    }

    return result;
}

#[cfg(not(test))]
const LEN: usize = 31;
#[cfg(test)]
const LEN: usize = 11;

#[aoc(day3, part1)]
fn solver_part1(input: &Vec<Vec<u8>>) -> u32 {
    let mut x = 0;

    let mut res: u32 = 0;

    for line in input {
        res += line[x] as u32;

        if (x + 3) >= LEN {
            x = (x + 3) - LEN
        } else {
            x += 3
        }
    }

    return res;
}

#[aoc(day3, part2)]
fn solver_part2(input: &Vec<Vec<u8>>) -> u32 {
    let res1 = solver_part2_inner(&input, 1, 1);
    let res2 = solver_part2_inner(&input, 3, 1);
    let res3 = solver_part2_inner(&input, 5, 1);
    let res4 = solver_part2_inner(&input, 7, 1);
    let res5 = solver_part2_inner(&input, 1, 2);

    res1 * res2 * res3 * res4 * res5
}

fn solver_part2_inner(input: &Vec<Vec<u8>>, xstep: usize, ystep: usize) -> u32 {
    let mut y = 0;
    let mut x = 0;

    let mut res: u32 = 0;

    let ylen = input.len();

    loop {
        if y >= ylen {
            break;
        }

        res += input[y][x] as u32;

        if (x + xstep) >= LEN {
            x = (x + xstep) - LEN
        } else {
            x += xstep
        }

        y += ystep;
    }

    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
";

    #[test]
    fn test_parse_input() {
        let res: Vec<Vec<u8>> = vec![
            vec![0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0],
            vec![0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1],
            vec![0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0],
            vec![0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1],
            vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0],
            vec![1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1],
            vec![0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 1],
        ];

        assert_eq!(res, parse_input(TEST_INPUT));
    }

    #[test]
    fn test_find_trees() {
        let res: Vec<Vec<u8>> = vec![
            vec![0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0],
            vec![0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1],
            vec![0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0],
            vec![0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1],
            vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0],
            vec![1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1],
            vec![0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 1],
        ];

        assert_eq!(7, solver_part1(&res));
    }
}
