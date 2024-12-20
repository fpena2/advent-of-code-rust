use regex::Regex;

advent_of_code::solution!(14);

#[derive(Debug)]
struct Robot {
    x: usize,
    y: usize,
    vx: i32,
    vy: i32,
    world: (usize, usize),
}

impl Robot {
    fn simulate(&self, seconds: u32) -> (usize, usize) {
        let sim_x = (self.x as i32 + self.vx * seconds as i32).rem_euclid(self.world.0 as i32);
        let sim_y = (self.y as i32 + self.vy * seconds as i32).rem_euclid(self.world.1 as i32);
        (sim_x as usize, sim_y as usize)
    }
}

fn parse(input: &str, world: (usize, usize)) -> Vec<Robot> {
    let re = Regex::new(r"p=([\d-]+),([\d-]+) v=([\d-]+),([\d-]+)").unwrap();
    let mut robots: Vec<Robot> = Vec::new();
    for line in input.lines() {
        for capture in re.captures_iter(line) {
            let x = capture[1].parse().unwrap();
            let y = capture[2].parse().unwrap();
            let vx = capture[3].parse().unwrap();
            let vy = capture[4].parse().unwrap();
            robots.push(Robot {
                x,
                y,
                vx,
                vy,
                world,
            });
        }
    }
    robots
}
pub fn part_one(input: &str) -> Option<u32> {
    let world: (usize, usize) = (11, 7); // 101 x 103 for real problem
    let horizontal_median = (world.0 - 1) / 2;
    let vertical_median = (world.1 - 1) / 2;

    let robots_in_grid = parse(input, world)
        .iter()
        .map(|r| r.simulate(100))
        .collect::<Vec<_>>();

    let mut quadrants = [0; 4];
    for (x, y) in robots_in_grid {
        if x == horizontal_median || y == vertical_median {
            continue;
        }
        if x < horizontal_median {
            if y < vertical_median {
                quadrants[0] += 1; // top left
            } else {
                quadrants[1] += 1;
            }
        } else {
            if y < vertical_median {
                quadrants[2] += 1;
            } else {
                quadrants[3] += 1; // bottom right
            }
        }
    }
    Some(quadrants.iter().product())
}

pub fn part_two(input: &str) -> Option<usize> {
    let world: (usize, usize) = (101, 103);
    let horizontal_median = (world.0 - 1) / 2;
    let vertical_median = (world.1 - 1) / 2;

    let robots = parse(input, world);

    let mut min_safety_factor = i32::MAX;
    let mut best_iteration = None;

    for sec in 0..world.0 * world.1 {
        let mut robots_in_grid = vec![];
        for robot in &robots {
            robots_in_grid.push(robot.simulate(sec as u32));
        }

        let mut quadrants = [0; 4];
        for (x, y) in robots_in_grid {
            if x == horizontal_median || y == vertical_median {
                continue;
            }
            if x < horizontal_median {
                if y < vertical_median {
                    quadrants[0] += 1; // top left
                } else {
                    quadrants[1] += 1;
                }
            } else {
                if y < vertical_median {
                    quadrants[2] += 1;
                } else {
                    quadrants[3] += 1; // bottom right
                }
            }
        }

        let curr_safety_factor: i32 = quadrants.iter().product();
        if curr_safety_factor < min_safety_factor {
            min_safety_factor = curr_safety_factor;
            best_iteration = Some(sec);
        }
    }
    best_iteration
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }
}
