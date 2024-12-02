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

fn part_2(input: &str) -> usize {
    input.lines().filter(|&line| {
        let nums: Vec<_> = line.split_whitespace().map(|num| num.parse::<i8>().unwrap()).collect();
        (-1..=nums.len() as isize).any(|x| {
            match nums.iter().enumerate().filter(|&(n, _)|
                n as isize != x
            ).fold((State::Init, 0), |(state, prev), (_, &num)| {
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
        })
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
