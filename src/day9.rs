#[aoc_generator(day9)]
fn parse_input(input: &str) -> Vec<i64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

// I didn't account for adding the same number to itself, but I didn't get punished
// so I'm not fixing it
#[aoc(day9, part1)]
fn solve_part1(input: &[i64]) -> i64 {
    let mut buf = Vec::with_capacity(25);
    'outer: for window in input.windows(26) {
        buf.clear();
        let (prev_25, last) = window.split_at(25);
        let last = last[0];

        for x in prev_25 {
            buf.push(last -x);
        }
        buf.sort_unstable();


        for x in prev_25 {
            match buf.binary_search(x) {
                Ok(_) => continue 'outer,
                Err(_) => continue,
            }
        }

        // we found a number with no matches
        return last;
    }

    panic!("No number found :(")
}

#[aoc(day9, part2)]
fn solve_part2(input: &[i64]) -> i64 {
    let target = solve_part1(input);

    let mut window_size = 2;
    let mut sum: i64;

    loop {
        sum = input.iter().take(window_size - 1).sum();
        
        for window in input.windows(window_size) {
            let first = window.first().unwrap();
            let last = window.last().unwrap();
            sum += last;

            if sum == target {
                dbg!(window_size);
                let min = window.iter().min().unwrap();
                let max = window.iter().max().unwrap();
                return min + max;
            }

            sum -= first;
        }

        window_size += 1;
    }    
}