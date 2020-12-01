#[aoc_generator(day1)]
fn sorted_vec_generator(input: &str) -> Vec<i32> {
    let mut v: Vec<_> = input.lines().map(|l| l.parse::<i32>().unwrap()).collect();
    v.sort();
    v
}

#[aoc(day1, part1)]
fn solve_part1(input: &[i32]) -> i32 {
    let target = 2020;
    let mut first_num_index = 0;
    let mut last_num_index = input.len() - 1;
    'first_num: for (i, first_num) in input.iter().enumerate() {
        first_num_index = i;
        loop {
            let sum = first_num + input[last_num_index];
            if sum < target {
                continue 'first_num;
            } else if sum == target {
                break 'first_num;
            } else {
                last_num_index -= 1;
            }
        }
    }

    let num_one = input[first_num_index];
    let num_two = input[last_num_index];
    assert_eq!(num_one + num_two, target, "Something went wrong :(");

    num_one * num_two
}

#[aoc(day1, part2)]
fn solve_part2(input: &[i32]) -> i32 {
    let target = 2020;
    let mut first_num_index = 0;
    let mut second_num_index = 1;
    let mut third_num_index = input.len() - 1;
    'first_num: for (i, first_num) in input.iter().enumerate() {
        first_num_index = i;
        third_num_index = input.len() - 1;
        'second_num: for (i, second_num) in input.iter().enumerate() {
            second_num_index = i;
            loop {
                let sum = first_num + second_num + input[third_num_index];
                if third_num_index <= second_num_index {
                    break 'second_num;
                } else if sum < target {
                    continue 'second_num;
                } else if sum == target {
                    break 'first_num;
                } else {
                    third_num_index -= 1;
                }
            }
        }
    }

    let num_one = input[first_num_index];
    let num_two = input[second_num_index];
    let num_three = input[third_num_index];
    assert_eq!(num_one + num_two + num_three, target, "Something went wrong :(");

    num_one * num_two * num_three
}