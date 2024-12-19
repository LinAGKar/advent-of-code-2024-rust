use std::io::Read;

fn possible(pattern: &[u8], position: usize, maybe_possible: &mut [bool], towels: &[Vec<u8>]) -> bool {
    if position == pattern.len() {
        return true;
    }

    if !maybe_possible[position] {
        return false;
    }

    let possible = towels.iter().any(|towel|
        towel.len() + position <= pattern.len() &&
        pattern[position..position + towel.len()] == *towel &&
        possible(pattern, position + towel.len(), maybe_possible, towels)
    );

    if !possible {
        maybe_possible[position] = false;
    }

    possible
}

fn part_1(input: &str) -> usize {
    let mut lines = input.lines();
    let towels: Vec<Vec<_>> = lines.next().unwrap().split(", ").map(|towel|
        towel.chars().map(|c| c as u8).collect()
    ).collect();

    lines.skip(1).filter(|&pattern| {
        let pattern: Vec<_> = pattern.chars().map(|c| c as u8).collect();
        possible(&pattern, 0, &mut &mut vec![true; pattern.len()], &towels)
    }).count()
}

fn count_possibilities(pattern: &[u8], position: usize, possibilities: &mut [Option<u64>], towels: &[Vec<u8>]) -> u64 {
    if position == pattern.len() {
        return 1;
    }

    if let Some(possibilities) = possibilities[position] {
        return possibilities;
    }

    let possible = towels.iter().map(|towel|
        if towel.len() + position > pattern.len() || pattern[position..position + towel.len()] != *towel {
            0
        } else {
            count_possibilities(pattern, position + towel.len(), possibilities, towels)
        }
    ).sum();

    possibilities[position] = Some(possible);

    possible
}

fn part_2(input: &str) -> u64 {
    let mut lines = input.lines();
    let towels: Vec<Vec<_>> = lines.next().unwrap().split(", ").map(|towel|
        towel.chars().map(|c| c as u8).collect()
    ).collect();

    lines.skip(1).map(|pattern| {
        let pattern: Vec<_> = pattern.chars().map(|c| c as u8).collect();
        count_possibilities(&pattern, 0, &mut vec![None; pattern.len()], &towels)
    }).sum()
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
