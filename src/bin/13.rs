use regex::Regex;

advent_of_code::solution!(13);

#[derive(Debug)]
struct Prize(u64, u64);

#[derive(Debug)]
struct Button {
    x: u64,
    y: u64,
}
impl Button {
    fn new(x: u64, y: u64) -> Button {
        Button { x, y }
    }
}

#[derive(Debug)]
struct Buttons {
    a: Button,
    b: Button,
}
impl Buttons {
    fn new(a: Button, b: Button) -> Buttons {
        Buttons { a, b }
    }
}

#[derive(Debug)]
struct Machine {
    buttons: Buttons,
    prize: Prize,
}
impl Machine {
    fn new(b: Buttons, p: Prize) -> Machine {
        Machine {
            buttons: b,
            prize: p,
        }
    }
}

fn parse(input: &str) -> Vec<Machine> {
    let mut machines = Vec::new();
    let re = Regex::new(r"Button A: X\+(?P<a_x>\d+), Y\+(?P<a_y>\d+)\s+Button B: X\+(?P<b_x>\d+), Y\+(?P<b_y>\d+)\s+Prize: X=(?P<p_x>\d+), Y=(?P<p_y>\d+)").unwrap();
    for block in input.split("\n\n") {
        for cap in re.captures_iter(block) {
            let x = cap["a_x"].parse().unwrap();
            let y = cap["a_y"].parse().unwrap();
            let button_a = Button::new(x, y);

            let x = cap["b_x"].parse().unwrap();
            let y = cap["b_y"].parse().unwrap();
            let button_b = Button::new(x, y);

            let x = cap["p_x"].parse().unwrap();
            let y = cap["p_y"].parse().unwrap();
            let prize = Prize(x, y);
            machines.push(Machine::new(Buttons::new(button_a, button_b), prize));
        }
    }
    machines
}

fn calculate_min_score(machine: Machine) -> Option<u64> {
    let a_button = machine.buttons.a;
    let b_button = machine.buttons.b;

    let mut min_score = u64::MAX;
    for a in 0..100 {
        for b in 0..100 {
            // NOTE: two equations and two unknowns
            if a_button.x * a + b_button.x * b == machine.prize.0
                && a_button.y * a + b_button.y * b == machine.prize.1
            {
                min_score = min_score.min(a * 3 + b);
            }
        }
    }

    if min_score == u64::MAX {
        return None;
    }

    Some(min_score)
}
pub fn part_one(input: &str) -> Option<u64> {
    let machines = parse(input);
    let mut total = 0;
    for machine in machines {
        if let Some(score) = calculate_min_score(machine) {
            total += score;
        }
    }
    Some(total)
}

fn calculate_min_score_algebra(machine: Machine) -> Option<f64> {
    let offset: u64 = 10_000_000_000_000;
    let a_button = machine.buttons.a;
    let b_button = machine.buttons.b;
    let price_x = (machine.prize.0 + offset) as f64;
    let price_y = (machine.prize.1 + offset) as f64;
    let a_presses = (price_x * b_button.y as f64 - price_y * b_button.x as f64)
        / (a_button.x as f64 * b_button.y as f64 - a_button.y as f64 * b_button.x as f64);
    let b_presses = (price_x - a_button.x as f64 * a_presses) / b_button.x as f64;

    // Make sure the presses are integers
    if a_presses % 1.0 == 0.0 && b_presses % 1.0 == 0.0 {
        return Some(a_presses * 3.0 + b_presses);
    }

    None
}
pub fn part_two(input: &str) -> Option<u64> {
    let machines = parse(input);
    let mut total = 0;
    for machine in machines {
        if let Some(score) = calculate_min_score_algebra(machine) {
            total += score as u64;
        }
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
