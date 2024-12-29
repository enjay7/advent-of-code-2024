advent_of_code::solution!(3);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u64> {
    //Regex to match valid mul(x,y) pattern
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    // Iterate over all matches, extract x and y and sum their product
    let total: u64 = re.captures_iter(input)
        .map(|cap| {
            let x: u64 = cap[1].parse().unwrap();
            let y: u64 = cap[2].parse().unwrap();
            x * y
        })
        .sum();

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    // Regex patterns to match mul(x,y), do(), don't()
    let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut total: u64 = 0;
    let mut mul_enabled = true;

    for cap in re.captures_iter(input) {
        match &cap[0] {
            "do()" => mul_enabled = true,
            "don't()" => mul_enabled = false,
            _ => {
                if mul_enabled {
                    let x: u64 = cap[1].parse().unwrap();
                    let y: u64 = cap[2].parse().unwrap();
                    total += x * y;
                }
            }
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
