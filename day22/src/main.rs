use std::io::Read;

fn part_1(input: &str) -> u64 {
    input.lines().map(|line| {
        let mut num: u32 = line.parse().unwrap();
        for _ in 0..2000 {
            num = num ^ (num << 6) & 0xFFFFFF;
            num = num ^ (num >> 5) & 0xFFFFFF;
            num = num ^ (num << 11) & 0xFFFFFF;
        }
        num as u64
    }).sum()
}

const SEQ_COUNT: usize = 19 * 19 * 19 * 19;

fn part_2(input: &str) -> u16 {
    let mut bananas_per_sequence = vec![0; SEQ_COUNT];
    let mut sequence_seen_at = vec![-1; SEQ_COUNT];

    for (n, line) in input.lines().enumerate() {
        let mut num: u32 = line.parse().unwrap();
        let mut sequence = 0;
        let mut old_price = num % 10;
        for i in 0..2000 {
            num = num ^ (num << 6) & 0xFFFFFF;
            num = num ^ (num >> 5) & 0xFFFFFF;
            num = num ^ (num << 11) & 0xFFFFFF;
            let price = num % 10;
            let diff = price + 9 - old_price;
            old_price = price;
            sequence = (sequence * 19 + diff) % SEQ_COUNT as u32;
            if i >= 3 {
                if sequence_seen_at[sequence as usize] < n as i16 {
                    sequence_seen_at[sequence as usize] = n as i16;
                    bananas_per_sequence[sequence as usize] += price as u16;
                }
            }
        }
    }

    bananas_per_sequence.into_iter().max().unwrap()
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
