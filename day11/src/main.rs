use std::{collections::HashMap, io::Read};

const TEN_POWERS: [u64; 20] = {
    let mut result = [0; 20];
    let mut current = 1;
    let mut i = 0;
    loop {
        result[i] = current;
        if i >= result.len() - 1 {
            break;
        }
        current *= 10;
        i += 1;
    }
    result
};

fn split_number(num: u64) -> Option<[u64; 2]> {
    let significant_digits = TEN_POWERS.iter().position(|&x| x > num).unwrap_or(TEN_POWERS.len());
    if significant_digits % 2 == 0 {
        let cutoff = TEN_POWERS[significant_digits / 2];
        Some([num / cutoff, num % cutoff])
    } else {
        None
    }
}

fn add_rock(number: u64, count: u64, rocks: &mut HashMap<u64, u64>) {
    *rocks.entry(number).or_insert(0) += count;
}

fn solve(input: &str, iterations: u16) -> u64 {
    let mut rocks = HashMap::new();
    for rock in input.split_ascii_whitespace().map(|s| s.parse::<u64>().unwrap()) {
        add_rock(rock, 1, &mut rocks);
    }
    let mut new_rocks = HashMap::new();

    for _ in 0..iterations {
        for (&number, &count) in &rocks {
            if number == 0 {
                add_rock(1, count, &mut new_rocks);
            } else if let Some([a, b]) = split_number(number) {
                add_rock(a, count, &mut new_rocks);
                add_rock(b, count, &mut new_rocks);
            } else {
                add_rock(number * 2024, count, &mut new_rocks);
            }
        }

        rocks.clear();
        std::mem::swap(&mut rocks, &mut new_rocks);
    }

    rocks.into_iter().map(|(_, count)| count).sum()
}

fn part_1(input: &str) -> u64 {
    solve(input, 25)
}

fn part_2(input: &str) -> u64 {
    solve(input, 75)
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
