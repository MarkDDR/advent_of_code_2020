#[aoc_generator(day2)]
fn day2_generator(input: &str) -> Vec<Policy> {
    let mut policies = vec![];
    for line in input.lines() {
        let mut iter = line.split(|c: char| !c.is_alphanumeric());
        let num_a = iter.next().unwrap().parse::<u8>().unwrap();
        let num_b = iter.next().unwrap().parse::<u8>().unwrap();
        let letter = iter.next().unwrap().as_bytes()[0];
        let password = iter.nth(1).unwrap().as_bytes().to_vec();
        policies.push(Policy {
            num_a,
            num_b,
            letter,
            password,
        })
    }

    policies
}

struct Policy {
    num_a: u8,
    num_b: u8,
    letter: u8,
    password: Vec<u8>,
}

impl Policy {
    fn part1_is_valid(&self) -> bool {
        let mut appearances = 0;
        for letter in &self.password {
            if letter == &self.letter {
                appearances += 1;
            }
        }
        self.num_a <= appearances && appearances <= self.num_b
    }

    fn part2_is_valid(&self) -> bool {
        let at_a = self.password[self.num_a as usize - 1] == self.letter;
        let at_b = self.password[self.num_b as usize - 1] == self.letter;
        at_a ^ at_b
    }
}

#[aoc(day2, part1)]
fn num_valid_passwords_part1(policy_lines: &[Policy]) -> usize {
    policy_lines.iter().filter(|x| x.part1_is_valid()).count()
}

#[aoc(day2, part2)]
fn num_valid_passwords_part2(policy_lines: &[Policy]) -> usize {
    policy_lines.iter().filter(|x| x.part2_is_valid()).count()
}