use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    // parse the input into left and right lists

    let (mut left, mut right) = parse_input(input);

    left.sort_unstable();
    right.sort_unstable();

    // calculate the total distance by summing up abs diff
    let total_distance: u64 = left
        .iter()
        .zip(right.iter()) // combine the two lists element by element
        .map(|(l, r)| (l - r).abs() as u64)
        .sum();

    Some(total_distance) // Return the result wrapper in Some
}

pub fn part_two(input: &str) -> Option<u64> {
    let (left, right) = parse_input(input);

    // create a frequency map for the numbers in the right list
    let mut frequency_map = HashMap::new();
    for &num in &right {
        *frequency_map.entry(num).or_insert(0) += 1;
    }

    let similarity_score: u64 = left
        .iter()
        .map(|&num| num as u64 * *frequency_map.get(&num).unwrap_or(&0) as u64)
        .sum();

    Some(similarity_score)
}

pub fn parse_input(input: &str) -> (Vec<i64>, Vec<i64>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    // Normalize line endings and process each line
    for line in input.lines() {
        let line = line.trim(); // Remove extra whitespace (e.g., `\r`)
        let mut parts = line.split_whitespace(); // Split by spaces
        let l: i64 = parts.next().unwrap().parse().unwrap();
        let r: i64 = parts.next().unwrap().parse().unwrap();

        left.push(l);
        right.push(r);
    }

    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        // Verify that input is being parsed correctly
        // let input = advent_of_code::template::read_file("examples", DAY);
        // println!("{:?}", input);
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
