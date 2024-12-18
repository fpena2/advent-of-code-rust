use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(18);
const GRID_SIZE: usize = 70;
const NUMB_OF_BYTES: usize = 1024;

fn generate_grid() -> Vec<Vec<char>> {
    vec![vec!['.'; GRID_SIZE + 1]; GRID_SIZE + 1]
}

fn process_input(input: &str) -> Vec<(usize, usize)> {
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
    let mut grid = generate_grid();
    for (x, y) in process_input(input).iter().take(NUMB_OF_BYTES) {
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
                println!("{}", dist + 1);
                return Some(dist + 1);
            }

            seen.insert(next_step);
            queue.push_back((next_step.0, next_step.1, dist + 1));
        }
    }

    None
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
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
