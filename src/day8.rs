/// Day 8 - Part 1 : 1859
///     generator: 121.102µs,
///     runner: 20.387µs
///
/// Day 8 - Part 2 : 1235
///     generator: 99.782µs,
///     runner: 776.507µs

#[derive(Debug, Clone)]
pub enum Op {
    NOP,
    ACC,
    JMP,
}

impl From<&str> for Op {
    fn from(input: &str) -> Self {
        match input {
            "nop" => Self::NOP,
            "acc" => Self::ACC,
            "jmp" => Self::JMP,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Instruction {
    op: Op,
    value: i32,
}

impl Instruction {
    fn exec(self: &Self, accumulator: &mut i32) -> i32 {
        match self.op {
            Op::NOP => return 1,
            Op::ACC => {
                *accumulator += self.value;
                return 1;
            }
            Op::JMP => return self.value,
        }
    }
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        let splits: Vec<&str> = input.split_whitespace().collect();
        return Instruction {
            op: Op::from(splits[0]),
            value: splits[1].parse().unwrap(),
        };
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| Instruction::from(line)).collect()
}

#[aoc(day8, part1)]
pub fn solver_part1(input: &Vec<Instruction>) -> i32 {
    let mut acc = 0;
    let mut pos = 0;
    let mut past_pos: Vec<usize> = Vec::new();

    while pos < input.len() {
        if past_pos.contains(&pos) {
            break;
        }
        past_pos.push(pos);
        pos = (pos as i32 + input[pos].exec(&mut acc)) as usize;
    }

    return acc;
}

fn is_broken(input: &Vec<Instruction>) -> bool {
    let mut acc = 0;
    let mut pos = 0;
    let mut past_pos: Vec<usize> = Vec::new();

    while pos < input.len() {
        if past_pos.contains(&pos) {
            return true;
        }
        past_pos.push(pos);
        pos = (pos as i32 + input[pos].exec(&mut acc)) as usize;
    }

    false
}

#[aoc(day8, part2)]
pub fn solver_part2(input: &Vec<Instruction>) -> i32 {
    let mut acc = 0;
    let mut pos = 0;
    let mut past_pos: Vec<usize> = Vec::new();

    while pos < input.len() {
        if past_pos.contains(&pos) {
            break;
        }
        past_pos.push(pos);
        pos = (pos as i32 + input[pos].exec(&mut acc)) as usize;
    }

    past_pos.reverse();

    for pos in past_pos {
        let mut input = (*input).clone();
        match input[pos].op {
            Op::JMP => {
                input[pos] = Instruction {
                    op: Op::NOP,
                    value: input[pos].value,
                }
            }
            Op::NOP => {
                input[pos] = Instruction {
                    op: Op::JMP,
                    value: input[pos].value,
                }
            }
            _ => continue,
        }

        if is_broken(&input) {
            continue;
        } else {
            let mut acc = 0;
            let mut pos = 0;

            while pos < input.len() {
                pos = (pos as i32 + input[pos].exec(&mut acc)) as usize;
            }

            return acc;
        }
    }

    return 0;
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
";

    use super::*;

    #[test]
    fn test_part1() {
        let inst = input_generator(TEST_INPUT);
        assert_eq!(5, solver_part1(&inst));
    }

    #[test]
    fn test_part2() {
        let inst = input_generator(TEST_INPUT);
        assert_eq!(8, solver_part2(&inst));
    }
}
