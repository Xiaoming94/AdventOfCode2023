use std::collections::HashMap;
use std::vec::Vec;
#[derive(Debug, Eq, PartialEq, Hash)]
enum Color
{
    Red,
    Green,
    Blue
}
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

    fn is_round_possible(&self, round: HashMap<Color, u32>) -> bool
    {
        round.iter().all(|(color, &n)|{
            match color {
                Color::Red => n <= self.red_cubes,
                Color::Green => n <= self.green_cubes,
                Color::Blue => n <= self.blue_cubes,
            }
        })
    }
}

fn gameline_to_hashmap(line: String) -> HashMap<Color, u32>
{
    HashMap::new()
}

pub fn check_possible_games(input: String, game: Game) -> Vec<u32>
{
    let game_round_strings:Vec<&str> = input.split("\n").collect();
    let rounds = game_round_strings.iter()
                                                                         .map(|line| { gameline_to_hashmap(line.to_string()) } );
    Vec::new()
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
        assert!(cube_game.is_round_possible(round));
    }
    
    #[test]
    fn is_round_possible_1greencube_test()
    {
        let cube_game = Game::new(1,2,3);
        let round = HashMap::from([
            (Color::Green, 1)
        ]);
        assert!(cube_game.is_round_possible(round));
    }
    
    #[test]
    fn is_round_possible_1bluecube_test()
    {
        let cube_game = Game::new(1,2,3);
        let round = HashMap::from([
            (Color::Blue, 1)
        ]);
        assert!(cube_game.is_round_possible(round));
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
        assert!(cube_game.is_round_possible(round));
    }

    #[test]
    fn is_round_possible_2blue_1green_test()
    {
        let cube_game = Game::new(1,2,3);
        let round = HashMap::from([
            (Color::Blue, 2),
            (Color::Green, 1)
        ]);
        assert!(cube_game.is_round_possible(round));
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
        assert!(!cube_game.is_round_possible(round));
    }
}