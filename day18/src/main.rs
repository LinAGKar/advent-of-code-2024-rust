use std::{collections::VecDeque, io::Read};

const SIZE: usize = 71;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Wall,
    Empty,
    Visited,
}

fn find_path(start: [usize; 2], map: &mut [[Tile; SIZE]; SIZE]) -> Option<u16> {
    let end = [SIZE - 1, SIZE - 1];

    let mut to_check = VecDeque::new();
    to_check.push_back((start, 0));
    map[0][0] = Tile::Visited;

    while let Some((pos, steps)) = to_check.pop_front() {
        if pos == end {
            return Some(steps);
        }

        let [x, y] = pos;

        for [x, y] in [
            [x.wrapping_sub(1), y],
            [x + 1, y],
            [x, y.wrapping_sub(1)],
            [x, y + 1],
        ] {
            if x >= SIZE || y >= SIZE {
                continue;
            }

            match map[x][y] {
                Tile::Wall => {},
                Tile::Visited => {},
                Tile::Empty => {
                    map[x][y] = Tile::Visited;
                    to_check.push_back(([x, y], steps + 1))
                }
            }
        }
    }

    None
}

fn part_1(input: &str) -> u16 {
    let mut map = [[Tile::Empty; SIZE]; SIZE];
    for line in input.lines().take(1024) {
        let mut nums = line.split(',').map(|x| x.parse::<usize>().unwrap());
        let x = nums.next().unwrap();
        let y = nums.next().unwrap();
        map[x][y] = Tile::Wall;
    }

    find_path([0, 0], &mut map).unwrap()
}

fn part_2(input: &str) -> String {
    let mut map = [[Tile::Empty; SIZE]; SIZE];
    let bytes: Vec<_> = input.lines().map(|line| {
        let mut nums = line.split(',').map(|x| x.parse::<usize>().unwrap());
        let x = nums.next().unwrap();
        let y = nums.next().unwrap();
        [x, y]
    }).collect();

    for &[x, y] in &bytes {
        map[x][y] = Tile::Wall;
    }

    find_path([0, 0], &mut map);
    for [x, y] in bytes.into_iter().rev() {
        map[x][y] = Tile::Empty;

        if [
            [x.wrapping_sub(1), y],
            [x + 1, y],
            [x, y.wrapping_sub(1)],
            [x, y + 1],
        ].into_iter().any(|[x, y]| {
            x < SIZE && y < SIZE && map[x][y] == Tile::Visited
        }) && find_path([x, y], &mut map).is_some() {
            return format!("{},{}", x, y);
        }
    }

    panic!()
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
