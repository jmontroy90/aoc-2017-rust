use std::fs;

pub fn run_day1(input_file: &str) {
    let input = load_input(input_file);
    println!("{}", day1(input.as_str()));
    println!("{}", day2(input.as_str()));
}

fn day1(input: &str) -> u32 {
    let mut run_sum = 0;
    for (i, c) in input.char_indices() {
        let next_ix = (i + 1) % input.len();
        if c == input.chars().nth(next_ix).unwrap() {
            run_sum += c.to_digit(10).unwrap();
        }
    }
    run_sum
}

fn day2(input: &str) -> u32 {
    let mut run_sum = 0;
    let hop = input.len() / 2;
    for (i, c) in input.char_indices() {
        let cmp_ix = (i + hop) % input.len();
        if c == input.chars().nth(cmp_ix).unwrap() {
            run_sum += c.to_digit(10).unwrap();
        }
    }
    run_sum
}

fn load_input(file_name: &str) -> String {
    fs::read_to_string(file_name).unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1_basic() {
        assert_eq!(day1("1122"), 3);
        assert_eq!(day1("1234"), 0);
        assert_eq!(day1("1111"), 4);
        assert_eq!(day1("91212129"), 9);
    }

    #[test]
    fn test_day2() {
        assert_eq!(day2("1212"), 6);
        assert_eq!(day2("1221"), 0);
        assert_eq!(day2("123425"), 4);
        assert_eq!(day2("123123"), 12);
        assert_eq!(day2("12131415"), 4);
    }


}