use std::io::Read;

fn part_1(input: &Vec<u8>) -> u32 {
    let width = input.iter().position(|&c| c == b'\n').unwrap() + 1;
    input.iter().enumerate().filter(|&(_, &x)| x == b'X').map(|(n, _)| {
        let mut count = 0;
        if n >= 3 && input[n - 3..n] == [b'S', b'A', b'M'] {
            count += 1;
        }
        if n < input.len() - 3 && input[n + 1..n + 4] == [b'M', b'A', b'S'] {
            count += 1;
        }
        if n >= 3 * width && input[n - 3 * width..n].iter().step_by(width).eq([b'S', b'A', b'M'].iter()) {
            count += 1;
        }
        if n < input.len() - 3 * width && input[n + width..n + 3 * width + 1].iter().step_by(width).eq([b'M', b'A', b'S'].iter()) {
            count += 1;
        }
        if n >= width * 3 + 3 && input[n - 3 * width - 3..n].iter().step_by(width + 1).eq([b'S', b'A', b'M'].iter()) {
            count += 1;
        }
        if n >= width * 3 && input[n - 3 * width + 3..n].iter().step_by(width - 1).eq([b'S', b'A', b'M'].iter()) {
            count += 1;
        }
        if n <= input.len() - width * 3 - 5 && input[n + width + 1..n + 3 * width + 4].iter().step_by(width + 1).eq([b'M', b'A', b'S'].iter()) {
            count += 1;
        }
        if n <= input.len() - width * 3 - 2 && input[n + width - 1..n + 3 * width - 2].iter().step_by(width - 1).eq([b'M', b'A', b'S'].iter()) {
            count += 1;
        }

        count
    }).sum()
}

fn part_2(input: &Vec<u8>) -> usize {
    let width = input.iter().position(|&c| c == b'\n').unwrap() + 1;
    input.iter().enumerate().take(input.len() - width - 2).skip(width + 1).filter(|&(n, &x)| {
        x == b'A' &&
        [[b'M', b'S'], [b'S', b'M']].contains(&[input[n - width - 1], input[n + width + 1]]) &&
        [[b'M', b'S'], [b'S', b'M']].contains(&[input[n - width + 1], input[n + width - 1]])
    }).count()
}

fn main() {
    let mut input = Vec::new();
    std::io::stdin().read_to_end(&mut input).unwrap();

    let start_time = std::time::Instant::now();
    let result = part_1(&input);
    println!("Part 1 time: {:?}", std::time::Instant::now() - start_time);
    println!("Part 1 result: {}", result);

    let start_time = std::time::Instant::now();
    let result = part_2(&input);
    println!("Part 2 time: {:?}", std::time::Instant::now() - start_time);
    println!("Part 2 result: {}", result);
}
