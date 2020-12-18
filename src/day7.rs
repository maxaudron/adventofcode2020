/// Day 7 - Part 1 : 144
///     generator: 4.693143ms,
///     runner: 452.483µs
///
/// Day 7 - Part 2 : 5956
///     generator: 3.749036ms,
///     runner: 88.536µs
use regex::Regex;

#[derive(Debug, PartialEq)]
struct BagRule {
    bag: String,
    num: u32,
    content: Option<String>,
}

#[aoc_generator(day7)]
fn input_generator(input: &str) -> Vec<BagRule> {
    let wre = Regex::new(r"(\d*) *(\w* \w*) bags*\.*").unwrap();

    let mut bags = Vec::new();

    for line in input.lines() {
        parse_bag(line, &wre, &mut bags);
    }

    bags
}

#[aoc(day7, part2)]
fn part2(bags: &Vec<BagRule>) -> String {
    get_num_needed(bags, "shiny gold").to_string()
}

fn get_num_needed(bags: &Vec<BagRule>, needed: &str) -> u32 {
    let mut res = 0;

    let mut bag: Vec<&BagRule> = bags.iter().filter(|bag| bag.bag == needed).collect();

    for i in bag {
        match &i.content {
            Some(content) => res += (get_num_needed(bags, &content) * i.num) + i.num,
            None => res += 1 * i.num,
        }
    }

    return res;
}

#[aoc(day7, part1)]
fn part1(bags: &Vec<BagRule>) -> usize {
    let mut res = find_containing(&bags, "shiny gold");
    res.sort();
    res.dedup();
    res.len()
}

fn find_containing<'a>(bags: &'a Vec<BagRule>, search: &str) -> Vec<&'a str> {
    let mut res: Vec<&str> = Vec::new();

    res.extend(
        bags.iter()
            .filter(|bag| bag.content.is_some() && bag.content.as_ref().unwrap() == search)
            .map(|bag| bag.bag.as_str()),
    );

    let mut res2: Vec<&str> = Vec::new();

    for i in &res {
        res2.append(&mut find_containing(bags, i));
    }

    res.append(&mut res2);

    return res;
}

fn parse_bag<'a>(input: &'a str, wre: &Regex, bags: &mut Vec<BagRule>) {
    let mut input = input.split(" contain ");
    let name = wre
        .captures(input.next().unwrap())
        .unwrap()
        .get(2)
        .unwrap()
        .as_str();
    for i in input.collect::<Vec<&str>>()[0].split(", ") {
        match wre.captures(i).unwrap().get(1).unwrap().as_str().parse() {
            Ok(a) => {
                bags.push(BagRule {
                    bag: name.to_string(),
                    num: a,
                    content: Some(
                        wre.captures(i)
                            .unwrap()
                            .get(2)
                            .unwrap()
                            .as_str()
                            .to_string(),
                    ),
                });
            }
            _ => {
                bags.push(BagRule {
                    bag: name.to_string(),
                    num: 0,
                    content: None,
                });
            }
        }
    }
}

const TEST_INPUT: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
";
