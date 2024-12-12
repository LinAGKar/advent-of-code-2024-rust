use std::io::Read;

fn solve(input: &str, bulk_discount: bool) -> u32 {
    let map: Vec<_> = input.chars().filter(|&c| c != '\n').map(|c| c as u32 as u8).collect();
    let width = input.chars().position(|c| c == '\n').unwrap();
    let height = map.len() / width;

    let mut checked = vec![false; map.len()];
    let mut to_check = Vec::new();

    let coord_to_index = |[x, y]: [usize; 2]| -> usize {
        y * width + x
    };

    let mut tot = 0;

    for start_y in 0..height {
        for start_x in 0..width {
            let start = [start_x, start_y];
            if checked[coord_to_index(start)] {
                continue;
            }

            let mut area = 0;
            let mut circumference = 0;
            let mut corners = 0;

            to_check.push(start);
            checked[coord_to_index(start)] = true;
            let plant = map[coord_to_index(start)];

            let is_same_plant = |[x, y]: [usize; 2]| -> bool {
                x < width && y < height && map[coord_to_index([x, y])] == plant
            };

            while let Some([x, y]) = to_check.pop() {
                area += 1;

                let adjacent = [
                    [x.wrapping_sub(1), y],
                    [x, y.wrapping_sub(1)],
                    [x + 1, y],
                    [x, y + 1],
                ];

                let adjacent_same = [
                    is_same_plant(adjacent[0]),
                    is_same_plant(adjacent[1]),
                    is_same_plant(adjacent[2]),
                    is_same_plant(adjacent[3]),
                ];

                if bulk_discount {

                    let diagonally_adjacent = [
                        [x.wrapping_sub(1), y.wrapping_sub(1)],
                        [x + 1, y.wrapping_sub(1)],
                        [x + 1, y + 1],
                        [x.wrapping_sub(1), y + 1],
                    ];

                    for i in 0..4 {
                        if !adjacent_same[i] && !adjacent_same[(i + 1) % 4] {
                            // Convex corner
                            corners += 1;
                        } else if adjacent_same[i] && adjacent_same[(i + 1) % 4] &&
                                  !is_same_plant(diagonally_adjacent[i]) {
                            // Concave corner
                            corners += 1;
                        }
                    }
                }

                for (pos, same) in adjacent.into_iter().zip(adjacent_same) {
                    if !same {
                        circumference += 1;
                    } else if !checked[coord_to_index(pos)] {
                        to_check.push(pos);
                        checked[coord_to_index(pos)] = true;
                    }
                }
            }

            tot += area * if bulk_discount {
                corners
            } else {
                circumference
            };
        }
    }

    tot
}


fn part_1(input: &str) -> u32 {
    solve(input, false)
}

fn part_2(input: &str) -> u32 {
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
