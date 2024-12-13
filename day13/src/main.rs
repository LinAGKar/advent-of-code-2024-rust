use std::io::Read;

fn extended_euclid(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (d, x, y) = extended_euclid(b, a % b);
        (d, y, x - (a / b) * y)
    }
}

fn solve(input: &str, part_2: bool) -> i64 {
    let press_limit = if part_2 { i64::MAX } else { 100 };
    let offset = if part_2 { 10000000000000 } else { 0 };

    regex::Regex::new(r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)")
            .unwrap().captures_iter(input).map(|cap| {
        let mut nums = cap.iter().skip(1).map(|x| x.unwrap().as_str().parse::<i64>().unwrap());
        let (a_x, a_y, b_x, b_y, p_x, p_y) = (
            nums.next().unwrap(), nums.next().unwrap(), nums.next().unwrap(),
            nums.next().unwrap(), nums.next().unwrap() + offset, nums.next().unwrap() + offset,
        );

        // StarMath syntax (e.g. LibreOffice Math):
        // left [ matrix { a_x # b_x ## a_y # b_y } right ] left [ stack { a # b } right ] = left [ stack { p_x # p_y } right ] newline
        // left [ matrix { a_x # b_x ## a_y # b_y } mline stack { p_x # p_y } right ] newline
        // left [ matrix { 1 # b_x over a_x ## a_y # b_y } mline stack { p_x over a_x # p_y } right ] newline
        // left [ matrix { 1 # b_x over a_x ## 0 # b_y - { a_y b_x } over a_x } mline stack { p_x over a_x # p_y - { a_y p_x } over a_x } right ] newline
        // left [ matrix { 1 # b_x over a_x ## 0 # { a_x b_y - a_y b_x } over a_x } mline stack { p_x over a_x # { a_x p_y - a_y p_x } over a_x } right ] newline
        // left [ matrix { 1 # b_x over a_x ## 0 # 1 } mline stack { p_x over a_x # { a_x p_y - a_y p_x } over { a_x b_y - a_y b_x } } right ] newline
        // left [ matrix { 1 # 0 ## 0 # 1 } mline stack { p_x over a_x - b_x over a_x { a_x p_y - a_y p_x } over { a_x b_y - a_y b_x } # { a_x p_y - a_y p_x } over { a_x b_y - a_y b_x } } right ] newline
        // left [ matrix { 1 # 0 ## 0 # 1 } mline stack { p_x over a_x - { b_x p_y - a_y b_x p_x over a_x } over { a_x b_y - a_y b_x } # { a_x p_y - a_y p_x } over { a_x b_y - a_y b_x } } right ] newline
        // left [ matrix { 1 # 0 ## 0 # 1 } mline stack { { p_x ( a_x b_y - a_y b_x ) - a_x ( b_x p_y - a_y b_x p_x over a_x) } over { a_x ( a_x b_y - a_y b_x ) } # { a_x p_y - a_y p_x } over { a_x b_y - a_y b_x } } right ] newline
        // left [ matrix { 1 # 0 ## 0 # 1 } mline stack { { a_x b_y p_x - a_y b_x p_x - a_x b_x p_y + a_y b_x p_x } over { a_x ( a_x b_y - a_y b_x ) } # { a_x p_y - a_y p_x } over { a_x b_y - a_y b_x } } right ] newline
        // left [ matrix { 1 # 0 ## 0 # 1 } mline stack { { a_x ( b_y p_x - b_x p_y ) } over { a_x ( a_x b_y - a_y b_x ) } # { a_x p_y - a_y p_x } over { a_x b_y - a_y b_x } } right ] newline
        // left [ matrix { 1 # 0 ## 0 # 1 } mline stack { { b_y p_x - b_x p_y } over { a_x b_y - a_y b_x } # { a_x p_y - a_y p_x } over { a_x b_y - a_y b_x } } right ] newline

        let denominator = a_x * b_y - a_y * b_x;

        if denominator != 0 {
            let numerator_a = b_y * p_x - b_x * p_y;
            let numerator_b = a_x * p_y - a_y * p_x;
            let presses_a = numerator_a / denominator;
            let presses_b = numerator_b / denominator;

            if numerator_a % denominator == 0 && numerator_b % denominator == 0 &&
               presses_a >= 0 && presses_b >= 0 && presses_a <= press_limit && presses_b <= press_limit {
                presses_a * 3 + presses_b
            } else {
                // Would need non-integer, negative or too many presses
                0
            }
        } else if a_x * p_y != a_y * p_x {
            // Both vectors point in the same direction, but not toward prize
            0
        } else {
            // Both vectors point toward prize
            // Only need to consider one dimension, since the other is a uniform multiple
            // Find integer solutions for a_x * presses_a + b_x * presses_b = p_x
            let (a_x, b_x, p_x) = if p_x != 0 {
                (a_x, b_x, p_x)
            } else {
                // Use y-axis if there is no movement on x-axis
                (a_y, b_y, p_y)
            };

            let (gcd, a_0, b_0) = extended_euclid(a_x, b_x);
            if (p_x % gcd) != 0 {
                return 0;
            }
            let a_x_simplified = a_x / gcd;
            let b_x_simplified = b_x / gcd;
            let p_x_simplified = p_x / gcd;
            let a_p = a_0 * p_x_simplified;
            let b_p = b_0 * p_x_simplified;
            // presses_a = a_p + b_x_simplified * k, k ∈ ℤ
            // presses_b = b_p - a_x_simplified * k, k ∈ ℤ

            let max_presses_a = std::cmp::min(p_x / a_x, press_limit);
            let max_presses_b = std::cmp::min(p_x / b_x, press_limit);
            let min_k = -a_p.div_euclid(b_x_simplified);
            let max_k = b_p.div_euclid(a_x_simplified);
            let min_presses_a = a_p + b_x_simplified * min_k;
            let min_presses_b = b_p - a_x_simplified * max_k;
            let max_k_2 = std::cmp::min(min_k + (max_presses_a - min_presses_a).div_euclid(b_x_simplified), max_k);
            let min_k_2 = std::cmp::max(max_k - (max_presses_b - min_presses_b).div_euclid(a_x_simplified), min_k);
            if min_k_2 > max_k_2 {
                return 0;
            }

            let k = if a_x > b_x * 3 {
                max_k_2
            } else {
                min_k_2
            };

            let presses_a = a_p + b_x_simplified * k;
            let presses_b = b_p - a_x_simplified * k;

            presses_a * 3 + presses_b
        }
    }).sum()
}

fn part_1(input: &str) -> i64 {
    solve(input, false)
}

fn part_2(input: &str) -> i64 {
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
