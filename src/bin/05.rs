use std::{collections::HashMap, vec};

advent_of_code::solution!(5);

fn get_page_rules(page: u32, rules: &Vec<HashMap<u32, u32>>) -> Vec<u32> {
    let mut page_rules = vec![];
    for rule in rules {
        if let Some(a) = rule.get(&page) {
            page_rules.push(*a);
        }
    }
    page_rules
}

fn check_update(update: &[u32], rules: &Vec<HashMap<u32, u32>>) -> bool {
    let mut valid_pages = vec![];
    for (page_index, page) in update.iter().enumerate() {
        let page_rules = get_page_rules(*page, rules);

        // Check if this page's rules allow it to be placed before any other page
        let mut next_pages_check = vec![];
        let next_pages = &update[page_index + 1..];
        for next_page in next_pages.iter() {
            if page_rules.contains(next_page) {
                next_pages_check.push(*next_page);
            }
        }

        if next_pages_check.iter().eq(next_pages.iter()) {
            valid_pages.push(*page);
        }
    }

    if valid_pages.iter().eq(update.iter()) {
        return true;
    }

    false
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut rules = Vec::new();
    let mut updates = Vec::new();
    for line in input.lines() {
        if line.contains('|') {
            let parts: Vec<&str> = line.split('|').collect();
            assert_eq!(parts.len(), 2);
            let first: u32 = parts[0].parse().unwrap();
            let second: u32 = parts[1].parse().unwrap();
            let mut rule: HashMap<u32, u32> = HashMap::new();
            rule.insert(first, second);
            rules.push(rule);
        }
        if line.contains(',') {
            let update: Vec<u32> = line.split(',').map(|v| v.parse().unwrap()).collect();
            updates.push(update);
        }
    }

    let mut valid_updates = vec![];
    for update in updates {
        if check_update(&update, &rules) {
            valid_updates.push(update.clone());
        }
    }

    Some(valid_updates.iter().map(|s| s[s.len() / 2]).sum())
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
