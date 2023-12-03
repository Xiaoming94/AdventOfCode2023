use advent_of_code::{day2, utils::read_input};

fn main() -> Result<(), std::io::Error> {
    let input_lines = read_input();
    let cube_game = day2::Game::new(12u32,13u32,14u32);
    let results = day2::check_possible_games(input_lines.as_str(), &cube_game);
    println!("Valid games: {:?}", results);
    println!("sum of valid game ids: {}", results.iter().fold(0, |accu, gameid| accu + gameid));
    Ok(())
}
