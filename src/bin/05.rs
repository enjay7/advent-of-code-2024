use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let (rules, updates) = parse_input(input);

    // check each update, sum the middle page of those that are correctly ordered
    let total_middle_sum: u64 = updates.iter()
        .filter(|update| is_correctly_ordered(update, &rules)) // keep ony correctly ordered update
        .map(|update| update[update.len() / 2] as u64) // Get the middle page
        .sum();
    Some(total_middle_sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (rules, updates) = parse_input(input);

    // Check each update, for each of the incorrectly ordered update, reorder based on the rules
    let total_middle_sum: u64 = updates.iter()
        .filter(|update| !is_correctly_ordered(update, &rules))
        .map(|update| {
            let sorted_update = re_order_update(update, &rules); // Reorder the update
            sorted_update[sorted_update.len() / 2] as u64
        })
        .sum();
    Some(total_middle_sum)
}

/// parses the input into ordering rules and updates
fn parse_input(input: &str) -> (HashSet<(u32, u32)>, Vec<Vec<u32>>) {
    let mut rules = HashSet::new();
    let mut updates = Vec::new();

    // Split the input into sections
    let mut lines = input.lines();
    for line in &mut lines {
        if line.is_empty(){
            break;
        }
        // parse rules of the form "x|y" into tuple (x, y)
        let (x, y) = line.split_once('|').unwrap();
        rules.insert((x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()));
    }

    // parse updates as comma seperated page numbers
    for line in lines {
        let update = line
            .split(',')
            .map(|n| n.parse::<u32>().unwrap())
            .collect();
        updates.push(update);
    }

    (rules, updates)
}

fn is_correctly_ordered(update: &[u32], rules: &HashSet<(u32, u32)>) -> bool {
    // use a helper to map each page to its position in the update
    let mut positions = HashMap::new();
    for (i, &page) in update.iter().enumerate() {
        positions.insert(page, i);
    }

    // Check all rules to ensure they are respected within the update
    for &(x, y) in rules {
        if let(Some(&pos_x), Some(&pos_y)) = (positions.get(&x), positions.get(&y)){
            if pos_x > pos_y {
                return false; // rule violated
            }
        }
    }

    true
}

fn re_order_update(update: &[u32], rules: &HashSet<(u32, u32)>) -> Vec<u32> {
    let mut sorted_update = update.to_vec();

    // sort the update using the custom ordering rules
    sorted_update.sort_by(|&x, &y| {
        if rules.contains(&(x, y)) {
            std::cmp::Ordering::Less // x<y
        } else if rules.contains(&(y,x)) {
            std::cmp::Ordering::Greater // x>y
        } else {
            std::cmp::Ordering::Equal // No rule, keep the original relative order
        }
    });
    sorted_update
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
