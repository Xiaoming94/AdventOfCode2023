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
}

pub fn check_possible_games(input: String, game: Game) -> std::vec::Vec<u32>
{
    Vec::new()
}