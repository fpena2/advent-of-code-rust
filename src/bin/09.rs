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

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
