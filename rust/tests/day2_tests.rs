use googletest::prelude::*;
use advent_of_code::day2;

const NO_VALID_GAMES: Vec<u32> = Vec::<u32>::new();

#[cfg(test)]
mod day2_acceptance_tests {
    use super::*;
    const PROBLEM_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
                                 Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
                                 Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
                                 Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
                                 Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[googletest::test]
    fn problem1_test()
    {
        let cube_game = day2::Game::new(12u32,13u32,14u32);
        let results = day2::check_possible_games(PROBLEM_INPUT, &cube_game);
        let results_sum = results.iter().fold(0, |accu, &n| {accu + n});
        expect_that!(results, eq(vec![1u32, 2u32, 5u32]));
        expect_that!(results_sum, eq(8));
    }

    #[googletest::test]
    #[ignore]
    fn problem2_test()
    {
        let results = day2::calc_power_minimum_cubes(PROBLEM_INPUT);
        let results_sum = results.iter().fold(0, |accu, &n| {accu + n});
        expect_that!(results, eq(vec![48u32, 12u32, 1560u32, 630u32, 36u32]));
        expect_that!(results_sum, eq(2286));
    }
}

#[cfg(test)]
mod day2_problem1_tests {
    use super::*;
    #[googletest::test]
    fn tc_1game_1bluecube()
    {
        let input = "Game 1: 1 blue";
        let cube_game = day2::Game::new(12,13,14);

        let results = day2::check_possible_games(input, &cube_game);
        expect_that!(results, eq(vec![1]));
    }

    #[googletest::test]
    fn tc_1game_validround()
    {
        let input = "Game 1: 3 blue, 1 red, 1 green";
        let cube_game = day2::Game::new(12,13,14);

        let results = day2::check_possible_games(input, &cube_game);
        expect_that!(results, eq(vec![1]));
    }

    #[googletest::test]
    fn tc_1game_invalidround()
    {
        let input = "Game 1: 3 blue, 14 red, 1 green";
        let cube_game = day2::Game::new(12,13,14);

        let results = day2::check_possible_games(input, &cube_game);
        expect_that!(results, eq(NO_VALID_GAMES));
    }

    #[googletest::test]
    fn tc_1game_1valid_1invalidrounds()
    {
        let input = "Game 1: 3 blue, 1 red, 1 green; 14 red, 4 green";
        let cube_game = day2::Game::new(12,13,14);

        let results = day2::check_possible_games(input, &cube_game);
        expect_that!(results, eq(NO_VALID_GAMES));
    }

    #[googletest::test]
    fn tc_2games_validrounds()
    {
        let input = "Game 1: 3 blue, 1 red, 1 green; 5 red, 4 green\n\
                        Game 2: 5 blue, 10 red";
        let cube_game = day2::Game::new(12,13,14);

        let results = day2::check_possible_games(input, &cube_game);
        expect_that!(results, eq(vec![1,2]));
    }

    #[googletest::test]
    fn tc_2games_1invalidgame()
    {
        let input = "Game 1: 3 blue, 1 red, 1 green; 5 red, 4 green\n\
                        Game 2: 5 blue, 20 red";
        let cube_game = day2::Game::new(12,13,14);

        let results = day2::check_possible_games(input, &cube_game);
        expect_that!(results, eq(vec![1]));
    }
} 

#[cfg(test)]
mod day2_problem2_tests {
    use super::*;
    #[googletest::test]
    fn tc_1game_1redcube ()
    {
        let input = "Game1: 1 red";
        
        let results = day2::calc_power_minimum_cubes(input);
        expect_that!(results, eq(vec![1]));
    }

    #[googletest::test]
    fn tc_1game_5balls_total()
    {
        let input = "Game 1: 3 blue, 1 red, 1 green";

        let results = day2::calc_power_minimum_cubes(input);
        expect_that!(results, eq(vec![3]));
    }
}