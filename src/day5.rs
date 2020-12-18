/// Day 5 - Part 1 : 871
///     generator: 79.405µs,
///     runner: 1.844µs
///
/// Day 5 - Part 2 : 640
///     generator: 71.326µs,
///     runner: 762ns

#[aoc_generator(day5)]
fn get_ids(input: &str) -> Vec<u32> {
    let mut res = Vec::new();
    for line in input.lines() {
        res.push(get_id(line.as_bytes()));
    }

    res.sort();
    return res;
}

#[aoc(day5, part1)]
fn get_highest_id(input: &Vec<u32>) -> u32 {
    let mut input = input.clone();
    input.reverse();
    return input[0];
}

#[aoc(day5, part2)]
fn get_free_seat(input: &Vec<u32>) -> u32 {
    let mut pos = 1;

    loop {
        if input[pos - 1] == input[pos] - 1 && input[pos + 1] == input[pos] + 1 {
            pos += 2;
            continue;
        } else {
            if !input.contains(&(input[pos] - 1)) {
                return input[pos] - 1;
            } else if !input.contains(&(input[pos] + 1)) {
                return input[pos] + 1;
            }
        }
    }
}

fn get_id(line: &[u8]) -> u32 {
    return (get_row(&line[..7], 0, 127) as u32 * 8) + get_column(&line[7..], 0, 7) as u32;
}

fn get_row(op: &[u8], lower: u8, upper: u8) -> u8 {
    match op {
        [b'F', tail @ ..] => get_row(tail, lower, lower + ((upper - lower) / 2)),
        [b'B', tail @ ..] => get_row(tail, lower + ((upper - lower) / 2) + 1, upper),
        _ => return lower,
    }
}

fn get_column(op: &[u8], lower: u8, upper: u8) -> u8 {
    match op {
        [b'L', tail @ ..] => get_column(tail, lower, lower + ((upper - lower) / 2)),
        [b'R', tail @ ..] => get_column(tail, lower + ((upper - lower) / 2) + 1, upper),
        _ => return lower,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_row() {
        assert_eq!(44, get_row(b"FBFBBFF", 0, 127));
        assert_eq!(70, get_row(b"BFFFBBF", 0, 127));
        assert_eq!(14, get_row(b"FFFBBBF", 0, 127));
        assert_eq!(102, get_row(b"BBFFBBF", 0, 127));
    }

    #[test]
    fn test_get_column() {
        assert_eq!(5, get_column(b"RLR", 0, 7));
        assert_eq!(7, get_column(b"RRR", 0, 7));
        assert_eq!(7, get_column(b"RRR", 0, 7));
        assert_eq!(4, get_column(b"RLL", 0, 7));
    }

    #[test]
    fn test_get_id() {
        assert_eq!(357, get_id(b"FBFBBFFRLR"));
        assert_eq!(567, get_id(b"BFFFBBFRRR"));
        assert_eq!(119, get_id(b"FFFBBBFRRR"));
        assert_eq!(820, get_id(b"BBFFBBFRLL"));
    }
}
