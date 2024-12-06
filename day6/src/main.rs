use std::io::Read;

fn parse_map(input: &str) -> (usize, Vec<bool>, usize) {
    let width = input.chars().position(|c| c == '\n').unwrap();
    let map = input.chars().filter(|&c| c != '\n').map(|c| match c {
        '.' | '^' => false,
        '#' => true,
        _ => panic!(),
    }).collect::<Vec<_>>();
    let start = input.chars().filter(|&c| c != '\n').position(|c| c == '^').unwrap();
    (width, map, start)
}

fn trace_path(width: usize, map: &[bool], start: usize, start_heading: usize, track_visited: bool) -> (u16, Vec<(u16, u8)>, bool) {
    let mut pos = start;
    let mut heading = start_heading;
    let mut visited = vec![false; map.len()];
    let mut visited_list = Vec::new();
    let mut visited_in_direction = vec![[false; 4]; map.len()];
    let mut visited_count = 0;

    let found_loop = loop {
        if visited_in_direction[pos][heading] {
            break true;
        }

        if !visited[pos] {
            visited[pos] = true;
            visited_count += 1;
            if track_visited {
                visited_list.push((pos as u16, heading as u8));
            }
        }
        visited_in_direction[pos][heading] = true;

        let outside_map = match heading {
            0 => pos < width,
            1 => pos % width == width - 1,
            2 => pos >= map.len() - width,
            3 => pos % width == 0,
            _ => panic!(),
        };

        if outside_map {
            break false;
        }

        let next_pos = match heading {
            0 => pos - width,
            1 => pos + 1,
            2 => pos + width,
            3 => pos - 1,
            _ => panic!(),
        };

        if map[next_pos] {
            heading = (heading + 1) % 4;
        } else {
            pos = next_pos;
        }
    };

    (visited_count, visited_list, found_loop)
}

fn part_1(input: &str) -> u16 {
    let (width, map, start) = parse_map(input);
    let (visited_count, _, _) = trace_path(width, &map, start, 0, false);
    visited_count
}

fn part_2(input: &str) -> usize {
    let (width, mut map, start) = parse_map(input);
    let (_, visited_list, _) = trace_path(width, &map, start, 0, true);
    visited_list.iter().skip(1).filter(|&&(obstruction_pos, obstruction_direction)| {
        let obstruction_pos = obstruction_pos as usize;
        let start = match obstruction_direction {
            0 => obstruction_pos + width,
            1 => obstruction_pos - 1,
            2 => obstruction_pos - width,
            3 => obstruction_pos + 1,
            _ => panic!(),
        };
        map[obstruction_pos] = true;
        let (_, _, found_loop) = trace_path(width, &map, start, obstruction_direction as usize, false);
        map[obstruction_pos] = false;
        found_loop
    }).count()
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
