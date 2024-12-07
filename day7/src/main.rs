use std::io::Read;

fn magnitude_above(num: u64) -> u64 {
    let mut magnitude = 1;
    while magnitude <= num {
        magnitude *= 10;
    }
    magnitude
}

fn can_get_result(operands: &[u64], current_val: u64, target: u64, allow_concat: bool) -> bool {
    if current_val > target {
        false
    } else if operands.is_empty() {
        current_val == target
    } else {
        can_get_result(&operands[1..], current_val + operands[0], target, allow_concat) ||
            can_get_result(&operands[1..], current_val * operands[0], target, allow_concat) ||
            (allow_concat &&
                can_get_result(
                    &operands[1..], current_val * magnitude_above(operands[0]) + operands[0], target, allow_concat,
                ))
    }
}

fn solve(input: &str, allow_concat: bool) -> u64 {
    input.lines().filter_map(|line| {
        let mut nums = line.split_whitespace().map(|num| num.trim_end_matches(':').parse::<u64>().unwrap());
        let result = nums.next().unwrap();
        let operands: Vec<_> = nums.collect();

        if can_get_result(&operands[1..], operands[0], result, allow_concat) {
            Some(result)
        } else {
            None
        }
    }).sum()
}

fn part_1(input: &str) -> u64 {
    solve(input, false)
}

fn part_2(input: &str) -> u64 {
    solve(input, true)
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let start_time = std::time::Instant::now();
    let result = part_1(&input);
    println!("Part 1 time: {:?}", std::time::Instant::now() - start_time);
    println!("Part 1 result: {}", result);

    let start_time = std::time::Instant::now();
    let result = part_2(&input);
    println!("Part 2 time: {:?}", std::time::Instant::now() - start_time);
    println!("Part 2 result: {}", result);
}
