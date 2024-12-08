advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    use regex::Regex;
    let re = Regex::new(r"mul[(](?<a>\d{1,3}),(?<b>\d{1,3})[)]").unwrap();
    let res = re
        .captures_iter(input)
        .filter_map(|caps| {
            let a: u32 = caps.name("a")?.as_str().parse().ok()?;
            let b: u32 = caps.name("b")?.as_str().parse().ok()?;
            Some(a * b)
        })
        .sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    use regex::Regex;
    let re = Regex::new(r"(mul[(]\d{1,3},\d{1,3}[)]|do[(][)]|don't[(][)])").unwrap();
    let matches: Vec<_> = re.find_iter(input).map(|m| m.as_str()).collect();

    let mut ans = String::new();
    let mut skip_enabled = false;
    for mul in matches {
        match mul {
            "do()" => skip_enabled = false,
            "don't()" => skip_enabled = true,
            _ => {
                if skip_enabled == true {
                    continue;
                }
                ans.push_str(mul);
            }
        }
    }
    part_one(ans.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(result, Some(48));
    }
}
