use std::io::Read;

#[derive(Debug, Clone, Copy)]
enum DirectionKey {
    Up = 0,
    Activate = 1,
    Left = 2,
    Down = 3,
    Right = 4,
}

fn calc_level_costs(previous_costs: &[u64], new_costs: &mut [u64], paths: &[Vec<Vec<DirectionKey>>]) {
    for (paths, new_cost) in paths.iter().zip(new_costs) {
        *new_cost = paths.iter().map(|path| {
            // Sum up the costs of going from each button to the next one and pressing it, starting from Activate
            let mut pos = DirectionKey::Activate;
            path.iter().map(|&new_pos| {
                let cost = previous_costs[pos as usize * 5 + new_pos as usize];
                pos = new_pos;
                cost
            }).sum()
        }).min().unwrap()
    }
}

fn get_paths<const HOLE_Y: u8>(paths: &mut Vec<Vec<DirectionKey>>, direction_key_positions: &[[u8; 2]], start: usize, end: usize) {
    let [start_x, start_y] = direction_key_positions[start];
    let [end_x, end_y] = direction_key_positions[end];

    if !(start_x == 0 && end_y == HOLE_Y) {
        // Start by going vertically and then horizontally
        // This must not be done if we start on the left button and go to the top row, as that would make us pass
        // over an empty space.
        let mut path = Vec::new();
        if start_y < end_y {
            path.extend((start_y..end_y).map(|_| DirectionKey::Down));
        } else if start_y > end_y {
            path.extend((end_y..start_y).map(|_| DirectionKey::Up));
        }
        if start_x < end_x {
            path.extend((start_x..end_x).map(|_| DirectionKey::Right));
        } else if start_x > end_x {
            path.extend((end_x..start_x).map(|_| DirectionKey::Left));
        }
        // We always need to end with Activate, so we actually press the button we go to.
        path.push(DirectionKey::Activate);
        paths.push(path);
    }

    if start_x != end_x && start_y != end_y && !(start_y == HOLE_Y && end_x == 0) {
        // If we need to both vertically and horizontally, we can also do it by going horizontally first.
        // This must not be done if we end on the left button, as that would make us pass over an empty space.
        let mut path = Vec::new();
        if start_x < end_x {
            path.extend((start_x..end_x).map(|_| DirectionKey::Right));
        } else if start_x > end_x {
            path.extend((end_x..start_x).map(|_| DirectionKey::Left));
        }
        if start_y < end_y {
            path.extend((start_y..end_y).map(|_| DirectionKey::Down));
        } else if start_y > end_y {
            path.extend((end_y..start_y).map(|_| DirectionKey::Up));
        }
        // We always need to end with Activate, so we actually press the button we go to.
        path.push(DirectionKey::Activate);
        paths.push(path);
    }

    // It is never worth zigzagging, as such paths can be reduced into a non-zigzagging path just by duplicating
    // some presses while eliminating others, to get a path that takes fewer presses in total.
}

fn calc_directional_key_costs<const ROBOT_KEYPADS: u8>() -> Vec<u64> {
    // Where each key is located on the directional keypad
    let direction_key_positions = [
        [1, 0],
        [2, 0],
        [0, 1],
        [1, 1],
        [2, 1],
    ];

    // Possible button inputs required to get the robot at the next level to press any button from any starting position
    let direction_key_paths: Vec<_> = (0..(5 * 5)).map(|i| {
        let mut paths = Vec::new();
        let start = i / 5;
        let end = i % 5;
        get_paths::<0>(&mut paths, &direction_key_positions, start, end);
        paths
    }).collect();

    // How many button presses it takes to get to any button from any other button and then press it
    let mut path_costs: Vec<u64> = direction_key_paths.iter().map(|paths| {
        paths.iter().map(|path| path.len() as u64).min().unwrap()
    }).collect();

    let mut new_costs = vec![0; 5 * 5];
    for _ in 0..ROBOT_KEYPADS - 1 {
        calc_level_costs(&path_costs, &mut new_costs, &direction_key_paths);
        std::mem::swap(&mut path_costs, &mut new_costs);
    }

    path_costs
}

fn calculate_combo_complexities<const ROBOT_KEYPADS: u8>(input: &str) -> u64 {
    let directional_key_costs = calc_directional_key_costs::<ROBOT_KEYPADS>();

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

    let mut paths = Vec::new();
    input.lines().map(|combo| {
        let number: u64 = combo.split('A').next().unwrap().parse().unwrap();

        let mut pos = numeric_key_positions.len() - 1;
        combo.chars().map(|key| {
            let new_pos = match key {
                '0'..='9' => key as usize - '0' as usize,
                'A' => 10,
                _ => panic!("Invalid character: {}", key),
            };
            get_paths::<3>(&mut paths, &numeric_key_positions, pos, new_pos);
            let cost: u64 = paths.iter().map(|path| {
                let mut pos = DirectionKey::Activate;
                path.iter().map(|&new_pos| {
                    let cost = directional_key_costs[pos as usize * 5 + new_pos as usize];
                    pos = new_pos;
                    cost
                }).sum()
            }).min().unwrap();
            pos = new_pos;
            paths.clear();
            cost
        }).sum::<u64>() * number
    }).sum()
}

fn part_1(input: &str) -> u64 {
    calculate_combo_complexities::<2>(input)
}

fn part_2(input: &str) -> u64 {
    calculate_combo_complexities::<25>(input)
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
