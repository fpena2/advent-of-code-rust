advent_of_code::solution!(6);

fn input_to_matrix(input: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut arrow_position = (0usize, 0usize);

    for (y, row) in input.lines().enumerate() {
        let col: Vec<char> = row.chars().collect();
        if let Some(arrow_x) = col.iter().position(|&c| c == '^') {
            arrow_position = (arrow_x, y); // Set as (x, y)
        }
        grid.push(col);
    }
    (grid, arrow_position)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (matrix, arrow) = input_to_matrix(input);

    // println!("Arrow: ({}, {})", arrow.0, arrow.1);

    let (mut x, mut y) = arrow;
    let mut dir: (i32, i32) = (0, -1); // start going up
    let mut map = matrix.clone();

    loop {
        // Check bounds before accessing map
        if y >= map.len() || x >= map[0].len() {
            break;
        }

        if map[y][x] == '^' || map[y][x] == '.' || map[y][x] == 'x' {
            // mark as visited
            map[y][x] = 'x';
            // println!("C-Point: ({},{})", x, y);

            // Walk
            let new_x = (x as i32 + dir.0) as usize;
            let new_y = (y as i32 + dir.1) as usize;

            // Check bounds before moving
            if new_y >= map.len() || new_x >= map[0].len() {
                break;
            }

            x = new_x;
            y = new_y;
            // println!("N-Point: ({},{}) - {}", x, y, map[y][x]);

            if map[y][x] == '#' {
                // println!("Hit a wall: ({},{})", x, y);

                // Undo this move
                x = (x as i32 - dir.0) as usize;
                y = (y as i32 - dir.1) as usize;
                // println!("U-Point: ({},{})", x, y);

                // Turn 90 degrees
                match dir {
                    (0, -1) => dir = (1, 0),  // turn right
                    (1, 0) => dir = (0, 1),   // turn down
                    (0, 1) => dir = (-1, 0),  // turn left
                    (-1, 0) => dir = (0, -1), // turn up
                    _ => unreachable!(),
                }
            }
        }
    }

    // pretty print the map
    // for row in map {
    //     println!("{}", row.iter().collect::<String>());
    // }

    Some(
        map.iter()
            .flat_map(|c| c.iter())
            .filter(|&c| *c == 'x')
            .count() as u32,
    )
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
