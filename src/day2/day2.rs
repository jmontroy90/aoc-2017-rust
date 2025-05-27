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
        let (min, max) = line.iter().fold((u32::MAX, 0), |(min, max), &i| (min.min(i), max.max(i)));
        sum += max - min;
    };
    sum
}
fn part2(input: Vec<Vec<u32>>) -> u32 {
    let mut sum = 0;
    for line in input {
        sum += find_divisible_sum(line)
    };
    sum
}

fn find_divisible_sum(ns: Vec<u32>) -> u32 {
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

    // #[test]
    // fn test_part1() {
    //     let tests: Vec<(&str, u32)> = vec![
    //         ("1122", 3),
    //         ("1234", 0),
    //         ("1111", 4),
    //         ("91212129", 9)
    //     ];
    //     for (input, expected) in tests {
    //         assert_eq!(part1(&input.to_digits()), expected)    
    //     }
    // }
    // 
    // #[test]
    // fn test_part2() {
    //     let tests: Vec<(&str, u32)> = vec![
    //         ("1212", 6),
    //         ("1221", 0),
    //         ("123425", 4),
    //         ("123123", 12),
    //         ("12131415", 4),
    //     ];
    //     for (input, expected) in tests {
    //         assert_eq!(part2(&input.to_digits()), expected);
    //     }
    // }


}

// trait ToDigits {
//     fn to_digits(&self) -> Vec<u32>;
// }
// 
// impl ToDigits for &str {
//     fn to_digits(&self) -> Vec<u32> {
//         self.chars().filter_map(|c| c.to_digit(10)).collect()
//     }
// }