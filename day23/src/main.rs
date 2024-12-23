use std::{collections::HashMap, io::Read};

fn parse_graph(input: &str) -> (Vec<&str>, Vec<Vec<usize>>) {
    let mut computer_names = Vec::<&str>::new();
    let mut computer_by_name = HashMap::new();
    let mut connections = Vec::new();

    let mut get_computer_index = |name| {
        if let Some(&index) = computer_by_name.get(&name) {
            index
        } else {
            let index = computer_names.len();
            computer_names.push(name);
            computer_by_name.insert(name, index);
            index
        }
    };

    for line in input.lines() {
        let mut parts = line.split("-");
        let from = get_computer_index(parts.next().unwrap());
        let to = get_computer_index(parts.next().unwrap());
        let max = std::cmp::max(from, to);
        if connections.len() <= max {
            connections.resize_with(max + 1, Vec::new);
        }
        if to > from {
            connections[from].push(to);
        } else if from > to {
            connections[to].push(from);
        }
    }

    for connections in &mut connections {
        connections.sort_unstable();
    }

    (computer_names, connections)
}

fn part_1(input: &str) -> u16 {
    let (computer_names, connections) = parse_graph(input);

    let t_start = |index: usize| {
        computer_names[index].starts_with("t")
    };

    connections.iter().enumerate().map(|(a, a_connections)| {
        a_connections.iter().map(|&b| {
            connections[b].iter().filter(|&&c| {
                a_connections.binary_search(&c).is_ok() && (t_start(a) || t_start(b) || t_start(c))
            }).count() as u16
        }).sum::<u16>()
    }).sum()
}

fn get_largest_group(largest_group: &mut Vec<usize>, current_group: &mut Vec<usize>, connections: &Vec<Vec<usize>>) {
    let &at = current_group.last().unwrap();

    for &next_computer in &connections[at] {
        if !current_group.iter().take(current_group.len() - 1).all(|&previous_computer| {
            connections[previous_computer].binary_search(&next_computer).is_ok()
        }) {
            continue;
        }

        current_group.push(next_computer);
        get_largest_group(largest_group, current_group, connections);
        current_group.pop();
    }

    if current_group.len() > largest_group.len() {
        largest_group.clear();
        largest_group.extend_from_slice(&current_group);
    }
}

fn part_2(input: &str) -> String {
    let (computer_names, connections) = parse_graph(input);

    let mut largest_group = Vec::new();
    let mut current_group = Vec::new();
    for i in 0..connections.len() {
        current_group.push(i);
        get_largest_group(&mut largest_group, &mut current_group, &connections);
        current_group.clear();
    }

    let mut group_names: Vec<_> = largest_group.into_iter().map(|i| computer_names[i]).collect();
    group_names.sort_unstable();
    group_names.join(",")
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
