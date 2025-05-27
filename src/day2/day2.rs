use std::fs;
use std::io;

pub fn run_day2(input_file: &str) {
    let input = load_input(input_file);
    if let Ok(ins) = input {
        println!("{}", part1(ins.clone()));
        println!("{}", part2(ins));
    }
}


fn part1(input: Vec<Vec<u32>>) -> u32 {
    let mut sum = 0;
    for line in input {
        sum += find_minmax_diff(&line);
    };
    sum
}

fn find_minmax_diff(ns: &[u32]) -> u32 {
    let (min, max) = ns.iter().fold((u32::MAX, 0), |(min, max), &i| (min.min(i), max.max(i)));
    max - min
}

fn part2(input: Vec<Vec<u32>>) -> u32 {
    let mut sum = 0;
    for line in input {
        sum += find_divisible_sum(&line)
    };
    sum
}

fn find_divisible_sum(ns: &[u32]) -> u32 {
    for (i, n1) in ns.iter().enumerate() {
        for n2 in ns[i+1..].iter() {
            if n1 % n2 == 0 {
                return n1 / n2
            } else if n2 % n1 == 0 {
                return n2 / n1
            }
        }
    }
    0 // shouldn't get here
}

fn load_input(file_name: &str) -> Result<Vec<Vec<u32>>, io::Error> {
    let content = fs::read_to_string(file_name)?;
    let parsed = content.lines()
        .map(|line| line.split_ascii_whitespace().map(|d| d.parse::<u32>().unwrap()).collect()).collect();
    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_minmax_diff() {
        let tests: Vec<(&[u32], u32)> = vec![
            (&[5, 1, 9, 5], 8),
            (&[7, 5, 3], 4),
            (&[2, 4, 6, 8], 6),
        ];
        for (input, expected) in tests {
            assert_eq!(find_minmax_diff(input), expected)
        }
    }

    #[test]
    fn test_part2() {
        let tests: Vec<(&[u32], u32)> = vec![
            (&[5, 9, 2, 8], 4),
            (&[9, 4, 7, 3], 3),
            (&[3, 8, 6, 5], 2)
        ];
        for (input, expected) in tests {
            assert_eq!(find_divisible_sum(input), expected);
        }
    }
}
