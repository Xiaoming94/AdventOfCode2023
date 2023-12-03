use std::collections::HashMap;
use std::vec::Vec;
#[derive(Debug, Eq, PartialEq, Hash)]
enum Color
{
    Red,
    Green,
    Blue,
    Undefined
}

type GameRound = HashMap<Color, u32>;
pub struct Game
{
    red_cubes: u32,
    green_cubes: u32,
    blue_cubes: u32
}

impl Game {
    pub fn new(red_cubes: u32,
               green_cubes: u32,
               blue_cubes: u32) -> Self {
        
        Game {
            red_cubes: red_cubes,
            green_cubes: green_cubes,
            blue_cubes: blue_cubes
        }
    }

    fn is_round_possible(&self, round: &GameRound) -> bool
    {
        round.iter().all(|(color, &n)|{
            match color {
                Color::Red => n <= self.red_cubes,
                Color::Green => n <= self.green_cubes,
                Color::Blue => n <= self.blue_cubes,
                Color::Undefined => false
            }
        })
    }
}

fn to_gameround(round_str: &str) -> GameRound
{
    round_str.split(", ")
             .map(|cubes_color| 
                  {
                      let color_ncubes:Vec<&str> = cubes_color.split_whitespace().collect();
                      let number = color_ncubes[0].parse::<u32>().unwrap();
                      let color = color_ncubes[1].to_lowercase();
                      match color.as_str() {
                        "red"   => (Color::Red, number),
                        "blue"  => (Color::Blue, number),
                        "green" => (Color::Green, number),
                        _       => (Color::Undefined, number)
                      }
                  }).collect::<GameRound>()
}

fn gameline_to_roundsvec(line: &str) -> Vec<GameRound>
{
    let game_round_split: Vec<&str> = line.split(":").collect();
    let rounds_str = game_round_split[1].to_string();
    rounds_str.split(";")
              .map(|round_str| to_gameround(round_str))
              .collect()
}

pub fn check_possible_games(input: &str, game: &Game) -> Vec<u32>
{
    let game_round_strings:Vec<&str> = input.split("\n").collect();
    let mut valid_games = Vec::<u32>::new();
    let mut index = 1u32;
    for g in game_round_strings.iter()
                               .map(|line| { gameline_to_roundsvec(line) } )
    {
        if g.iter().all(|round| {game.is_round_possible(round)})
        {
            valid_games.push(index);
        }
        index += 1;
    }
    valid_games
}

#[cfg(test)]
mod unit_tests {
    use std::collections::HashMap;

    use crate::day2::{Game, Color};
    use super::*;
    #[test]
    fn game_init_test()
    {
        Game::new(1,2,3);
    }

    #[test]
    fn is_round_possible_1redcube_test()
    {
        let cube_game = Game::new(1,2,3);
        let round = HashMap::from([
            (Color::Red, 1)
        ]);
        assert!(cube_game.is_round_possible(&round));
    }
    
    #[test]
    fn is_round_possible_1greencube_test()
    {
        let cube_game = Game::new(1,2,3);
        let round = HashMap::from([
            (Color::Green, 1)
        ]);
        assert!(cube_game.is_round_possible(&round));
    }
    
    #[test]
    fn is_round_possible_1bluecube_test()
    {
        let cube_game = Game::new(1,2,3);
        let round = HashMap::from([
            (Color::Blue, 1)
        ]);
        assert!(cube_game.is_round_possible(&round));
    }
    
    #[test]
    fn is_round_possible_1of_each_color_test()
    {
        let cube_game = Game::new(1,2,3);
        let round = HashMap::from([
            (Color::Blue, 1),
            (Color::Red, 1),
            (Color::Green, 1)
        ]);
        assert!(cube_game.is_round_possible(&round));
    }

    #[test]
    fn is_round_possible_2blue_1green_test()
    {
        let cube_game = Game::new(1,2,3);
        let round = HashMap::from([
            (Color::Blue, 2),
            (Color::Green, 1)
        ]);
        assert!(cube_game.is_round_possible(&round));
    }

    #[test]
    fn is_round_possible_toomany_red_cubes()
    {
        let cube_game = Game::new(1,2,3);
        let round = HashMap::from([
            (Color::Blue, 1),
            (Color::Red, 2),
            (Color::Green, 1)
        ]);
        assert!(!cube_game.is_round_possible(&round));
    }
}