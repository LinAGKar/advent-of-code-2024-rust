use std::io::Read;

fn part_1(input: &str) -> usize {
    let width = input.chars().position(|c| c == '\n').unwrap();
    let map = input.chars().filter(|&c| c != '\n').map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<_>>();
    let mut positions = Vec::new();
    let mut new_positions = Vec::new();

    map.iter().enumerate().filter(|&(_, &height)| height == 0).map(|(pos, _)| {
        positions.clear();
        positions.push(pos);

        for i in 1..=9 {
            for &pos in &positions {
                if pos % width > 0 && map[pos - 1] == i && !new_positions.contains(&(pos - 1)) {
                    new_positions.push(pos - 1);
                }
                if pos % width < width - 1 && map[pos + 1] == i && !new_positions.contains(&(pos + 1)) {
                    new_positions.push(pos + 1);
                }
                if pos >= width && map[pos - width] == i && !new_positions.contains(&(pos - width)) {
                    new_positions.push(pos - width);
                }
                if pos < map.len() - width && map[pos + width] == i && !new_positions.contains(&(pos + width)) {
                    new_positions.push(pos + width);
                }
            }

            positions.clear();
            std::mem::swap(&mut positions, &mut new_positions);
        }

        positions.len()
    }).sum()
}

fn part_2(input: &str) -> u16 {
    let width = input.chars().position(|c| c == '\n').unwrap();
    let map = input.chars().filter(|&c| c != '\n').map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<_>>();
    let mut positions = Vec::new();
    let mut new_positions = Vec::new();
    let mut to_try = Vec::new();

    map.iter().enumerate().filter(|&(_, &height)| height == 0).map(|(pos, _)| {
        positions.clear();
        positions.push((pos, 1));

        for i in 1..=9 {
            for &(pos, paths) in &positions {
                if pos % width > 0 {
                    to_try.push(pos - 1);
                }
                if pos % width < width - 1 {
                    to_try.push(pos + 1);
                }
                if pos >= width {
                    to_try.push(pos - width);
                }
                if pos < map.len() - width {
                    to_try.push(pos + width);
                }
                for &new_pos in to_try.iter().filter(|&&new_pos| map[new_pos] == i) {
                    if let Some((_, existing_paths)) = new_positions.iter_mut().find(|&&mut (pos, _)| pos == new_pos) {
                        *existing_paths += paths;
                    } else {
                        new_positions.push((new_pos, paths));
                    }
                }
                to_try.clear();
            }

            positions.clear();
            std::mem::swap(&mut positions, &mut new_positions);
        }

        positions.iter().map(|&(_, paths)| paths).sum::<u16>()
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
