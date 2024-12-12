use std::collections::{HashMap, HashSet};

advent_of_code::solution!(9);

fn disk_map_to_mem_blocks(input: &str) -> Vec<i32> {
    let mut blocks: Vec<Vec<i32>> = vec![];
    let mut index = 0;
    for (i, value) in input.chars().enumerate() {
        let value = value.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            blocks.push(vec![index; value]);
            index += 1;
        } else {
            blocks.push(vec![-1; value]);
        }
    }

    blocks.iter().flatten().cloned().collect::<Vec<_>>()
}

fn align_file_blocks(blocks: &mut Vec<i32>) {
    let index_of_blanks = blocks
        .iter()
        .enumerate()
        .filter_map(|(i, v)| if *v == -1 { Some(i) } else { None })
        .collect::<Vec<_>>();

    for index in index_of_blanks {
        // Find the first non-blank block
        while *blocks.last().unwrap() == -1 {
            blocks.pop().unwrap();
        }

        // Break if blocks len is less then the index
        if blocks.len() <= index {
            break;
        }

        // Swap the non-blank block with the blank block
        blocks[index] = blocks.pop().unwrap();
    }
}

fn checksum(blocks: &Vec<i32>) -> i64 {
    let mut sum: i64 = 0;
    for (index, &block) in blocks.iter().enumerate() {
        let block: i64 = block.try_into().unwrap();
        let index: i64 = index.try_into().unwrap();
        let product: i64 = index.checked_mul(block).unwrap();
        sum = sum.checked_add(product.into()).unwrap();
    }
    sum
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut blocks = disk_map_to_mem_blocks(input);

    align_file_blocks(&mut blocks);

    Some(checksum(&blocks))
}

fn is_empty(file: &[i32]) -> bool {
    file.iter().all(|&v| v == -1)
}

fn is_file(file: &[i32]) -> bool {
    file.iter().all(|&v| v != -1)
}

fn is_segmented(file: &[i32]) -> bool {
    !is_empty(file) && !is_file(file)
}

fn pack_file_blocks(blocks: &mut Vec<i32>) -> Vec<Vec<i32>> {
    let mut div_blocks = vec![];
    let mut curr_file = vec![];
    for &val in blocks.iter() {
        if curr_file.last().is_none() || curr_file.last().unwrap() == &val {
            curr_file.push(val);
        } else {
            div_blocks.push(curr_file);
            curr_file = vec![val];
        }
    }

    // Push the last group if it exists
    if !curr_file.is_empty() {
        div_blocks.push(curr_file);
    }

    div_blocks
}

fn align_full_file_blocks(blocks: Vec<i32>) -> Vec<i32> {
    let mut div_blocks = pack_file_blocks(&mut blocks.clone());

    let mut right = div_blocks.len() - 1;
    let mut left = 0usize;
    while right > 1 && left < div_blocks.len() - 1 {
        // Go to the first empty block
        while is_file(&div_blocks[left]) {
            left += 1;
        }

        // Go to the first non-empty block
        while is_empty(&div_blocks[right]) || is_segmented(&div_blocks[right]) {
            right -= 1;
        }

        // Swap
        if div_blocks[right].len() <= div_blocks[left].len() {
            // Regular files
            if is_empty(&div_blocks[left]) {
                let last = &div_blocks[right].clone();
                div_blocks[left][0..last.len()].copy_from_slice(&last);
            } else {
                // Segmented files
                // if is_segmented(&div_blocks[left])
                {
                    let last = &div_blocks[right].clone();
                    let segment_start = div_blocks[left].iter().position(|&v| v == -1).unwrap();
                    div_blocks[left][segment_start..].copy_from_slice(&last);
                }
            }

            div_blocks[right].fill(-1);
            left += 1;
            right -= 1;
        } else {
            right -= 1;
        }

        if left > right {
            left = 0;
        }
    }

    div_blocks.iter().flatten().cloned().collect::<Vec<_>>()
}

fn checksum2(aligned_blocks: &Vec<i32>) -> i64 {
    let mut map: HashMap<i32, (usize, usize)> = HashMap::new();
    let unique_file_ids: HashSet<_> = HashSet::from_iter(aligned_blocks.iter().cloned());
    for file_id in unique_file_ids {
        let start = aligned_blocks.iter().position(|&v| v == file_id).unwrap();
        let end = aligned_blocks.iter().rposition(|&v| v == file_id).unwrap() + 1;
        map.insert(file_id, (start, end));
    }

    // remove the -1s from the map
    map.remove(&-1);

    let mut result: i64 = 0;
    for (&id, &(start, end)) in &map {
        for x in start..end {
            let x: i64 = x.try_into().unwrap();
            let id: i64 = id.try_into().unwrap();
            let product: i64 = x.checked_mul(id).unwrap();
            result = result.checked_add(product.into()).unwrap();
            println!("{} * {} = {}", id, x, product);
        }
    }

    result
}

pub fn part_two(input: &str) -> Option<i64> {
    let blocks = disk_map_to_mem_blocks(input);
    let aligned_blocks: Vec<i32> = align_full_file_blocks(blocks.clone());

    // println!("{:?}", blocks);
    // println!("{:?}", aligned_blocks);

    Some(checksum2(&aligned_blocks))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
