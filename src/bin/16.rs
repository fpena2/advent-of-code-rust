use std::{char, collections::HashSet};

advent_of_code::solution!(16);

fn grid(block: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for (_y, row) in block.lines().enumerate() {
        let col: Vec<char> = row.chars().collect();
        grid.push(col);
    }
    grid
}

fn get_location(grid: &Vec<Vec<char>>, marker: char) -> (isize, isize) {
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == marker {
                return (i as isize, j as isize);
            }
        }
    }
    unreachable!()
}

fn dijkstra(grid: &Vec<Vec<char>>) -> u32 {
    use std::collections::BinaryHeap;

    let start = get_location(grid, 'S');
    // let end = get_location(grid, 'E');

    let diff = (0isize, 1isize);
    // NOTE: store negative cost to make this into a min-heap
    // Rust has a max-heap implementation
    let mut heap = BinaryHeap::from([(0i32, start, diff)]);
    let mut seen = HashSet::from([(start, diff)]);

    while heap.len() > 0 {
        let (cost, point, diff) = heap.pop().unwrap();

        // println!("Cost: {} Point: {:?} Diff: {:?}", cost, point, diff);

        seen.insert((point, diff));

        if grid[point.0 as usize][point.1 as usize] == 'E' {
            return cost.abs() as u32;
        }

        let a = (cost - 1, (point.0 + diff.0, point.1 + diff.1), diff);
        let b = (cost - 1000, point, (diff.1, -diff.0));
        let c = (cost - 1000, point, (-diff.1, diff.0));
        for n in [a, b, c] {
            if grid[n.1 .0 as usize][n.1 .1 as usize] == '#' {
                continue;
            }
            if seen.contains(&(n.1, n.2)) {
                continue;
            }
            heap.push(n);
        }
    }

    0
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = grid(input);
    let result = dijkstra(&map);
    Some(result)
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
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
