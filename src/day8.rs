use std::{collections::HashSet, collections::HashMap};

#[derive(Copy, Clone, Debug)]
enum Instructions {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl Instructions {
    fn parse(input: &str) -> Vec<Self> {
        let mut out = vec![];

        for line in input.lines() {
            let mut words = line.split(' ');
            let word = words.next().unwrap();
            let amt: i32 = {
                let mut second_word = words.next().unwrap();
                if second_word.starts_with('+') {
                    second_word = &second_word[1..];
                }
                second_word.parse().unwrap()
            };

            match word {
                "nop" => out.push(Instructions::Nop(amt)),
                "acc" => out.push(Instructions::Acc(amt)),
                "jmp" => out.push(Instructions::Jmp(amt)),
                _ => panic!("unknown instruction"),
            }
        }

        out
    }
}

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Vec<Instructions> {
    Instructions::parse(input)
}

#[aoc(day8, part1)]
fn solve_part1(input: &[Instructions]) -> i32 {
    let mut head = 0;
    let mut acc = 0;
    let mut visited = HashSet::new();

    while visited.insert(head) {
        match input[head] {
            Instructions::Nop(_) => {},
            Instructions::Acc(amt) => acc += amt,
            Instructions::Jmp(amt) => {
                head += amt as usize;
                continue;
            },
        }
        head += 1;
    }
    acc
}

#[aoc(day8, part2)]
fn solve_part2(input: &[Instructions]) -> i32 {
    // First run the computer normally to see where we visit
    let mut head = 0;
    let mut acc = 0;
    let mut visited = HashMap::new();
    let mut jmps_and_nops = vec![];

    while let None = visited.get(&head) {
        visited.insert(head, acc);
        
        match input[head] {
            Instructions::Nop(_) => jmps_and_nops.push(head),
            Instructions::Acc(amt) => acc += amt,
            Instructions::Jmp(amt) => {
                jmps_and_nops.push(head);
                head += amt as usize;
                continue;
            },
        }
        head += 1;
    }

    // Now for every jmp or nop we visited, flip it and see what happens
    for mut head in jmps_and_nops {
        let mut acc = *visited.get(&head).unwrap();
        
        match input[head] {
            Instructions::Nop(amt) => head += amt as usize,
            Instructions::Jmp(_) => head += 1,
            _ => panic!("uh oh"),
        }

        // if we visit an instruction we already visited then we know it didn't work
        while let None = visited.get(&head) {
            visited.insert(head, acc);
            
            match input.get(head) {
                Some(Instructions::Nop(_)) => {},
                Some(Instructions::Acc(amt)) => acc += amt,
                Some(Instructions::Jmp(amt)) => {
                    head += *amt as usize;
                    continue;
                },
                None => {
                    if head == input.len() {
                        return acc;
                    } else {
                        // we are too far past, if we hit this try skipping
                        panic!("maybe implement this codepath");
                    }
                },
            }
            head += 1;
        }
    }
    panic!("We didn't find it :(");
}