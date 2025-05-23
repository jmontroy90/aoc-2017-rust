use std::fs;
use std::io;

pub fn run_day1(input_file: &str) {
    let input = load_input(input_file);
    if let Ok(ins) = input {
        println!("{}", part1(&ins));
        println!("{}", part2(&ins));
    }
}
fn sum_equal_elems(input: &[u32], offset: usize) -> u32 {
    let mut run_sum = 0;
    for (i, c) in input.iter().enumerate() {
        let cmp_ix = (i + offset) % input.len();
        if *c == input[cmp_ix] {
            run_sum += c
        }
    }
    run_sum
}

fn part1(input: &[u32]) -> u32 {
    sum_equal_elems(input, 1)
}

fn part2(input: &[u32]) -> u32 {
    sum_equal_elems(input, input.len() / 2)
}

fn load_input(file_name: &str) -> Result<Vec<u32>, io::Error> {
    let content = fs::read_to_string(file_name)?;
    let digits: Vec<u32> = content.chars().flat_map(|c| c.to_digit(10)).collect();
    Ok(digits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let tests: Vec<(&str, u32)> = vec![
            ("1122", 3),
            ("1234", 0),
            ("1111", 4),
            ("91212129", 9)
        ];
        for (input, expected) in tests {
            assert_eq!(part1(&input.to_digits()), expected)    
        }
    }

    #[test]
    fn test_part2() {
        let tests: Vec<(&str, u32)> = vec![
            ("1212", 6),
            ("1221", 0),
            ("123425", 4),
            ("123123", 12),
            ("12131415", 4),
        ];
        for (input, expected) in tests {
            assert_eq!(part2(&input.to_digits()), expected);
        }
    }


}

trait ToDigits {
    fn to_digits(&self) -> Vec<u32>;
}

impl ToDigits for &str {
    fn to_digits(&self) -> Vec<u32> {
        self.chars().filter_map(|c| c.to_digit(10)).collect()
    }
}