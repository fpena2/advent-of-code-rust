advent_of_code::solution!(15);

fn parse(input: &str) -> (&str, &str) {
    let mut block_iter = input.split("\n\n");
    let wearhouse = block_iter.next().unwrap();
    let robot_moves = block_iter.next().unwrap();
    (wearhouse, robot_moves)
}

fn wearhouse_grid(block: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for (_y, row) in block.lines().enumerate() {
        let col: Vec<char> = row.chars().collect();
        grid.push(col);
    }
    grid
}

fn moves_list(block: &str) -> Vec<char> {
    block.lines().flat_map(|s| s.chars()).collect()
}

fn robot_location(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == '@' {
                return (i, j);
            }
        }
    }
    unreachable!()
}

fn cal_next_point(point: &(usize, usize), c: &char) -> (usize, usize) {
    let mut point = *point;
    if *c == '^' {
        point.0 -= 1;
    } else if *c == 'v' {
        point.0 += 1;
    } else if *c == '<' {
        point.1 -= 1;
    } else if *c == '>' {
        point.1 += 1;
    }
    point
}

fn cal_gps_coordinates(grid: &Vec<Vec<char>>) -> usize {
    let mut results = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 'O' {
                results += 100 * i + j;
            }
        }
    }
    results
}

fn simulate(moves: &Vec<char>, grid: &Vec<Vec<char>>) -> usize {
    let mut grid = grid.clone();
    let mut init_point = robot_location(&grid);

    for m in moves {
        let mut to_move = vec![init_point];
        let mut curr_point = init_point;
        let mut stop = false;

        loop {
            curr_point = cal_next_point(&curr_point, m);
            let value = grid[curr_point.0][curr_point.1];
            if value == '#' {
                stop = true;
                break;
            }
            if value == 'O' {
                to_move.push(curr_point);
            }
            if value == '.' {
                break;
            }
        }

        if stop {
            continue;
        }

        grid[init_point.0][init_point.1] = '.';

        let next_point = cal_next_point(&init_point, m);
        grid[next_point.0][next_point.1] = '@';

        for point in to_move.iter().skip(1) {
            let next_point = cal_next_point(point, m);
            grid[next_point.0][next_point.1] = 'O';
        }

        init_point = cal_next_point(&init_point, m);

        // println!("{}", m);
        // for row in grid.iter() {
        //     println!("{}", row.iter().collect::<String>());
        // }
    }

    cal_gps_coordinates(&grid)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (wearhouse, robot_moves) = parse(input);
    let grid = wearhouse_grid(wearhouse);
    let moves = moves_list(robot_moves);
    let results = simulate(&moves, &grid);
    Some(results)
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
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
