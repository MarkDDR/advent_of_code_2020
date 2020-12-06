use std::collections::HashSet;

#[aoc(day6, part1, Bytes)]
fn solve_part1(input: &[u8]) -> usize {
    let mut all_groups = vec![];
    let mut current_group = HashSet::new();

    for line in input.split(|b| *b == b'\n') {
        if line.is_empty() {
            all_groups.push(current_group.len());
            current_group = HashSet::new();
            continue;
        }
        for c in line {
            current_group.insert(*c);
        }
    }
    all_groups.push(current_group.len());

    all_groups.iter().sum()
}

#[aoc(day6, part2, Bytes)]
fn solve_part2(input: &[u8]) -> usize {
    let mut all_groups = vec![];
    let mut current_group: Option<HashSet<u8>> = None;

    for line in input.split(|b| *b == b'\n') {
        if line.is_empty() {
            all_groups.push(current_group.map_or(0, |set| set.len()));
            current_group = None;
            continue;
        }
        let mut member = HashSet::new();
        for c in line {
            member.insert(*c);
        }

        current_group = if let Some(current_group) = current_group {
            Some(current_group.intersection(&member).map(|b| *b).collect())
        } else {
            Some(member)
        }
    }
    all_groups.push(current_group.map_or(0, |set| set.len()));

    all_groups.iter().sum()
}