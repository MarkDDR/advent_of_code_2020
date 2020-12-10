use std::collections::HashMap;

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Vec<i32> {
    let mut vec: Vec<i32> = input.lines().map(|l| l.parse().unwrap()).collect();
    vec.sort_unstable();
    vec
}

#[aoc(day10, part1)]
fn solve_part1(input: &[i32]) -> i32 {
    let mut diff_1 = 0;
    // takes into account last step
    let mut diff_3 = 1;
    let mut last_num = 0;

    for x in input {
        match x - last_num {
            1 => diff_1 += 1,
            3 => diff_3 += 1,
            _ => {},
        }
        last_num = *x;
    }

    diff_1 * diff_3
}

// Key insight for part 2: num_paths(x) = num_paths(x-1) + num_paths(x-2) + num_paths(x-3)
#[aoc(day10, part2)]
fn solve_part2(input: &[i32]) -> i64 {
    let mut number_paths = HashMap::new();
    number_paths.insert(0, 1i64);

    for x in input {
        let mut paths_x = 0;
        for i in 1..=3 {
            paths_x += number_paths.get(&(x - i)).unwrap_or(&0);
        }
        number_paths.insert(*x, paths_x);
    }

    *number_paths.get(input.last().unwrap()).unwrap()
}

#[aoc(day10, part2, vec)]
fn solve_part2_vec(input: &[i32]) -> i64 {
    let mut number_paths = vec![0i64; *input.last().unwrap() as usize + 1];
    number_paths[0] = 1;

    for x in input {
        let x = *x as usize;
        let mut paths_x = 0;
        // saturating sub to deal with first few numbers that can go below 0
        let min_i = x.saturating_sub(3);
        for i in min_i..=x-1 {
            paths_x += number_paths[i];
        }
        number_paths[x] = paths_x;
    }

    *number_paths.last().unwrap()
}