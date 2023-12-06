use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let result = algo(input);
    println!("Result: {}", result);
}

fn find_numbers(line: &str) -> Vec<(usize, usize)> {
    let mut indexes = Vec::new();
    let mut number_index : Option<usize> = None;
    for (i, c) in line.chars().enumerate() {
        if c.is_digit(10) {
            if number_index.is_none() {
                number_index = Some(i);
            }
        } else {
            if let Some(index) = number_index {
                indexes.push((index, i - index));
            }
            number_index = None;
        }
    }
    if let Some(index) = number_index {
        indexes.push((index, 0));
    }
    indexes
}

fn is_valid(lines: &Vec<&str>, line: usize, index: usize, length: usize) -> bool {
    fn is_symbol(c: char) -> bool {
        !c.is_digit(10) && !c.is_whitespace() && c != '.'
    }
    let mut is_valid = false;
    for current in line.saturating_sub(1)..=line.saturating_add(1) {
        for i in index.saturating_sub(1)..=(index + length + 1) {
            let Some(line) = lines.get(current) else {
                continue;
            };
            let Some(c) = line.chars().nth(i) else {
                continue;
            };
            if is_symbol(c) {
                is_valid = true;
                break;
            }
        }
    }
    is_valid
}
// 531267 too low
// 536849 too high
fn get_number(lines : &Vec<&str>, index : usize, length: usize, line : usize) -> u32 {
    let current = lines.get(line).unwrap_or(&"");
    current[index..(index + length)].parse::<u32>().unwrap_or_default()
}

fn algo(input: String) -> u32 {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let mut count = 0_u32;
    for (number, line) in lines.iter().enumerate() {
        let numbers = find_numbers(line);
        for (index, length) in numbers {
            if is_valid(&lines, number, index, length) {
                count += get_number(&lines, index, length, number);
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::algo;

    #[test]
    fn test_example() {
        let input = "
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
        ";
        assert_eq!(algo(input.to_string()), 4361);
    }
}
