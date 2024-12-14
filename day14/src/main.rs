use std::io::Read;

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;
const HALF_WIDTH: i32 = WIDTH / 2;
const HALF_HEIGHT: i32 = HEIGHT / 2;

fn part_1(input: &str) -> u32 {
    let mut robots_per_quadrants = [0; 4];

    for line in input.lines() {
        let mut nums = line
            .split(&['p', 'v', '=', ',', ' '])
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i32>().unwrap());

        let px = nums.next().unwrap();
        let py = nums.next().unwrap();
        let vx = nums.next().unwrap();
        let vy = nums.next().unwrap();
        let px = (px + vx * 100).rem_euclid(WIDTH);
        let py = (py + vy * 100).rem_euclid(HEIGHT);

        if let Some(pos) = match (px, py) {
            (HALF_WIDTH, _) => None,
            (_, HALF_HEIGHT) => None,
            (..HALF_WIDTH, ..HALF_HEIGHT) => Some(0),
            (HALF_WIDTH.., ..HALF_HEIGHT) => Some(1),
            (..HALF_WIDTH, HALF_HEIGHT..) => Some(2),
            (HALF_WIDTH.., HALF_HEIGHT..) => Some(3),
        } {
            robots_per_quadrants[pos] += 1;
        }
    }

    robots_per_quadrants.into_iter().product()
}

fn part_2(input: &str) -> u16 {
    let mut robots: Vec<_> = input.lines().map(|line| {
        let mut nums = line
            .split(&['p', 'v', '=', ',', ' '])
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i32>().unwrap());

        let px = nums.next().unwrap();
        let py = nums.next().unwrap();
        let vx = nums.next().unwrap();
        let vy = nums.next().unwrap();
        [px, py, vx, vy]
    }).collect();

    let mut occupied_tiles = [[0u16; HEIGHT as usize]; WIDTH as usize];
    let mut t = 0;

    loop {
        t += 1;

        for robot in &mut robots {
            robot[0] = (robot[0] + robot[2]).rem_euclid(WIDTH);
            robot[1] = (robot[1] + robot[3]).rem_euclid(HEIGHT);

            occupied_tiles[robot[0] as usize][robot[1] as usize] = t;
        }

        let adjacent_robots = robots.iter().filter(|&&[x, y, _, _]| {
            [
                [x + 1, y],
                [x - 1, y],
                [x, y + 1],
                [x, y - 1],
            ].into_iter().any(|[x, y]| {
                x >= 0 && x < WIDTH && y >= 0 && y < HEIGHT && occupied_tiles[x as usize][y as usize] == t
            })
        }).count();

        if adjacent_robots > robots.len() / 2 {
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    print!("{}", if occupied_tiles[x as usize][y as usize] == t { '█' } else { ' ' });
                }
                println!();
            }

            break t;
        }
    }
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
