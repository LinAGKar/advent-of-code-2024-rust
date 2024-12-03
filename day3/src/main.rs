use std::io::Read;
use regex::Regex;

fn part_1(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input).map(|cap|
        cap[1].parse::<u32>().unwrap() *
        cap[2].parse::<u32>().unwrap()
    ).sum()
}

fn part_2(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|(do\(\))|(don't\(\))").unwrap();
    re.captures_iter(input).fold((0, true), |(acc, enabled), cap|
        if cap.get(3).is_some() {
            (acc, true)
        } else if cap.get(4).is_some() {
            (acc, false)
        } else {
            (acc + if enabled {
                cap[1].parse::<u32>().unwrap() *
                cap[2].parse::<u32>().unwrap()
            } else {
                0
            }, enabled)
        }
    ).0
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
