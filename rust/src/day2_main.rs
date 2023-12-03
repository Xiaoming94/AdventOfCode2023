use advent_of_code::{day2, utils::read_input};

fn main() -> Result<(), std::io::Error> {
    let input_lines = read_input();
    
    println!("\n==== STARTING DAY 2 PROBLEM 1 ====\n");
    let cube_game = day2::Game::new(12u32,13u32,14u32);
    let results = day2::check_possible_games(input_lines.as_str(), &cube_game);
    println!("Valid games: {:?}", results);
    println!("sum of valid game ids: {}", results.iter().fold(0, |accu, gameid| accu + gameid));
    
    println!("\n==== STARTING DAY 2 PROBLEM 1 ====\n");
    let results2 = day2::calc_power_minimum_cubes(&input_lines);
    println!("Power of Minimum cubes on each game: {:?}", results2);
    println!("Sum of powers: {}", results2.iter().sum::<u32>());
    Ok(())
}
