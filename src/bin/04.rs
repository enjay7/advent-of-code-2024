advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_input(input);
    let word = "XMAS".chars().collect::<Vec<_>>();
    let directions = vec![
        (0, 1), // Right
        (0, -1), // Left
        (1, 0), // Down
        (-1, 0), // Up
        (1, 1), // Diagonal Down right
        (-1, 1),// Diagonal up-right
        (-1, -1), // Diagonal up-left
        (1, -1), // Diagonal Down left
    ];

    let mut count: u64 = 0;

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            for &(dr, dc) in &directions {
                if is_xmas(&grid, row as i32, col as i32, dr, dc, &word) {
                    count += 1;
                }
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = parse_input(input);
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count: u64 = 0;

    for row in 0..rows {
        for col in 0..cols {
            if is_x_mas(&grid, row, col) {
                count += 1;
            }
        }
    }
    Some(count)
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect())
    .collect()
}

fn is_xmas(grid: &Vec<Vec<char>>, row: i32, col: i32, dr: i32, dc: i32, word: &Vec<char>) -> bool {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    for (i, &ch) in word.iter().enumerate() {
        let nr = row + i as i32 * dr;
        let nc = col + i as i32 * dc;

        // Check bounds and characters match
        if nr < 0 || nr >= rows || nc < 0 || nc >= cols || grid[nr as usize][nc as usize] != ch {
            return false;
        }
    }
    true
}

fn is_x_mas(grid: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    // In this part we know for sure that the center of the x_mas should be a.
    // Lets ignore any cell that is not a potential center for our x_mas
    if grid[row][col] != 'A' {
        return false;
    }

    // row, col should be a center so it cannot be the extreme row/col
    if row < 1 || row >= grid.len() - 1 || col < 1 || col >= grid[0].len() - 1 {
        return false;
    }

    // Check diagonals for MAS patterns
    let top_left = (row - 1, col -1);
    let bottom_right = (row + 1, col + 1);

    let top_right = (row - 1, col + 1);
    let bottom_left = (row + 1, col - 1);

    // Check first diagonal (top left to bottom right)
    let d1_valid = is_valid_mas(grid[top_left.0][top_left.1], grid[bottom_right.0][bottom_right.1]);

    // Check second diagonal (top right to bottom left)
    let d2_valid = is_valid_mas(grid[top_right.0][top_right.1], grid[bottom_left.0][bottom_left.1]);

    d1_valid && d2_valid
}

fn is_valid_mas(c1: char, c2: char) -> bool {
    (c1 == 'M' && c2 == 'S') || (c1 == 'S' && c2 == 'M' )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
