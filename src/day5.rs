use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
struct AirplaneSeat(u32);

impl AirplaneSeat {
    // fn get_row(&self) -> u32 {
    //     self.0 >> 3
    // }

    // fn get_col(&self) -> u32 {
    //     self.0 & 0b111
    // }

    fn get_id(&self) -> u32 {
        self.0
    }
}

impl FromStr for AirplaneSeat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut num = 0;
        for b in s.bytes() {
            num <<= 1;
            match b {
                b'F' | b'L' => num |= 0,
                b'B' | b'R' => num |= 1,
                unknown => panic!("Unrecognized character: `{}`", unknown as char),
            }
        }
        assert!(num == num & ((1<<10) - 1), "Input too long");
        Ok(Self(num))
    }
}

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Vec<AirplaneSeat> {
    let mut out = vec![];
    for line in input.lines() {
        out.push(line.parse().unwrap());
    }
    out
}

#[aoc(day5, part1)]
fn solve_part1(input: &[AirplaneSeat]) -> u32 {
    input.iter().map(|x| x.get_id()).max().unwrap()
}

#[aoc(day5, part2)]
fn solve_part2(input: &[AirplaneSeat]) -> u32 {
    let mut sorted = input.to_vec();
    sorted.sort_by_key(|k| k.get_id());
    let mut last_id = 0;

    for id in &sorted {
        let id = id.get_id();
        if last_id + 2 == id {
            return last_id + 1;
        }
        last_id = id;
    }

    panic!("no missing id found");
}