use advent_of_code::day6;

use googletest::prelude::*;

#[cfg(test)]
mod acceptance_tests {
    use super::*;

    const INPUT: &str = "Time:      7  15   30\nDistance:  9  40  200";
    #[googletest::test]
    #[ignore]
    fn problem1() {
        let solutions = day6::find_winning_solutions(INPUT);
        let expected_solutions = vec![4u32, 8u32, 9u32];
        expect_that!(solutions, eq(expected_solutions));

        let result: u32 = solutions.iter().product();
        expect_that!(result, eq(288));
    }
}
