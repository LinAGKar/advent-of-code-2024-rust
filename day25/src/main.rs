use std::io::Read;

fn part_1(input: &str) -> u16 {
    let mut lines = input.lines();
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    loop {
        let mut current = [[false; 5]; 7];

        for i in 0..7 {
            let line = lines.next().unwrap();
            for (j, c) in line.chars().enumerate() {
                current[i][j] = c == '#';
            }
        }

        if current[0][0] {
            locks.push((0..5).fold(0u32, |acc, i| {
                (acc << 4) | (1u32..=6u32).find(|&j| !current[j as usize][i]).unwrap()
            }));
        } else {
            keys.push((0..5).fold(0u32, |acc, i| {
                (acc << 4) | ((0u32..=5u32).find(|&j| !current[5 - j as usize][i]).unwrap() + 1)
            }));
        }

        if lines.next().is_none() {
            break;
        }
    }

    keys.into_iter().map(|key| {
        locks.iter().filter(|&&lock| {
            (key + lock) & 0x88888 == 0
        }).count() as u16
    }).sum()
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let start_time = std::time::Instant::now();
    let result = part_1(&input);
    println!("Time: {:?}", std::time::Instant::now() - start_time);
    println!("Result: {}", result);
}
