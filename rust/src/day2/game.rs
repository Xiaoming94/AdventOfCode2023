use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Color
{
    Red,
    Green,
    Blue,
    Undefined
}
pub struct Game
{
    red_cubes: u32,
    green_cubes: u32,
    blue_cubes: u32
}

pub type GameRound = HashMap<Color, u32>;

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

    pub fn is_round_possible(&self, round: &GameRound) -> bool
    {
        round.iter().all(|(color, &n)|{
            match color {
                Color::Red       => n <= self.red_cubes,
                Color::Green     => n <= self.green_cubes,
                Color::Blue      => n <= self.blue_cubes,
                Color::Undefined => false
            }
        })
    }
}

#[cfg(test)]
mod unit_tests {
    use std::collections::HashMap;

    use crate::day2::{Game, Color};

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