advent_of_code::solution!(10);

use std::collections::{HashMap, HashSet, VecDeque};

fn input_to_matrix(input: &str) -> Vec<Vec<usize>> {
    let mut grid: Vec<Vec<usize>> = Vec::new();
    for (_y, row) in input.lines().enumerate() {
        let col: Vec<usize> = row
            .chars()
            .map(|v| v.to_digit(10).unwrap() as usize)
            .collect();
        grid.push(col); // col as ( y, x )
    }
    grid
}

fn find_trailheads(grid: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    let mut trailheads: Vec<(usize, usize)> = Vec::new();
    let rows = grid.len();
    let cols = grid[0].len();
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == 0 {
                trailheads.push((i, j));
            }
        }
    }
    trailheads
}

fn neighbors(point: &(usize, usize), grid: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = Vec::new();
    if point.0 < grid.len() - 1 {
        neighbors.push((point.0 + 1, point.1));
    }
    if point.0 > 0 {
        neighbors.push((point.0 - 1, point.1));
    }
    if point.1 < grid[0].len() - 1 {
        neighbors.push((point.0, point.1 + 1));
    }
    if point.1 > 0 {
        neighbors.push((point.0, point.1 - 1));
    }
    neighbors
}

fn score_trailhead(trailhead: &(usize, usize), grid: &Vec<Vec<usize>>) -> u32 {
    let mut deque: VecDeque<(usize, usize)> = VecDeque::new();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();

    deque.push_back(*trailhead);
    seen.insert(*trailhead);

    let mut trailends = 0;

    while deque.len() > 0 {
        let current = deque.pop_front().unwrap();

        // look at all the neighbors
        for neighbor in neighbors(&current, &grid) {
            // if the neighbor is out of bounds, continue
            if neighbor.0 >= grid.len() || neighbor.1 >= grid[0].len() {
                continue;
            }

            // if the next position is too far, continue
            if grid[neighbor.0][neighbor.1] != grid[current.0][current.1] + 1 {
                continue;
            }

            // if the neighbor has already been checked, continue
            if seen.contains(&neighbor) {
                continue;
            } else {
                seen.insert(neighbor);
            }

            if grid[neighbor.0][neighbor.1] == 9 {
                trailends += 1;
            } else {
                deque.push_back(neighbor);
            }

            // println!(
            //     "{:?}: {} -> {:?} : {}",
            //     current, grid[current.0][current.1], neighbor, grid[neighbor.0][neighbor.1]
            // );
        }
    }

    trailends
}
pub fn part_one(input: &str) -> Option<u32> {
    let grid = input_to_matrix(input);

    let trailheads = find_trailheads(&grid);

    // println!("{:?}", trailheads);
    let mut result = 0;
    for trailhead in trailheads {
        let score = score_trailhead(&trailhead, &grid);
        result += score;
        // println!("Trailhead: {:?} Score: {}", trailhead, score);
        // break;
    }

    Some(result)
}

fn score_unique_trails_to_trailend(trailhead: &(usize, usize), grid: &Vec<Vec<usize>>) -> usize {
    let mut deque: VecDeque<(usize, usize)> = VecDeque::new();
    let mut seen: HashMap<(usize, usize), usize> = HashMap::new();

    deque.push_back(*trailhead);
    seen.insert(*trailhead, 1);

    let mut trails = 0usize;

    while deque.len() > 0 {
        let current = deque.pop_front().unwrap();
        if grid[current.0][current.1] == 9 {
            trails += *seen.get(&current).unwrap();
        }

        // look at all the neighbors
        for neighbor in neighbors(&current, &grid) {
            // if the neighbor is out of bounds, continue
            if neighbor.0 >= grid.len() || neighbor.1 >= grid[0].len() {
                continue;
            }

            // if the next position is too far, continue
            if grid[neighbor.0][neighbor.1] != grid[current.0][current.1] + 1 {
                continue;
            }

            // if the neighbor has already been checked, continue
            if seen.contains_key(&neighbor) {
                *seen.get_mut(&neighbor).unwrap() += *seen.get(&current).unwrap();
                continue;
            }
            seen.insert(neighbor, *seen.get(&current).unwrap());
            deque.push_back(neighbor);

            // println!(
            //     "{:?}: {} -> {:?} : {}",
            //     current, grid[current.0][current.1], neighbor, grid[neighbor.0][neighbor.1]
            // );
        }
    }

    trails
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = input_to_matrix(input);

    let trailheads = find_trailheads(&grid);

    // println!("{:?}", trailheads);
    let mut result = 0;
    for trailhead in trailheads {
        let score = score_unique_trails_to_trailend(&trailhead, &grid);
        result += score;
        // println!("Trailhead: {:?} Score: {}", trailhead, score);
        // break;
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
