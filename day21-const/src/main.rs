use std::io::Read;

#[derive(Debug, Clone, Copy)]
enum DirectionKey {
    Up = 0,
    Activate = 1,
    Left = 2,
    Down = 3,
    Right = 4,
}

#[derive(Clone, Copy)]
struct Path {
    presses: [DirectionKey; 6],
    len: usize,
}

impl Path {
    const fn push(&mut self, key: DirectionKey) {
        self.presses[self.len] = key;
        self.len += 1;
    }
}

#[derive(Clone, Copy)]
struct Paths {
    paths: [Path; 2],
    len: usize,
}

impl Paths {
    const fn new() -> Self {
        Self {
            paths: [Path { presses: [DirectionKey::Activate; 6], len: 0 }; 2],
            len: 0,
        }
    }
}

const fn calc_level_costs(previous_costs: &[u64; 25], paths: &[Paths; 25]) -> [u64; 25] {
    let mut costs = [u64::MAX; 25];

    let mut i = 0;
    while i < previous_costs.len() {
        let mut j = 0;
        while j < paths[i].len {
            let path = &paths[i].paths[j];
            let mut pos = DirectionKey::Activate;
            let mut cost = 0;
            let mut k = 0;
            while k < path.len {
                let new_pos = path.presses[k];
                cost += previous_costs[pos as usize * 5 + new_pos as usize];
                pos = new_pos;
                k += 1;
            }
            if cost < costs[i] {
                costs[i] = cost;
            }
            j += 1;
        }
        i += 1;
    }

    costs
}

const fn get_paths<const HOLE_Y: u8>(paths: &mut Paths, key_positions: &[[u8; 2]], start: usize, end: usize) {
    let [start_x, start_y] = key_positions[start];
    let [end_x, end_y] = key_positions[end];

    if !(start_x == 0 && end_y == HOLE_Y) {
        // Start by going vertically and then horizontally
        // This must not be done if we start on the left button and go to the top row, as that would make us pass
        // over an empty space.
        let path = &mut paths.paths[paths.len];
        paths.len += 1;
        if start_y < end_y {
            let mut i = start_y;
            while i < end_y {
                path.push(DirectionKey::Down);
                i += 1;
            }
        } else if start_y > end_y {
            let mut i = end_y;
            while i < start_y {
                path.push(DirectionKey::Up);
                i += 1;
            }
        }
        if start_x < end_x {
            let mut i = start_x;
            while i < end_x {
                path.push(DirectionKey::Right);
                i += 1;
            }
        } else if start_x > end_x {
            let mut i = end_x;
            while i < start_x {
                path.push(DirectionKey::Left);
                i += 1;
            }
        }
        // We always need to end with Activate, so we actually press the button we go to.
        path.push(DirectionKey::Activate);
    }

    if start_x != end_x && start_y != end_y && !(start_y == HOLE_Y && end_x == 0) {
        // If we need to both vertically and horizontally, we can also do it by going horizontally first.
        // This must not be done if we end on the left button, as that would make us pass over an empty space.
        let path = &mut paths.paths[paths.len];
        paths.len += 1;
        if start_x < end_x {
            let mut i = start_x;
            while i < end_x {
                path.push(DirectionKey::Right);
                i += 1;
            }
        } else if start_x > end_x {
            let mut i = end_x;
            while i < start_x {
                path.push(DirectionKey::Left);
                i += 1;
            }
        }
        if start_y < end_y {
            let mut i = start_y;
            while i < end_y {
                path.push(DirectionKey::Down);
                i += 1;
            }
        } else if start_y > end_y {
            let mut i = end_y;
            while i < start_y {
                path.push(DirectionKey::Up);
                i += 1;
            }
        }
        // We always need to end with Activate, so we actually press the button we go to.
        path.push(DirectionKey::Activate);
    }

    // It is never worth zigzagging, as such paths can be reduced into a non-zigzagging path just by duplicating
    // some presses while eliminating others, to get a path that takes fewer presses in total.
}

const fn calc_directional_key_costs(robot_keypads: u8) -> [u64; 25] {
    // Where each key is located on the directional keypad
    let direction_key_positions = [
        [1, 0],
        [2, 0],
        [0, 1],
        [1, 1],
        [2, 1],
    ];

    // Possible button inputs required to get the robot at the next level to press any button from any starting position
    let mut direction_key_paths = [Paths::new(); 25];
    let mut i = 0;
    while i < direction_key_paths.len() {
        let start = i / 5;
        let end = i % 5;
        get_paths::<0>(&mut direction_key_paths[i], &direction_key_positions, start, end);
        i += 1;
    }

    // How many button presses it takes to get to any button from any other button and then press it
    let mut path_costs: [u64; 25] = [0; 25];
    let mut i = 0;
    while i < path_costs.len() {
        path_costs[i] = u64::MAX;
        let mut j = 0;
        while j < direction_key_paths[i].len {
            let cost = direction_key_paths[i].paths[j].len as u64;
            if cost < path_costs[i] {
                path_costs[i] = cost;
            }
            j += 1;
        }
        i += 1;
    }

    let mut i = 0;
    while i < robot_keypads - 1 {
        path_costs = calc_level_costs(&path_costs, &direction_key_paths);
        i += 1;
    }

    path_costs
}

const fn calc_numeric_input_costs(robot_keypads: u8) -> [u64; 11 * 11] {
    let directional_key_costs = calc_directional_key_costs(robot_keypads);

    // Where each key is located on the numeric keypad.
    // Elements represent positions of 0-9 followed by A
    let numeric_key_positions = [
        [1, 3],
        [0, 2],
        [1, 2],
        [2, 2],
        [0, 1],
        [1, 1],
        [2, 1],
        [0, 0],
        [1, 0],
        [2, 0],
        [2, 3],
    ];

    // Possible button inputs required to get the robot to press any button from any starting position
    let mut costs = [u64::MAX; 11 * 11];
    let mut paths = Paths::new();
    let mut i = 0;
    while i < costs.len() {
        let start = i / 11;
        let end = i % 11;
        get_paths::<3>(&mut paths, &numeric_key_positions, start, end);

        let mut j = 0;
        while j < paths.len {
            let mut pos = DirectionKey::Activate;
            let mut cost = 0;
            let mut k = 0;
            while k < paths.paths[j].len {
                let new_pos = paths.paths[j].presses[k];
                cost += directional_key_costs[pos as usize * 5 + new_pos as usize];
                pos = new_pos;
                k += 1;
            }
            if cost < costs[i] {
                costs[i] = cost;
            }
            paths.paths[j].len = 0;
            j += 1;
        }
        paths.len = 0;
        i += 1;
    }

    costs
}

fn calculate_combo_complexities(input: &str, numeric_key_costs: &[u64]) -> u64 {
    input.lines().map(|combo| {
        let number: u64 = combo.split('A').next().unwrap().parse().unwrap();

        let mut pos = 10;
        combo.chars().map(|key| {
            let new_pos = match key {
                '0'..='9' => key as usize - '0' as usize,
                'A' => 10,
                _ => panic!("Invalid character: {}", key),
            };
            let cost = numeric_key_costs[pos * 11 + new_pos];
            pos = new_pos;
            cost
        }).sum::<u64>() * number
    }).sum()
}

fn part_1(input: &str) -> u64 {
    const NUMERIC_KEY_COSTS: [u64; 11 * 11] = calc_numeric_input_costs(2);
    calculate_combo_complexities(input, &NUMERIC_KEY_COSTS)
}

fn part_2(input: &str) -> u64 {
    const NUMERIC_KEY_COSTS: [u64; 11 * 11] = calc_numeric_input_costs(25);
    calculate_combo_complexities(input, &NUMERIC_KEY_COSTS)
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
