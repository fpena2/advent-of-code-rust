use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(12);

fn input_to_matrix(input: &str) -> Vec<Vec<char>> {
    let mut grid = Vec::new();
    for (_y, row) in input.lines().enumerate() {
        let col: Vec<char> = row.chars().collect();
        grid.push(col); // col as ( y, x )
    }
    grid
}

fn neighbors(point: &(usize, usize), grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
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

fn find_regions(grid: &Vec<Vec<char>>) -> Vec<HashSet<(usize, usize)>> {
    let mut regions: Vec<HashSet<(usize, usize)>> = Vec::new();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();

    let rows = grid.len();
    let cols = grid[0].len();
    for r in 0..rows {
        for c in 0..cols {
            if seen.contains(&(r, c)) {
                continue;
            }

            let mut region = HashSet::new();
            let mut deque: VecDeque<(usize, usize)> = VecDeque::new();

            region.insert((r, c));
            deque.push_back((r, c));

            let plant_type = grid[r][c];
            while deque.len() > 0 {
                let current = deque.pop_front().unwrap();
                for neighbor in neighbors(&current, &grid) {
                    // if the neighbor is out of bounds, continue
                    if neighbor.0 >= grid.len() || neighbor.1 >= grid[0].len() {
                        continue;
                    }

                    // if the neighbor plant is not the same type, continue
                    if grid[neighbor.0][neighbor.1] != plant_type {
                        continue;
                    }

                    // if the neighbor has already been added, continue
                    if region.contains(&neighbor) {
                        continue;
                    }

                    region.insert(neighbor);
                    deque.push_back(neighbor);
                }
            }

            regions.push(region.clone());
            seen.extend(region.iter());
        }
    }

    regions
}

fn calculate_perimeter(region: &HashSet<(usize, usize)>, grid: &Vec<Vec<char>>) -> u32 {
    let mut region_perimeter = 0u32;
    for plant in region.iter() {
        let mut plant_perimeter = 4u32;
        for neighbor in neighbors(&plant, &grid) {
            if region.contains(&neighbor) {
                plant_perimeter -= 1;
            }
        }
        region_perimeter += plant_perimeter;
    }
    region_perimeter
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input_to_matrix(input);
    let regions = find_regions(&grid);
    regions
        .iter()
        .map(|r| r.len() as u32 * calculate_perimeter(r, &grid))
        .sum::<u32>()
        .into()
}

fn sides(region: &[(f64, f64)]) -> usize {
    let mut corner_candidates = HashSet::new();

    for &(r, c) in region {
        let corners = [
            (r - 0.5, c - 0.5),
            (r + 0.5, c - 0.5),
            (r + 0.5, c + 0.5),
            (r - 0.5, c + 0.5),
        ];

        for &corner in &corners {
            // Scale and convert to integers to avoid precision issues
            let scaled_corner = ((corner.0 * 10.0) as i64, (corner.1 * 10.0) as i64);
            corner_candidates.insert(scaled_corner);
        }
    }

    let mut corners = 0;
    for &(cr, cc) in &corner_candidates {
        let config: Vec<bool> = [
            ((cr - 5, cc - 5), region),
            ((cr + 5, cc - 5), region),
            ((cr + 5, cc + 5), region),
            ((cr - 5, cc + 5), region),
        ]
        .iter()
        .map(|&(corner, region)| region.contains(&(corner.0 as f64 / 10.0, corner.1 as f64 / 10.0)))
        .collect();

        let number = config.iter().filter(|&&x| x).count();
        match number {
            1 => {
                corners += 1;
            }
            2 => {
                if config == vec![true, false, true, false]
                    || config == vec![false, true, false, true]
                {
                    corners += 2;
                }
            }
            3 => {
                corners += 1;
            }
            _ => {}
        }
    }

    corners
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = input_to_matrix(input);
    let regions = find_regions(&grid);
    regions
        .iter()
        .map(|r| {
            let region: Vec<(f64, f64)> = r.iter().map(|&(a, b)| (a as f64, b as f64)).collect();
            r.len() * sides(&region)
        })
        .sum::<usize>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(140));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(80));
    }
}
