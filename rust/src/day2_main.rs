use advent_of_code::day2;
use std::io::BufRead;

fn main() -> Result<(), std::io::Error> {
    let input_io = std::io::stdin();
    let mut input_lines = String::new();
    for line in input_io.lock().lines() {
        input_lines.push_str(line.unwrap().as_str());
        input_lines.push_str("\n");
    }
    let mut input_lines = input_lines.chars();
    input_lines.next_back();
    let cube_game = day2::Game::new(12u32,13u32,14u32);
    let results = day2::check_possible_games(input_lines.as_str(), &cube_game);
    println!("Valid games: {:?}", results);
    println!("sum of valid game ids: {}", results.iter().fold(0, |accu, gameid| accu + gameid));
    Ok(())
}
