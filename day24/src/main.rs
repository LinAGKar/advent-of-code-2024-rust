use std::{collections::HashMap, io::Read};

#[derive(Clone, Copy, Debug)]
enum GateOperator {
    And,
    Or,
    Xor,
}

#[derive(Clone, Copy, Debug)]
enum Gate<'a> {
    Binary(GateOperator, [&'a str; 2], Option<bool>),
    Value(bool),
}

fn evaluate_gate(output: &str, gates: &mut HashMap<&str, Gate>) -> bool {
    match gates.get(output) {
        None => panic!("Gate not found"),
        Some(&Gate::Value(val)) => val,
        Some(&Gate::Binary(_, _, Some(val))) => val,
        Some(&Gate::Binary(operator, [a, b], None)) => {
            let a_val = evaluate_gate(a, gates);
            let b_val = evaluate_gate(b, gates);
            let result = match operator {
                GateOperator::And => a_val & b_val,
                GateOperator::Or => a_val | b_val,
                GateOperator::Xor => a_val ^ b_val,
            };
            *gates.get_mut(output).unwrap() = Gate::Binary(operator, [a, b], Some(result));
            result
        },
    }
}

fn part_1(input: &str) -> u64 {
    let mut lines = input.lines();
    let mut gates = HashMap::new();

    for line in lines.by_ref().take_while(|line| !line.is_empty()) {
        let mut parts = line.split_ascii_whitespace();
        let output = parts.next().unwrap().trim_end_matches(':');
        let value = parts.next().unwrap() == "1";
        gates.insert(output, Gate::Value(value));
    }

    for line in lines {
        let mut parts = line.split_ascii_whitespace();
        let a = parts.next().unwrap();
        let operator = parts.next().unwrap();
        let b = parts.next().unwrap();
        let output = parts.nth(1).unwrap();
        gates.insert(output, Gate::Binary(match operator {
            "AND" => GateOperator::And,
            "OR" => GateOperator::Or,
            "XOR" => GateOperator::Xor,
            _ => panic!("Unknown gate type"),
        }, [a, b], None));
    }

    let mut z_gates: Vec<_> = gates.keys().filter(|&name| name.starts_with('z')).copied().collect();
    z_gates.sort_unstable();
    z_gates.into_iter().rev().fold(0, |acc, name| {
        (acc << 1) | evaluate_gate(name, &mut gates) as u64
    })
}

fn part_2(input: &str) -> String {
    let mut lines = input.lines();
    let mut gates = HashMap::new();
    let mut outputs_by_input = HashMap::new();

    for line in lines.by_ref().take_while(|line| !line.is_empty()) {
        let mut parts = line.split_ascii_whitespace();
        let name = parts.next().unwrap().trim_end_matches(':');
        let value = parts.next().unwrap() == "1";
        gates.insert(name, Gate::Value(value));
    }

    for line in lines {
        let mut parts = line.split_ascii_whitespace();
        let a = parts.next().unwrap();
        let operator = parts.next().unwrap();
        let b = parts.next().unwrap();
        let output = parts.nth(1).unwrap();
        gates.insert(output, Gate::Binary(match operator {
            "AND" => GateOperator::And,
            "OR" => GateOperator::Or,
            "XOR" => GateOperator::Xor,
            _ => panic!("Unknown gate type"),
        }, [a, b], None));
        outputs_by_input.entry(a).or_insert_with(Vec::new).push(output);
        outputs_by_input.entry(b).or_insert_with(Vec::new).push(output);
    }

    let max_z = gates.keys().copied().filter(|&name| name.starts_with('z')).max().unwrap();

    let mut wrong_type: Vec<_> = gates.iter().filter(|&(&output, &gate)| {
        match gate {
            Gate::Value(_) => false,
            Gate::Binary(GateOperator::Xor, [a, _], _) => {
                if a.starts_with(['x', 'y']) && !["x00", "y00"].contains(&a) {
                    if let Some(downstream) = outputs_by_input.get(output) {
                        downstream.len() != 2 ||
                        !downstream.iter().any(|&x| matches!(gates[x], Gate::Binary(GateOperator::Xor, _, _))) ||
                        !downstream.iter().any(|&x| matches!(gates[x], Gate::Binary(GateOperator::And, _, _)))
                    } else {
                        true
                    }
                } else {
                    !output.starts_with('z')
                }
            }

            Gate::Binary(GateOperator::Or, _, _) => {
                if let Some(downstream) = outputs_by_input.get(output) {
                    downstream.len() != 2 ||
                    !downstream.iter().any(|&x| matches!(gates[x], Gate::Binary(GateOperator::Xor, _, _))) ||
                    !downstream.iter().any(|&x| matches!(gates[x], Gate::Binary(GateOperator::And, _, _)))
                } else {
                    output != max_z
                }
            }

            Gate::Binary(GateOperator::And, [a, _], _) => {
                if ["x00", "y00"].contains(&a) {
                    if let Some(downstream) = outputs_by_input.get(output) {
                        downstream.len() != 2 ||
                        !downstream.iter().any(|&x| matches!(gates[x], Gate::Binary(GateOperator::Xor, _, _))) ||
                        !downstream.iter().any(|&x| matches!(gates[x], Gate::Binary(GateOperator::And, _, _)))
                    } else {
                        true
                    }
                } else {
                    if let Some(downstream) = outputs_by_input.get(output) {
                        downstream.len() != 1 ||
                        !matches!(gates[downstream[0]], Gate::Binary(GateOperator::Or, _, _))
                    } else {
                        true
                    }
                }
            }
        }
    }).map(|(&output, _)| output).collect();

    wrong_type.sort();
    wrong_type.join(",")
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
