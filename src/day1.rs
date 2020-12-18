/// Day 1 - Part 1 - base : 866436
///     generator: 27.655µs,
///     runner: 11.637µs
///
/// Day 1 - Part 1 - optimised : 866436
///     generator: 6.385µs,
///     runner: 11.336µs
///
/// Day 1 - Part 2 - base : 276650720
///     generator: 6.154µs,
///     runner: 4.030702ms
///
/// Day 1 - Part 2 - optimised : 276650720
///     generator: 2.956µs,
///     runner: 13.372µs

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input.lines().filter_map(|line| line.parse().ok()).collect()
}

#[aoc(day1, part1, base)]
fn get_2020_num(nums: &[u32]) -> Option<u32> {
    for x in nums {
        for y in nums {
            if x + y == 2020 {
                return Some(x * y);
            }
        }
    }

    None
}

#[aoc(day1, part1, optimised)]
fn get_2020_num_opt(nums: &[u32]) -> Option<u32> {
    let mut vec = nums.to_vec();
    vec.sort();

    let mut vec_rev = vec.clone();
    vec_rev.reverse();

    for x in vec {
        for y in &vec_rev {
            if x + y == 2020 {
                return Some(x * y);
            }
        }
    }

    None
}

#[aoc(day1, part2, base)]
fn solver_part2_base(nums: &[u32]) -> Option<u32> {
    for x in nums {
        for y in nums {
            for z in nums {
                if x + y + z == 2020 {
                    return Some(x * y * z);
                }
            }
        }
    }

    None
}

#[aoc(day1, part2, optimised)]
fn solver_part2_optimised(nums: &[u32]) -> Option<u32> {
    let mut vec = nums.to_vec();
    vec.sort();

    let mut vec_rev = vec.clone();
    vec_rev.reverse();

    for x in &vec {
        for y in &vec_rev {
            if x + y >= 2020 {
                continue;
            }
            for z in &vec_rev {
                if x + y + z == 2020 {
                    return Some(x * y * z);
                } else if x + y + z >= 2020 {
                    continue;
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn test_2020_num() {
    //     let nums: [u32; 6] = [1721, 979, 366, 299, 675, 1456];

    //     assert_eq!((1721, 299), get_2020_num(&nums).unwrap());
    // }

    // #[test]
    // fn test_2020_num_opt() {
    //     let nums: [u32; 6] = [1721, 979, 366, 299, 675, 1456];

    //     assert_eq!((299, 1721), get_2020_num_opt(&nums).unwrap());
    // }

    // #[bench]
    // // test tests::bench_2020_num ... bench:       7,116 ns/iter (+/- 395)
    // fn bench_2020_num(b: &mut Bencher) {
    //     b.iter(|| {
    //         get_2020_num(&test::black_box(NUMS)).unwrap();
    //     })
    // }

    // #[bench]
    // fn bench_2020_num_opt(b: &mut Bencher) {
    //     b.iter(|| {
    //         get_2020_num_opt(&test::black_box(NUMS)).unwrap();
    //     })
    // }
    //
    //

    // #[test]
    // fn test_2020_num() {
    //     let nums: [u32; 6] = [1721, 979, 366, 299, 675, 1456];

    //     assert_eq!((979, 366, 675), get_2020_num(&nums).unwrap());
    // }

    // #[test]
    // fn test_2020_num_opt() {
    //     let nums: [u32; 6] = [1721, 979, 366, 299, 675, 1456];

    //     assert_eq!((366, 979, 675), get_2020_num_opt(&nums).unwrap());
    // }

    // #[bench]
    // // test tests::bench_2020_num ... bench:       7,116 ns/iter (+/- 395)
    // fn bench_2020_num(b: &mut Bencher) {
    //     b.iter(|| {
    //         get_2020_num(&test::black_box(NUMS)).unwrap();
    //     })
    // }

    // #[bench]
    // fn bench_2020_num_opt(b: &mut Bencher) {
    //     b.iter(|| {
    //         get_2020_num_opt(&test::black_box(NUMS)).unwrap();
    //     })
    // }
}
