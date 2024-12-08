use std::io::Read;

fn solve(input: &str, part_2: bool) -> u16 {
    let mut frequencies = [const { Vec::new() }; 26 + 26 + 10];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate().filter(|&(_, c)| c != '.') {
            frequencies[if c.is_ascii_digit() {
                c as usize - '0' as usize
            } else if c.is_ascii_uppercase() {
                c as usize - 'A' as usize + 10
            } else if c.is_ascii_lowercase() {
                c as usize - 'a' as usize + 36
            } else {
                panic!("Invalid frequency: {}", c)
            }].push((x as i8, y as i8));
        }
    }

    let width = input.chars().position(|c| c == '\n').unwrap();
    let mut has_antinode: Vec<_> = vec![false; input.chars().filter(|&c| c != '\n').count()];
    let mut antinode_count = 0;

    for frequency in frequencies {
        for (n, &(ax, ay)) in frequency.iter().enumerate() {
            for &(bx, by) in &frequency[n + 1..] {
                let dx = ax - bx;
                let dy = ay - by;
                for (x, y, dx, dy) in [
                    (ax, ay, dx, dy),
                    (bx, by, -dx, -dy),
                ] {
                    for i in if part_2 { 0..=i8::MAX } else { 1..=1 } {
                        let (x, y) = (x + dx * i, y + dy * i);
                        if x < 0 || (x as usize) >= width || y < 0 || y as usize * width >= has_antinode.len() {
                            break;
                        }
                        let pos = y as usize * width + x as usize;
                        if !has_antinode[pos] {
                            antinode_count += 1;
                            has_antinode[pos] = true;
                        }
                    }
                }
            }
        }
    }

    antinode_count
}

fn part_1(input: &str) -> u16 {
    solve(input, false)
}

fn part_2(input: &str) -> u16 {
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
