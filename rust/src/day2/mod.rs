use std::{vec::Vec, collections::HashMap, cmp::max};

pub mod game; 

pub use crate::day2::game::Game;
use crate::day2::game::{Color, GameRound};


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

fn calc_minimum_cubes(game_rounds: &Vec<GameRound>) -> Vec<u32>
{
    let min_games = game_rounds.iter()
                               .fold(HashMap::new(), |mut game_with_mins, game_round| {
                                         {
                                            game_round.iter().for_each(|(color, n_cubes)|
                                            {
                                               game_with_mins.entry(color).and_modify(|current_val| *current_val = max(*current_val, n_cubes))
                                                                          .or_insert(n_cubes);
                                            });
                                         }
                                         game_with_mins
                                     });
    min_games.values().cloned().cloned().collect()
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

pub fn calc_power_minimum_cubes(input: &str) -> Vec<u32>
{
    let game_rounds: Vec<Vec<GameRound>> = input.split("\n").map(gameline_to_roundsvec).collect();
    game_rounds.iter().map(calc_minimum_cubes)
                      .map(|min_cubes| min_cubes.iter()
                                                .fold(1, |power, n_cubes| power * n_cubes ))
                      .collect()
}