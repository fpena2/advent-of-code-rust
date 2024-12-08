advent_of_code::solution!(8);

use std::collections::{HashMap, HashSet};

// The signal only applies its nefarious effect at specific antinodes
// based on the resonant frequencies of the antennas. In particular,
// an antinode occurs at any point that is perfectly in line with
// two antennas of the same frequency - but only when one of the
// antennas is twice as far away as the other. This means that for
// any pair of antennas with the same frequency, there are two
// antinodes, one on either side of them.

fn input_to_matrix(input: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for (_y, row) in input.lines().enumerate() {
        let col: Vec<char> = row.chars().collect();
        grid.push(col);
    }
    grid
}

fn find_antennas(grid: &Vec<Vec<char>>) -> HashMap<(usize, usize), char> {
    let mut antennas: HashMap<(usize, usize), char> = HashMap::new();
    for (i_row, row) in grid.iter().enumerate() {
        for (i_col, c) in row.iter().enumerate() {
            if c.is_alphabetic() || c.is_digit(10) {
                antennas.insert((i_col, i_row), *c);
            }
        }
    }
    antennas
}

fn find_antinodes(antennas: &HashMap<(usize, usize), char>) -> HashSet<(usize, usize)> {
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    for (antenna_position, antenna_type) in antennas.iter() {
        // println!("antenna: ({},{})", antenna_position.0, antenna_position.1);
        let other_antennas: Vec<(&(usize, usize), &char)> = antennas
            .iter()
            .filter(|a| a.0 != antenna_position && a.1 == antenna_type)
            .collect::<Vec<(&(usize, usize), &char)>>();

        for (other_antenna_position, _) in other_antennas {
            let x_diff = antenna_position.0 as isize - other_antenna_position.0 as isize;
            let y_diff = antenna_position.1 as isize - other_antenna_position.1 as isize;

            let antinode = (
                antenna_position.0 as isize + x_diff,
                antenna_position.1 as isize + y_diff,
            );
            antinodes.insert((antinode.0 as usize, antinode.1 as usize));
        }
    }
    antinodes
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input_to_matrix(input);
    let antennas = find_antennas(&grid);
    let antinodes = find_antinodes(&antennas)
        .into_iter()
        .filter(|n| n.0 < grid[0].len() && n.1 < grid.len())
        .collect::<HashSet<(usize, usize)>>();
    Some(antinodes.len() as u32)
}

fn find_harmonic_antinodes(antennas: &HashMap<(usize, usize), char>) -> HashSet<(usize, usize)> {
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    for (antenna_position, antenna_type) in antennas.iter() {
        // println!("antenna: ({},{})", antenna_position.0, antenna_position.1);
        let other_antennas: Vec<(&(usize, usize), &char)> = antennas
            .iter()
            .filter(|a| a.0 != antenna_position && a.1 == antenna_type)
            .collect::<Vec<(&(usize, usize), &char)>>();

        for (other_antenna_position, _) in other_antennas {
            let x_diff = antenna_position.0 as isize - other_antenna_position.0 as isize;
            let y_diff = antenna_position.1 as isize - other_antenna_position.1 as isize;

            let mut antinode;

            // FIXME:Hardcoded calculations of antinodes
            for i in 1..100 {
                antinode = (
                    antenna_position.0 as isize + x_diff * i,
                    antenna_position.1 as isize + y_diff * i,
                );
                antinodes.insert((antinode.0 as usize, antinode.1 as usize));
            }
        }

        // Antennas are also antinodes
        antinodes.insert(*antenna_position);
    }
    antinodes
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = input_to_matrix(input);
    let antennas = find_antennas(&grid);
    let antinodes = find_harmonic_antinodes(&antennas)
        .into_iter()
        .filter(|n| n.0 < grid[0].len() && n.1 < grid.len())
        .collect::<HashSet<(usize, usize)>>();
    // println!("Antinodes: {:?}", antinodes);
    Some(antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
