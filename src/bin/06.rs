use std::collections::HashSet;

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

// TODO: should I modify the map or return a new one?
fn find_guard_route(map: &mut Vec<Vec<char>>, guard_start: (usize, usize)) {
    let (mut x, mut y) = guard_start;
    let mut dir: (i32, i32) = (0, -1); // start going up

    loop {
        // Check bounds before accessing map
        if y >= map.len() || x >= map[0].len() {
            break;
        }

        if map[y][x] == '^' || map[y][x] == '.' || map[y][x] == 'x' {
            map[y][x] = 'x'; // mark as visited

            // Walk
            // println!("C-Point: ({},{})", x, y);
            let new_x = (x as i32 + dir.0) as usize;
            let new_y = (y as i32 + dir.1) as usize;

            // Check bounds before moving
            if new_y >= map.len() || new_x >= map[0].len() {
                break;
            }

            // println!("N-Point: ({},{}) - {}", x, y, map[y][x]);
            x = new_x;
            y = new_y;

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
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut map, guard_start) = input_to_matrix(input);
    find_guard_route(&mut map, guard_start);
    Some(
        map.iter()
            .flat_map(|c| c.iter())
            .filter(|&c| *c == 'x')
            .count() as u32,
    )
}

fn find_x_positions(map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();
    for (y, row) in map.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == 'x' {
                positions.push((x, y));
            }
        }
    }
    positions
}

fn will_guard_route_loop(map: &mut Vec<Vec<char>>, guard_start: (usize, usize)) -> bool {
    let (mut x, mut y) = guard_start;
    let mut dir: (i32, i32) = (0, -1); // start going up

    let mut visited_positions: HashSet<((usize, usize), (i32, i32))> = HashSet::new(); // store the location and direction

    loop {
        if y >= map.len() || x >= map[0].len() {
            break;
        }

        if map[y][x] == '^' || map[y][x] == '.' || map[y][x] == 'x' {
            map[y][x] = 'x'; // mark as visited

            let new_x = (x as i32 + dir.0) as usize;
            let new_y = (y as i32 + dir.1) as usize;
            if new_y >= map.len() || new_x >= map[0].len() {
                break;
            }

            x = new_x;
            y = new_y;
            if map[y][x] == '#' {
                // Undo this move
                x = (x as i32 - dir.0) as usize;
                y = (y as i32 - dir.1) as usize;

                // Turn 90 degrees
                match dir {
                    (0, -1) => dir = (1, 0),  // turn right
                    (1, 0) => dir = (0, 1),   // turn down
                    (0, 1) => dir = (-1, 0),  // turn left
                    (-1, 0) => dir = (0, -1), // turn up
                    _ => unreachable!(),
                }
            }

            // check if we have visited this position and could be looping
            if visited_positions.contains(&((x, y), dir)) {
                return true;
            }

            // save step
            visited_positions.insert(((x, y), dir));
        }
    }

    false
}

pub fn part_two(input: &str) -> Option<u32> {
    use rayon::prelude::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    let (mut map, guard_start) = input_to_matrix(input);
    let clean_map = map.clone();
    println!("Guard start: ({},{})", guard_start.0, guard_start.1);

    // number of iterations
    find_guard_route(&mut map, guard_start);
    let iterations: Vec<(usize, usize)> = find_x_positions(&map)
        .iter()
        .filter(|&&p| p != guard_start)
        .copied()
        .collect();
    println!("Number of iterations: {}", iterations.len());

    // brute force
    let loop_counter = AtomicU32::new(0);
    iterations.par_iter().for_each(|point| {
        let mut map = clean_map.clone();
        map[point.1][point.0] = '#'; // NOTE: (x, y) indexing on map

        if will_guard_route_loop(&mut map, guard_start) {
            loop_counter.fetch_add(1, Ordering::SeqCst);
        }
    });

    Some(loop_counter.load(Ordering::SeqCst))
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
        assert_eq!(result, Some(6));
    }
}
