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

fn is_update_valid(update: &[u32], rules: &Vec<HashMap<u32, u32>>) -> bool {
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

fn process_input(input: &str) -> (Vec<HashMap<u32, u32>>, Vec<Vec<u32>>) {
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
    (rules, updates)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, updates) = process_input(input);

    let mut valid_updates = vec![];
    for update in updates {
        if is_update_valid(&update, &rules) {
            valid_updates.push(update.clone());
        }
    }

    Some(valid_updates.iter().map(|s| s[s.len() / 2]).sum())
}

fn fix_invalid_update(update: &[u32], rules: &Vec<HashMap<u32, u32>>) -> Vec<u32> {
    let mut res = vec![];
    for &page in update.iter() {
        // println!("Coming in page: {}", page);
        if let Some(last_page) = res.last() {
            // println!("Last page in list: {}", last_page);
            let last_page_rules = get_page_rules(*last_page, rules);
            if last_page_rules.contains(&page) {
                // Rule allows this page to be inserted after the last page
                res.push(page);
            } else {
                for (index, r) in res.clone().iter().enumerate() {
                    if !get_page_rules(*r, rules).contains(&page) {
                        // println!("Inserting before page: {} ", r);
                        res.insert(index, page);
                        break;
                    }
                }
            }
        } else {
            // First page
            res.push(page);
        }
    }

    res
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, updates) = process_input(input);
    let mut invalid_updates = vec![];
    for update in updates {
        if !is_update_valid(&update, &rules) {
            invalid_updates.push(update.clone());
        }
    }

    Some(
        invalid_updates
            .iter()
            .map(|s| fix_invalid_update(s, &rules)[s.len() / 2])
            .sum(),
    )
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
        assert_eq!(result, Some(123));
    }
}
