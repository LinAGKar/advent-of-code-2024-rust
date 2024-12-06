use std::io::Read;

fn part_1(input: &str) -> u32 {
    let mut lines = input.lines();
    let mut must_come_after = vec![[false; 100]; 100];

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let mut parts = line.split('|').map(|x| x.parse::<u8>().unwrap());
        let a = parts.next().unwrap();
        let b = parts.next().unwrap();

        must_come_after[a as usize][b as usize] = true;
    }

    lines.filter_map(|line| {
        let nums = line.split(',').map(|x| x.parse::<u8>().unwrap()).collect::<Vec<_>>();
        for (n, &i) in nums.iter().enumerate() {
            for &j in nums.iter().take(n) {
                if must_come_after[i as usize][j as usize] {
                    return None;
                }
            }
        }
        Some(nums[nums.len() / 2] as u32)
    }).sum()
}

fn part_2(input: &str) -> u32 {
    let mut lines = input.lines();
    let mut must_come_after = vec![[false; 100]; 100];

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let mut parts = line.split('|').map(|x| x.parse::<u8>().unwrap());
        let a = parts.next().unwrap();
        let b = parts.next().unwrap();

        must_come_after[a as usize][b as usize] = true;
    }

    let mut nums = Vec::new();
    let mut new_nums = Vec::new();

    lines.filter_map(|line| {
        nums.clear();
        nums.extend(line.split(',').map(|x| x.parse::<u8>().unwrap()));
        new_nums.clear();
        new_nums.reserve(nums.len());
        let mut changed = false;
        while !nums.is_empty() {
            let next = nums.iter().position(|&x|
                !nums.iter().any(|&y| must_come_after[y as usize][x as usize])
            ).unwrap();

            new_nums.push(nums.remove(next));
            changed |= next > 0;
        }

        if !changed {
            None
        } else {
            Some(new_nums[new_nums.len() / 2] as u32)
        }
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
