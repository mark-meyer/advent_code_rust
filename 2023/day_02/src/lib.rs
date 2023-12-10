use std::collections::HashMap;
use std::cmp::max;
use std::num;
use lazy_static::lazy_static;

// Max values for the game
lazy_static! {
    pub static ref MAX_VALUES:HashMap<String, u32> = HashMap::from([
        (String::from("red"), 12),
        (String::from("green"), 13),
        (String::from("blue"), 14)
    ]);
}

pub fn solve_part_one<I, S>(lines: I) -> u32 
    where 
    I: Iterator<Item=S>,
    S: AsRef<str>
{
    lines
    .flat_map(|line| parse_line(line.as_ref()))
    .filter(|(_, mappings)| mappings.iter().all(is_valid_game))
    .map(|(game_id, _)| game_id)
    .sum()
}

pub fn solve_part_two<I, S>(lines: I) -> u32 
    where
    I: Iterator<Item=S>,
    S: AsRef<str>
{
   lines
   .flat_map(|line| parse_line(line.as_ref()))
   .map(|(_, mappings)| get_power_set(&mappings))
   .sum()
}

#[derive(Debug, Clone)]
pub struct FileParseError;
impl From<num::ParseIntError> for FileParseError {
    fn from(_: num::ParseIntError) -> FileParseError {
        FileParseError
    }
}


fn is_valid_game(game: &HashMap<String, u32>) -> bool{
    game.iter().all(|(k, v)| MAX_VALUES.get(k).unwrap() >= v) 
}

fn get_power_set(game: &Vec<HashMap<String, u32>>) -> u32 {
    let mut totals:HashMap<&str, u32> = HashMap::new();

    for hand in game.iter() {
        for (k, &v) in hand {
            totals
                .entry(k)
                .and_modify(|max_score| *max_score = max(*max_score, v))
                .or_insert(v);
        }
    };

    totals.values().product()
}

fn parse_line(s: &str) -> Result<(u32, Vec<HashMap<String, u32>>), FileParseError> {
    let (game, plays) = s
        .split_once(": ")
        .ok_or(FileParseError)?;

    let (_, game_id) = game
        .split_once(" ")
        .ok_or(FileParseError)?;

    let game_id:u32 = game_id.parse()?;

    let mut games:Vec<HashMap<String, u32>> = Vec::new();

    for play in plays.split("; "){
        let mut mapping:HashMap<String, u32> = HashMap::new();

        for cube in play.split(", "){
            let (n, color) = cube
                .split_once(char::is_whitespace)
                .ok_or(FileParseError)?;

            mapping.insert(color.to_string(), n.parse()?);
        }
        games.push(mapping);
       
    }

    Ok((game_id, games))
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line(){
        let (id, games) = parse_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").unwrap();
       
        assert_eq!(id, 1);
        assert_eq!(games[0], HashMap::from([
            (String::from("blue"), 3), 
            (String::from("red"), 4) 
            ])
        );

        assert_eq!(games[1], HashMap::from([
            (String::from("blue"), 6), 
            (String::from("red"), 1), 
            (String::from("green"), 2) 
            ])
        );
    }

    #[test]
    fn test_invalid(){
        let invalid_game = HashMap::from([
            (String::from("blue"), 6), 
            (String::from("red"), 1), 
            (String::from("green"), 22) 
        ]);

        assert_eq!(is_valid_game(&invalid_game), false)
    }

    #[test]
    fn test_valid(){
        let valid_game = HashMap::from([
            (String::from("blue"), 6), 
            (String::from("red"), 1), 
            (String::from("green"), 2) 
        ]);

        assert_eq!(is_valid_game(&valid_game), true)
    }

    #[test]
    fn power_set() {
        let hands = vec![
            HashMap::from([
            (String::from("blue"), 6), 
            (String::from("red"), 1), 
            (String::from("green"), 2) 
        ]),
        HashMap::from([
            (String::from("blue"), 3), 
            (String::from("red"), 4) 
            ])
        ];
    
        assert_eq!(get_power_set(&hands), 48)
    }

}