use advent_of_code::day6;

use googletest::prelude::*;

#[cfg(test)]
mod day6_acceptance_tests {
    use super::*;

    const INPUT: &str = "Time:      7  15   30\nDistance:  9  40  200";
    #[googletest::test]
    fn problem1() {
        let solutions = day6::find_winning_solutions(INPUT);
        let expected_solutions = vec![4u64, 8u64, 9u64];
        expect_that!(solutions, eq(expected_solutions));

        let result: u64 = solutions.iter().product();
        expect_that!(result, eq(288));
    }

    #[googletest::test]
    fn problem2() {
        let result = day6::find_joined_race_solution(INPUT);
        expect_that!(result, eq(71503));
    }
}

#[cfg(test)]
mod day6_problem1_tests {
    use super::*;

    #[googletest::test]
    fn tc_1race_with_only1nes() {
        let input = "Time: 1\nDistance: 1";
        let solutions = day6::find_winning_solutions(input);
        let expected_solutions: Vec<u64> = vec![];
        expect_that!(solutions, eq(expected_solutions));
    }

    #[googletest::test]
    fn tc_1race_time2_distance_1() {
        let input = "Time: 2\nDistance: 1";
        let solutions = day6::find_winning_solutions(input);
        let expected_solutions: Vec<u64> = vec![1];
        expect_that!(solutions, eq(expected_solutions));

        let result: u64 = solutions.iter().product();
        expect_that!(result, eq(1));
    }

    #[googletest::test]
    fn tc_1race_time6_distance_8() {
        let input = "Time: 6\nDistance: 8";
        let solutions = day6::find_winning_solutions(input);
        let expected_solutions: Vec<u64> = vec![1];
        expect_that!(solutions, eq(expected_solutions));

        let result: u64 = solutions.iter().product();
        expect_that!(result, eq(1));
    }

    #[googletest::test]
    fn tc_2races_time6and12_distance8and32() {
        let input = "Time: 6 12\nDistance: 8 32";
        let solutions = day6::find_winning_solutions(input);
        let expected_solutions: Vec<u64> = vec![1, 3];
        expect_that!(solutions, eq(expected_solutions));

        let result: u64 = solutions.iter().product();
        expect_that!(result, eq(3));
    }
}

/*
 * The general solution for Problem 2 is the same as problem 1.
 * Difference is that now the numbers of the rows are joined instead of separating into their own columns
 * So no new tcs for problem 2 is added
 */
