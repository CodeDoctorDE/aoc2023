use std::{error::Error, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let result = algo(input);
    println!("Result: {}", result);
}

struct Game {
    #[allow(dead_code)]
    id: u16,
    red: u8,
    green: u8,
    blue: u8,
}
fn get_game(s: &str) -> Result<Game, Box<dyn Error>> {
    const GAME_PREFIX: &str = "Game ";
    if !s.starts_with(GAME_PREFIX) {
        return Err("Invalid game prefix".into());
    }
    let point_index = s.find(":").ok_or("Invalid game format")?;
    let id = s[GAME_PREFIX.len()..point_index].parse::<u16>()?;
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for part in s[point_index + 1..].split(&[';', ':', ',']) {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        let space_index = part.find(" ").ok_or("Invalid game format")?;
        let color = part[space_index + 1..].trim();
        let value = part[..space_index].trim().parse::<u8>()?;
        match color {
            "red" => red = value.max(red),
            "green" => green = value.max(green),
            "blue" => blue = value.max(blue),
            _ => return Err(format!("Invalid color: {}", color).into()),
        }
    }
    Ok(Game {
        id,
        red,
        green,
        blue,
    })
}

fn get_power(game: &Game) -> u32 {
    (game.red as u32) * (game.green as u32) * (game.blue as u32)
}

fn count_power(games: Vec<Game>) -> u32 {
    let mut count = 0_u32;
    for game in games {
        count += get_power(&game);
    }
    count
}

fn algo(input: String) -> u32 {
    let mut games = Vec::new();
    for line in input.split("\n") {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let game = get_game(line).unwrap();
        games.push(game);
    }
    count_power(games)
}

#[cfg(test)]
mod tests {
    use crate::algo;

    #[test]
    fn test_example() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(algo(input.to_string()), 2286);
    }
}
