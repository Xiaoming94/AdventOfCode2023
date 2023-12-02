use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Color
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

    pub fn is_round_possible(&self, round: HashMap<Color, u32>) -> bool
    {
        round.iter().all(|(color, &n)|{
            match color {
                Color::Red => n <= self.red_cubes,
                Color::Green => n <= self.green_cubes,
                Color::Blue => n <= self.blue_cubes,
                _ => false
            }
        })
    }
}

pub fn check_possible_games(input: String, game: Game) -> std::vec::Vec<u32>
{
    Vec::new()
}

#[cfg(test)]
mod unit_tests {
    use std::collections::HashMap;

    use crate::day2::{Game, Color};
    #[test]
    fn game_init_test()
    {
        let cube_game = Game::new(1,2,3);
    }

    #[test]
    fn is_rount_possible_1redball_test()
    {
        let cube_game = Game::new(1,2,3);
        let round = HashMap::from([
            (Color::Red, 1)
        ]);
        assert!(cube_game.is_round_possible(round));
    }
    
    #[test]
    fn is_rount_possible_1greenball_test()
    {
        let cube_game = Game::new(1,2,3);
        let round = HashMap::from([
            (Color::Green, 1)
        ]);
        assert!(cube_game.is_round_possible(round));
    }
    
    #[test]
    fn is_rount_possible_1blueball_test()
    {
        let cube_game = Game::new(1,2,3);
        let round = HashMap::from([
            (Color::Blue, 1)
        ]);
        assert!(cube_game.is_round_possible(round));
    }
    
    #[test]
    fn is_rount_possible_1of_each_color_test()
    {
        let cube_game = Game::new(1,2,3);
        let round = HashMap::from([
            (Color::Blue, 1),
            (Color::Red, 1),
            (Color::Green, 1)
        ]);
        assert!(cube_game.is_round_possible(round));
    }
}