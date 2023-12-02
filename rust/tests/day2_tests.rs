use googletest::prelude::*;
use advent_of_code::day2;

#[googletest::test]
#[ignore]
fn day2_problem1_acceptance_test()
{
    let problem1_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
                                Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
                                Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
                                Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
                                Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    
    let cube_game = day2::Game::new(12u32,13u32,14u32);
    let results = day2::check_possible_games(problem1_input.to_string(), cube_game);
    let results_sum = results.iter().fold(0, |accu, &n| {accu + n});
    expect_that!(results, eq(vec![1u32, 2u32, 5u32]));
    expect_that!(results_sum, eq(8));
}

//#[cfg(test)]
//mod day2_problem1_unittests
//{
//    use super::*;
//}