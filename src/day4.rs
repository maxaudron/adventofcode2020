/// Day 4 - Part 1 : 222
///     generator: 250ns,
///     runner: 410.846µs
///
/// Day 4 - Part 2 : 140
///     generator: 111ns,
///     runner: 321.789µs
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
pub struct Passport {
    birth_year: u32,
    issue_year: u32,
    expiration_year: u32,
    height: String,
    hair_color: String,
    eye_color: EyeColor,
    passport_id: [u8; 9],
    country_id: Option<u32>,
}

#[derive(Debug, PartialEq)]
pub enum EyeColor {
    AMB,
    BLU,
    BRN,
    GRY,
    GRN,
    HZL,
    OTH,
    NONE,
}

impl Passport {
    fn default() -> Passport {
        return Passport {
            birth_year: 0,
            issue_year: 0,
            expiration_year: 0,
            height: String::new(),
            hair_color: String::new(),
            eye_color: EyeColor::NONE,
            passport_id: [0; 9],
            country_id: None,
        };
    }

    fn valid(&self) -> bool {
        return !(self.birth_year == 0
            || self.issue_year == 0
            || self.expiration_year == 0
            || self.height == ""
            || self.hair_color == ""
            || self.eye_color == EyeColor::NONE
            || self.passport_id == [0; 9]);
    }

    fn part1(input: &str) -> Option<Self> {
        let mut buf: Vec<u8> = Vec::new();
        let mut field: Vec<u8> = Vec::new();
        let mut value: Vec<u8> = Vec::new();
        field.reserve(3);
        value.reserve(32);

        let mut passport: Passport = Passport::default();

        for cha in input.as_bytes() {
            // :
            if cha == &0x3a {
                field = buf.clone();
                buf.clear()
            // \n or " "
            } else if cha == &0x0A || cha == &0x20 {
                value = buf.clone();
                if value == [] {
                    break;
                }
                buf.clear();

                match &field[..] {
                    //    byr (Birth Year)
                    [0x62, 0x79, 0x72] => {
                        passport.birth_year = std::str::from_utf8(&value).unwrap().parse().unwrap()
                    }
                    //    iyr (Issue Year)
                    [0x69, 0x79, 0x72] => {
                        passport.issue_year = std::str::from_utf8(&value).unwrap().parse().unwrap()
                    }
                    //    eyr (Expiration Year)
                    [0x65, 0x79, 0x72] => {
                        passport.expiration_year =
                            std::str::from_utf8(&value).unwrap().parse().unwrap()
                    }
                    //    hgt (Height)
                    [0x68, 0x67, 0x74] => {
                        passport.height = std::str::from_utf8(&value).unwrap().to_string()
                    }
                    //    hcl (Hair Color)
                    [0x68, 0x63, 0x6c] => {
                        passport.hair_color = std::str::from_utf8(&value).unwrap().to_string()
                    }
                    //    ecl (Eye Color)
                    [0x65, 0x63, 0x6c] => passport.eye_color = EyeColor::AMB,
                    //    pid (Passport ID)
                    [0x70, 0x69, 0x64] => passport.passport_id = [2; 9],
                    //    cid (Country ID)
                    [0x63, 0x69, 0x64] => {
                        passport.country_id =
                            Some(std::str::from_utf8(&value).unwrap().parse().unwrap())
                    }
                    _ => {
                        unimplemented!()
                    }
                }
            } else {
                buf.push(*cha)
            }
        }

        if passport.valid() {
            return Some(passport);
        } else {
            return None;
        }
    }

    fn part2(input: &str) -> Option<Self> {
        let mut buf: Vec<u8> = Vec::new();
        let mut field: Vec<u8> = Vec::new();
        let mut value: Vec<u8> = Vec::new();
        field.reserve(3);
        value.reserve(32);

        let mut passport: Passport = Passport::default();

        for cha in input.as_bytes() {
            // :
            if cha == &0x3a {
                field = buf.clone();
                buf.clear()
            // \n or " "
            } else if cha == &0x0A || cha == &0x20 {
                value = buf.clone();
                if value == [] {
                    break;
                }
                buf.clear();

                match &field[..] {
                    //    byr (Birth Year)
                    [0x62, 0x79, 0x72] => {
                        passport.birth_year = std::str::from_utf8(&value).unwrap().parse().unwrap();
                        if passport.birth_year > 2002 || passport.birth_year < 1920 {
                            return None;
                        }
                    }
                    //    iyr (Issue Year)
                    [0x69, 0x79, 0x72] => {
                        passport.issue_year = std::str::from_utf8(&value).unwrap().parse().unwrap();
                        if passport.issue_year > 2020 || passport.issue_year < 2010 {
                            return None;
                        }
                    }
                    //    eyr (Expiration Year)
                    [0x65, 0x79, 0x72] => {
                        passport.expiration_year =
                            std::str::from_utf8(&value).unwrap().parse().unwrap();
                        if passport.expiration_year > 2030 || passport.expiration_year < 2020 {
                            return None;
                        }
                    }
                    //    hgt (Height)
                    [0x68, 0x67, 0x74] => {
                        match &value[..] {
                            [_, _, 0x69, 0x6e] => {
                                let height: u32 =
                                    std::str::from_utf8(&value[..2]).unwrap().parse().unwrap();
                                if height > 76 || height < 59 {
                                    return None;
                                }
                            }
                            [_, _, _, 0x63, 0x6d] => {
                                let height: u32 =
                                    std::str::from_utf8(&value[..3]).unwrap().parse().unwrap();
                                if height > 193 || height < 150 {
                                    return None;
                                }
                            }
                            err => {
                                return None;
                            }
                        }

                        passport.height = std::str::from_utf8(&value).unwrap().to_string();
                    }
                    //    hcl (Hair Color)
                    [0x68, 0x63, 0x6c] => match &value[..] {
                        [0x23, 0x30..0x40 | 0x61..0x67, 0x30..0x40 | 0x61..0x67, 0x30..0x40 | 0x61..0x67, 0x30..0x40 | 0x61..0x67, 0x30..0x40 | 0x61..0x67, 0x30..0x40 | 0x61..0x67] => {
                            passport.hair_color = std::str::from_utf8(&value).unwrap().to_string()
                        }
                        err => {
                            return None;
                        }
                    },
                    //    ecl (Eye Color)
                    [0x65, 0x63, 0x6c] => {
                        passport.eye_color = match &value[..] {
                            [0x61, 0x6d, 0x62] => EyeColor::AMB,
                            [0x62, 0x6c, 0x75] => EyeColor::BLU,
                            [0x62, 0x72, 0x6e] => EyeColor::BRN,
                            [0x67, 0x72, 0x79] => EyeColor::GRY,
                            [0x67, 0x72, 0x6e] => EyeColor::GRN,
                            [0x68, 0x7a, 0x6c] => EyeColor::HZL,
                            [0x6f, 0x74, 0x68] => EyeColor::OTH,
                            err => {
                                return None;
                            }
                        }
                    }
                    //    pid (Passport ID)
                    [0x70, 0x69, 0x64] => {
                        passport.passport_id = match &value[..] {
                            [0x30..0x40, 0x30..0x40, 0x30..0x40, 0x30..0x40, 0x30..0x40, 0x30..0x40, 0x30..0x40, 0x30..0x40, 0x30..0x40] => {
                                value[..].try_into().unwrap()
                            }
                            err => {
                                return None;
                            }
                        }
                    }
                    //    cid (Country ID)
                    [0x63, 0x69, 0x64] => {
                        passport.country_id =
                            Some(std::str::from_utf8(&value).unwrap().parse().unwrap())
                    }
                    _ => {
                        unimplemented!()
                    }
                }
            } else {
                buf.push(*cha)
            }
        }

        if passport.valid() {
            return Some(passport);
        } else {
            return None;
        };
    }
}

#[aoc(day4, part1)]
fn solver_part1(input: &str) -> usize {
    let lines: Vec<&str> = input.split("\n\n").collect();

    let mut passports = Vec::new();

    for line in lines {
        let mut line = line.to_owned();
        line.push_str("\n");
        match Passport::part1(line.as_str()) {
            None => continue,
            Some(passport) => passports.push(passport),
        }
    }

    return passports.len();
}

#[aoc(day4, part2)]
fn solver_part2(input: &str) -> usize {
    let lines: Vec<&str> = input.split("\n\n").collect();

    let mut passports = Vec::new();

    for line in lines {
        let mut line = line.to_owned();
        line.push_str("\n");
        match Passport::part2(line.as_str()) {
            None => continue,
            Some(passport) => passports.push(passport),
        }
    }

    return passports.len();
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
        #[test]
        fn test_parse_passport_valid() {
            let passport: Passport = Passport {
                birth_year: 1937,
                issue_year: 2017,
                expiration_year: 2020,
                height: String::from("183cm"),
                hair_color: String::from("#fffffd"),
                eye_color: EyeColor::GRY,
                passport_id: [56, 54, 48, 48, 51, 51, 51, 50, 55],
                country_id: Some(147),
            };

            let input: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
    byr:1937 iyr:2017 cid:147 hgt:183cm
    ";

            assert_eq!(Some(passport), Passport::from(input));
        }

        #[test]
        fn test_parse_passport_invalid() {
            let input: &str = "ecl:gry eyr:2020 hcl:#fffffd
    byr:1937 iyr:2017 cid:147 hgt:183
    ";

            assert_eq!(None, Passport::from(input));
        }
        */
}

const TEST_INPUT: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
";
