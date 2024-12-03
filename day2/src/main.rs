use std::io::Read;

#[derive(Debug)]
enum State {
    Init,
    Start,
    Decreasing,
    Increasing,
    Unsafe,
}

fn part_1(input: &str) -> usize {
    input.lines().filter(|&line| {
        match line.split_whitespace().fold((State::Init, 0), |(state, prev), num| {
            let num = num.parse::<i8>().unwrap();
            (match (state, num - prev) {
                (State::Init, _) => State::Start,
                (State::Increasing | State::Start, 1..=3) => State::Increasing,
                (State::Decreasing | State::Start, -3..=-1) => State::Decreasing,
                _ => State::Unsafe,
            }, num)
        }).0 {
            State::Unsafe => false,
            _ => true,
        }
    }).count()
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Change {
    Increase,
    Decrease,
    Unsafe,
}

fn part_2(input: &str) -> usize {
    let mut nums = Vec::new();
    input.lines().filter(|&line| {
        nums.clear();
        nums.extend(line.split_whitespace().map(|num| num.parse::<i8>().unwrap()));
        let changes: Vec<_> = nums.windows(2).map(|window| match window[1] - window[0] {
            1..=3 => Change::Increase,
            -3..=-1 => Change::Decrease,
            _ => Change::Unsafe,
        }).collect();

        let expected_change = if changes.iter().filter(|&&x| x == Change::Increase).count() >= 2 {
            Change::Increase
        } else {
            Change::Decrease
        };
        let first_incorrect = changes.iter().enumerate().find_map(|(n, &x)| if x != expected_change {
            Some(n)
        } else {
            None
        });

        if let Some(n) = first_incorrect {
            if changes.iter().skip(n + 2).any(|&x| x != expected_change) {
                false
            } else if n == changes.len() - 1 {
                true
            } else if match (nums[n + 2] - nums[n], expected_change) {
                (-3..=-1, Change::Decrease) => true,
                (1..=3, Change::Increase) => true,
                _ => false,
            } {
                true
            } else if changes[n + 1] != expected_change {
                false
            } else if n == 0 {
                true
            } else if match (nums[n + 1] - nums[n - 1], expected_change) {
                (-3..=-1, Change::Decrease) => true,
                (1..=3, Change::Increase) => true,
                _ => false,
            } {
                true
            } else {
                false
            }
        } else {
            true
        }
    }).count()
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
