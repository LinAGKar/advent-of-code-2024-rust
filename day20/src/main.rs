use std::{collections::VecDeque, io::Read};

enum Tile {
    Track([Option<u16>; 2]),
    Wall,
}

fn calculate_distances(map: &mut Vec<Tile>, width: usize, index: usize, start: usize, end: usize) {
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    while let Some((pos, dist)) = queue.pop_front() {
        if let Tile::Track(dists) = &mut map[pos] {
            if dists[index].is_none() {
                dists[index] = Some(dist);
                for new_pos in [
                    pos + 1,
                    pos - 1,
                    pos + width,
                    pos - width,
                    ] {
                    queue.push_back((new_pos, dist + 1))
                }
            }
        }

        if pos == end {
            break;
        }
    }
}

fn calculate_map(input: &str) -> (Vec<Tile>, usize, u16) {
    let width = input.chars().position(|c| c == '\n').unwrap();
    let mut start = 0;
    let mut end = 0;
    let mut map = input.chars().filter(|&c| c != '\n').enumerate().map(|(n, c)| {
        match c {
            '.' => Tile::Track([None; 2]),
            '#' => Tile::Wall,
            'S' => {
                start = n;
                Tile::Track([None; 2])
            },
            'E' => {
                end = n;
                Tile::Track([None; 2])
            },
            _ => panic!("Invalid character: {}", c),
        }
    }).collect();

    calculate_distances(&mut map, width, 0, start, end);
    calculate_distances(&mut map, width, 1, end, start);

    let best_distance = match map[end] {
        Tile::Track([Some(dist), _]) => dist,
        _ => panic!("No path found"),
    };

    (map, width, best_distance)
}

const SAVE_DISTANCE: u16 = 100;

fn part_1(input: &str) -> usize {
    let (ref map, width, best_distance) = calculate_map(input);

    (1..map.len() / width - 1).flat_map(|y| (1..width - 1).filter_map(move |x| {
        let pos = y * width + x;
        if let Tile::Wall = map[pos] {
            let surrounding = [
                &map[pos - 1],
                &map[pos + 1],
                &map[pos - width],
                &map[pos + width],
            ];

            Some(surrounding.iter().flat_map(|&a| surrounding.iter().filter(move |&&b| {
                if let (Tile::Track([Some(dist_from_start), _]), Tile::Track([_, Some(dist_from_end)])) = (a, b) {
                    dist_from_start + dist_from_end + 2 <= best_distance - SAVE_DISTANCE
                } else {
                    false
                }
            })).count())
        } else {
            None
        }
    })).sum()
}

const CHEAT_DIST: usize = 20;

fn part_2(input: &str) -> usize {
    let (ref map, width, best_distance) = calculate_map(input);
    let height = map.len() / width;

    let result = (1..height - 1).flat_map(|y| {
        (1..width - 1).filter_map(move |x| {
            let pos = y * width + x;
            if let Tile::Track([Some(dist_from_start), _]) = map[pos] {
                Some(((std::cmp::max(x, CHEAT_DIST + 1) - CHEAT_DIST)..=(std::cmp::min(x + CHEAT_DIST, width - 2))).flat_map(|x2| {
                    let x_dist = x.abs_diff(x2);
                    let max_y_dist = CHEAT_DIST - x_dist;
                    ((std::cmp::max(y, max_y_dist + 1) - max_y_dist)..=(std::cmp::min(y + max_y_dist, height - 2))).filter(move |&y2| {
                        let pos2 = y2 * width + x2;
                        if let Tile::Track([_, Some(dist_from_end)]) = map[pos2] {
                            dist_from_start + dist_from_end + x_dist as u16 + y.abs_diff(y2) as u16 <= best_distance - SAVE_DISTANCE
                        } else {
                            false
                        }
                    })
                }).count())
            } else {
                None
            }
        })
    }).sum();

    result
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
