use regex::Regex;

struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    // cid: Option<String>,
}

impl Passport {
    fn is_valid(&self) -> bool {
        lazy_static! {
            static ref HGT_REGEX: Regex = Regex::new(r"(\d+)(in|cm)").unwrap();
            static ref HCL_REGEX: Regex = Regex::new(r"#[[:xdigit:]]{6}").unwrap();
            static ref PID_REGEX: Regex = Regex::new(r"^[[:digit:]]{9}$").unwrap();
        }

        if self.byr.len() != 4 {
            return false;
        }
        match self.byr.parse::<i32>() {
            Ok(byr) => {
                if byr < 1920 || byr > 2002 {
                    return false;
                }
            },
            Err(_) => return false,
        }

        if self.byr.len() != 4 {
            return false;
        }
        match self.iyr.parse::<i32>() {
            Ok(iyr) => {
                if iyr < 2010 || iyr > 2020 {
                    return false;
                }
            },
            Err(_) => return false,
        }

        if self.byr.len() != 4 {
            return false;
        }
        match self.eyr.parse::<i32>() {
            Ok(eyr) => {
                if eyr < 2020 || eyr > 2030 {
                    return false;
                }
            },
            Err(_) => return false,
        }

        if let Some(hgt_caps) = HGT_REGEX.captures(&self.hgt) {
            let hgt_int: i32 = hgt_caps[1].parse().unwrap();
            match &hgt_caps[2] {
                "cm" => {
                    if hgt_int < 150 || hgt_int > 193 {
                        return false;
                    }
                },
                "in" => {
                    if hgt_int < 59 || hgt_int > 76 {
                        return false;
                    }
                }
                _ => panic!("oh no"),
            }
        } else {
            return false;
        }

        if !HCL_REGEX.is_match(&self.hcl) {
            return false;
        }

        match self.ecl.as_str() {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {},
            _ => return false,
        }

        if !PID_REGEX.is_match(&self.pid) {
            return false;
        }

        true
    }
}

#[derive(Default)]
struct PassportBuilder {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl PassportBuilder {
    fn build(self) -> Option<Passport> {
        Some(Passport {
            byr: self.byr?,
            iyr: self.iyr?,
            eyr: self.eyr?,
            hgt: self.hgt?,
            hcl: self.hcl?,
            ecl: self.ecl?,
            pid: self.pid?,
            // cid: self.cid,
        })
    }

    fn parse_word(&mut self, input: &str) {
        let mut iter = input.split(':');
        let key = iter.next().expect("no key");
        let value = iter.next().expect("no value");
        let value = Some(value.to_string());

        match key {
            "byr" => self.byr = value,
            "iyr" => self.iyr = value,
            "eyr" => self.eyr = value,
            "hgt" => self.hgt = value,
            "hcl" => self.hcl = value,
            "ecl" => self.ecl = value,
            "pid" => self.pid = value,
            "cid" => self.cid = value,
            unknown_key => panic!("Unrecognized key: {}", unknown_key),
        }
    }
}

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Vec<Passport> {
    let mut surface_valid_passports = vec![];
    let mut current_builder = PassportBuilder::default();

    for word in input.split(|c| c == ' ' || c == '\n') {
        if word == "" {
            if let Some(passport) = current_builder.build() {
                surface_valid_passports.push(passport);
            }
            current_builder = PassportBuilder::default();
            continue;
        }
        current_builder.parse_word(word);
    }

    if let Some(passport) = current_builder.build() {
        surface_valid_passports.push(passport);
    }

    surface_valid_passports
}

#[aoc(day4, part1)]
fn solve_part1(input: &[Passport]) -> usize {
    input.len()
}

#[aoc(day4, part2)]
fn solve_part2(input: &[Passport]) -> usize {
    input.iter().filter(|p| p.is_valid()).count()
}