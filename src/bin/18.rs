use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(18);

fn generate_grid(grid_size: usize) -> Vec<Vec<char>> {
    vec![vec!['.'; grid_size + 1]; grid_size + 1]
}

fn get_bytes_coords(input: &str) -> Vec<(usize, usize)> {
    let mut points: Vec<(usize, usize)> = Vec::new();
    for line in input.lines() {
        let mut point = line.split(",");
        let x = point.next().unwrap().parse().unwrap();
        let y = point.next().unwrap().parse().unwrap();
        points.push((x, y));
    }
    points
}

pub fn part_one(input: &str) -> Option<usize> {
    const GRID_SIZE: usize = 6; // 70
    const NUMB_OF_BYTES: usize = 12; // 1024

    let mut grid = generate_grid(GRID_SIZE);
    for (x, y) in get_bytes_coords(input).iter().take(NUMB_OF_BYTES) {
        grid[*x][*y] = '#';
    }

    let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::from([(0, 0, 0)]);
    let mut seen: HashSet<(usize, usize)> = HashSet::from([(0, 0)]);
    while queue.len() > 0 {
        let (x, y, dist) = queue.pop_front().unwrap();
        for next_step in [
            (x + 1, y),
            (x.wrapping_sub(1), y),
            (x, y + 1),
            (x, y.wrapping_sub(1)),
        ] {
            if next_step.0 > GRID_SIZE || next_step.1 > GRID_SIZE {
                continue;
            }

            if grid[next_step.0][next_step.1] == '#' {
                continue;
            }

            if seen.contains(&next_step) {
                continue;
            }

            if next_step.0 == next_step.1 && next_step.0 == GRID_SIZE {
                return Some(dist + 1);
            }

            seen.insert(next_step);
            queue.push_back((next_step.0, next_step.1, dist + 1));
        }
    }

    None
}

fn connnected(
    grid: &Vec<Vec<char>>,
    falling_bytes: &Vec<(usize, usize)>,
    numb_of_bytes: usize,
) -> bool {
    let grid_size = grid.len() - 1;
    let mut grid = grid.clone();
    for (x, y) in falling_bytes.iter().take(numb_of_bytes) {
        grid[*x][*y] = '#';
    }

    let mut queue: VecDeque<(usize, usize)> = VecDeque::from([(0, 0)]);
    let mut seen: HashSet<(usize, usize)> = HashSet::from([(0, 0)]);
    while queue.len() > 0 {
        let (x, y) = queue.pop_front().unwrap();
        for next_step in [
            (x + 1, y),
            (x.wrapping_sub(1), y),
            (x, y + 1),
            (x, y.wrapping_sub(1)),
        ] {
            if next_step.0 > grid_size || next_step.1 > grid_size {
                continue;
            }

            if grid[next_step.0][next_step.1] == '#' {
                continue;
            }

            if seen.contains(&next_step) {
                continue;
            }

            if next_step.0 == next_step.1 && next_step.0 == grid_size {
                return true;
            }

            seen.insert(next_step);
            queue.push_back((next_step.0, next_step.1));
        }
    }

    false
}

pub fn part_two(input: &str) -> Option<String> {
    const GRID_SIZE: usize = 6; // 70;
    let grid = generate_grid(GRID_SIZE);
    let falling_bytes = get_bytes_coords(input);

    let mut l = 0;
    let mut r = falling_bytes.len() - 1;
    while l < r {
        let m = (l + r) / 2;
        if connnected(&grid, &falling_bytes, m + 1) {
            l = m + 1;
        } else {
            r = m;
        }
    }
    Some(format!("{},{}", falling_bytes[l].0, falling_bytes[l].1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6,1".to_string()));
    }
}
