advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let reports = parse_input(input);

    let safe_count: u64 = reports.iter()
        .filter(|report| is_safe(report)) // check if the row is safe
        .count() as u64;
    Some(safe_count)
}

pub fn part_two(_input: &str) -> Option<u64> {
    let reports = parse_input(_input);

    let safe_count: u64 = reports.iter()
        .filter(|report| {
            if is_safe(report) {
                return true;
            }

            // otherwise, check if any of the sub-arrays after removing one element is safe
            (0..report.len()).any(|i| {
                let modified_report: Vec<u64> = report.iter()
                    .enumerate()
                    .filter(|&(j,_)| j != i) // skip i-th element
                    .map(|(_, &x)| x) // collect the remaining elements
                    .collect();
                is_safe(&modified_report)
            })
        })
        .count() as u64;

    Some(safe_count)
}

fn parse_input(input: &str) -> Vec<Vec<u64>> {
    input.lines().map(|l| {
        l.split_whitespace()
            .map(|num| num.parse::<u64>().unwrap())
            .collect()
    })
    .collect()
}

fn is_safe(report: &Vec<u64>) -> bool {
    let inc: Vec<i64> = report
        .windows(2) // Create a sliding window of size 2as
        .map(|pair| pair[1]as i64 - pair[0] as i64)
        .collect();

    //inc now will have a difference list of each of the adjacent pairs in the report i, i+1
    //We will check if all the elements are either strictly increasing or decreasing with in the
    // expected range of 1,2,3 or -1,-2,-3

    // Check if all the differences are in {1,2,3} or {-1,-2,-3}
    inc.iter().all(|&x| (1..=3).contains(&x)) || inc.iter().all(|&x| (-3..=-1).contains(&x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
