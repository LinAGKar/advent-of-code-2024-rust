use std::{collections::HashMap, io::Read};

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    input.lines().map(|line| {
        let mut words = line.split_whitespace().map(|word| word.parse::<i32>().unwrap());
        (words.next().unwrap(), words.next().unwrap())
    }).unzip()
}

fn part_1(input: &str) -> i32 {
    let (mut a, mut b) = parse_input(input);

    a.sort_unstable();
    b.sort_unstable();

    a.into_iter().zip(b).map(|(a, b)| (a - b).abs()).sum()
}

fn part_2(input: &str) -> i32 {
    let (a, b) = parse_input(input);

    let mut b_counts = HashMap::new();
    for i in b {
        *b_counts.entry(i).or_insert(0) += 1;
    }

    a.into_iter().map(|x| x * b_counts.get(&x).copied().unwrap_or(0)).sum()
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
