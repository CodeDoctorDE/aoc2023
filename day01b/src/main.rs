use std::{f32::consts::E, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let result = algo(input);
    println!("Result: {}", result);
}

pub fn algo(input: String) -> u64 {
    let lines = input.split("\n");
    let mut result = 0_u64;
    fn get_digit(text: String, reverse: bool) -> u16 {
        for range in 1_usize..=text.len() {
            for offset in 0..=range {
                let start = if reverse { text.len() - range } else { offset };
                let end = if reverse { text.len() - offset } else { range };
                let slice = &text[start..end];
                let number = slice.parse::<u16>();
                if let Ok(number) = number {
                    return number;
                }
                match slice {
                    "one" => return 1,
                    "two" => return 2,
                    "three" => return 3,
                    "four" => return 4,
                    "five" => return 5,
                    "six" => return 6,
                    "seven" => return 7,
                    "eight" => return 8,
                    "nine" => return 9,
                    _ => continue,
                }
            }
        }
        panic!("No digit found in {}", text);
    }
    fn add_digits(digit1: u16, digit2: u16) -> u32 {
        let text = format!("{}{}", digit1, digit2);
        text.parse::<u32>().unwrap_or_default()
    }
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let first_number = get_digit(line.to_string(), false);
        let last_number = get_digit(line.to_string(), true);
        result += add_digits(first_number, last_number) as u64;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::algo;

    #[test]
    fn test_example() {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        assert_eq!(algo(input.to_string()), 281);
    }
}
