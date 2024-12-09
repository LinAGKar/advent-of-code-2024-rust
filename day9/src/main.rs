use std::collections::VecDeque;

#[derive(Clone, Copy)]
enum Area {
    File(u16),
    Empty,
}

fn part_1(input: &str) -> u64 {
    let mut disk: VecDeque<_> = input.trim().chars().enumerate().map(|(n, c)| {
        let size = c.to_digit(10).unwrap() as u8;
        let is_file = n % 2 == 0;
        if is_file {
            let file_id = (n / 2) as u16;
            (size, Area::File(file_id))
        } else {
            (size, Area::Empty)
        }
    }).collect();

    let mut position = 0;
    let mut result = 0;

    while let Some((size, block_type)) = disk.pop_front() {
        match block_type {
            Area::File(id) => {
                let size = size as u64;
                let start_pos = position;
                let end_pos = position + size - 1;
                let tot_block_positions = (end_pos + start_pos) * size / 2;
                result += tot_block_positions * id as u64;
                position += size;
            }
            Area::Empty => {
                let mut remaining_space = size as u64;
                while remaining_space > 0 {
                    match disk.back_mut() {
                        None => remaining_space = 0,
                        Some((_, Area::Empty)) => {
                            disk.pop_back();
                        }
                        Some((file_size_ref, Area::File(id))) => {
                            let file_size = *file_size_ref as u64;
                            let taken_from_file = std::cmp::min(remaining_space, *file_size_ref as u64);
                            let start_pos = position;
                            let end_pos = position + taken_from_file - 1;
                            let tot_block_positions = (end_pos + start_pos) * taken_from_file / 2;
                            result += tot_block_positions * *id as u64;
                            remaining_space -= taken_from_file;
                            position += taken_from_file;
                            if taken_from_file == file_size {
                                disk.pop_back();
                            } else {
                                let new_file_size = file_size - taken_from_file;
                                *file_size_ref = new_file_size as u8;
                            }
                        }
                    }
                }
            }
        }
    }

    result
}

fn part_2(input: &str) -> u64 {
    let mut sizes = input.trim().chars().map(|c|
        c.to_digit(10).unwrap() as u8
    );
    let mut position = 0;
    let mut files = Vec::new();
    let mut empty_spaces = Vec::new();
    for id in 0u16.. {
        if let Some(size) = sizes.next() {
            files.push((position, size, id));
            position += size as u32;
        } else {
            break;
        }
        if let Some(size) = sizes.next() {
            empty_spaces.push((position, size));
            position += size as u32;
        } else {
            break;
        }
    }

    let mut result = 0;

    let mut first_space_by_size = [None; 10];
    for (size, pos) in first_space_by_size.iter_mut().enumerate() {
        *pos = empty_spaces.iter().enumerate().find_map(|(index, &(_, space_size))| if space_size == size as u8 {
            Some(index as u16)
        } else {
            None
        });
    }

    while let Some((position, size, id)) = files.pop() {
        let position = if let Some(space_index_ref) = first_space_by_size
                .iter_mut()
                .skip(size as usize)
                .filter(|space_index| space_index.map_or(false, |space_index| space_index < files.len() as u16))
                .min() {
            let space_index = space_index_ref.unwrap();
            let &mut (_, space_size) = &mut empty_spaces[space_index as usize];
            *space_index_ref = empty_spaces
                .iter()
                .enumerate()
                .skip(space_index as usize + 1)
                .find_map(|(index, &(_, other_space_size))| if other_space_size == space_size {
                    Some(index as u16)
                } else {
                    None
                });
            let (space_pos, space_size) = &mut empty_spaces[space_index as usize];
            *space_size -= size;
            let old_space_pos = *space_pos;
            *space_pos += size as u32;
            if first_space_by_size[*space_size as usize].map_or(true, |first_space_index| space_index < first_space_index) {
                first_space_by_size[*space_size as usize] = Some(space_index);
            }
            old_space_pos
        } else {
            position
        } as u64;

        let size = size as u64;
        let start_pos = position;
        let end_pos = position + size - 1;
        let tot_block_positions = (end_pos + start_pos) * size / 2;
        result += tot_block_positions * id as u64;
    }

    result
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let start_time = std::time::Instant::now();
    let result = part_1(&input);
    println!("Part 1 time: {:?}", std::time::Instant::now() - start_time);
    println!("Part 1 result: {}", result);

    let start_time = std::time::Instant::now();
    let result = part_2(&input);
    println!("Part 2 time: {:?}", std::time::Instant::now() - start_time);
    println!("Part 2 result: {}", result);
}
