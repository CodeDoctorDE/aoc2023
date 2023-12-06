use std::fs;


fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let result = algo(input);
    println!("Result: {}", result);
}

pub fn algo(input: String) -> u32 {
    let lines = input.split("\n");
    let mut result = 0;
    fn get_digit<'a, I>(vals: I) -> u16
    where
        I: Iterator<Item = char>,
    {
        vals.into_iter().find(|c| c.is_digit(10))
            .unwrap_or('0')
            .to_digit(10)
            .unwrap_or_default() as u16
    }
    fn add_digits(digit1: u16, digit2: u16) -> u32 {
        let text = format!("{}{}", digit1, digit2);
        text.parse::<u32>().unwrap_or_default()
    }
    for line in lines {
        let first_number = get_digit(line.chars());
        let last_number = get_digit(line.chars().rev());
        result += add_digits(first_number, last_number);
    }
    result as u32
}

#[cfg(test)]
mod tests {
    use crate::algo;

    #[test]
    fn test_example() {
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        assert_eq!(algo(input.to_string()), 142);
    }
}
