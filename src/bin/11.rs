advent_of_code::solution!(11);

fn process_tones(input: String) -> String {
    let mut tones = vec![];
    for value in input.split(" ") {
        if value == "0" {
            tones.push(String::from("1"));
        } else if value.len() % 2 == 0 {
            let len = value.len();
            tones.push(String::from(&value[..len / 2]));
            // Remove leading zeros
            let val: u64 = value[len / 2..].parse().unwrap();
            tones.push(String::from(val.to_string()));
        } else {
            let val: u64 = value.parse().unwrap();
            let val = (val * 2024).to_string();
            tones.push(val);
        }
    }
    tones.join(" ")
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut line = String::from(input);
    for _ in 0..25 {
        line = process_tones(line);
    }

    Some(line.split(" ").count() as u32)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut line = String::from(input);
    for index in 0..75 {
        // line = process_tones_multithreaded(line);
        // println!("{}", index);
    }

    Some(line.split(" ").count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }
}
