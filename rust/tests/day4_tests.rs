use googletest::prelude::*;
use std::collections::HashMap;

use advent_of_code::day4;

#[cfg(test)]
mod acceptance_tests {

    use std::iter::zip;

    use super::*;
    #[googletest::test]
    fn problem1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
                     Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
                     Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
                     Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
                     Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
                     Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let results = day4::find_winning_card_scores(input);
        let expected_results = HashMap::from([
            ("Card 1", vec![48, 83, 17, 86]),
            ("Card 2", vec![32, 61]),
            ("Card 3", vec![1, 21]),
            ("Card 4", vec![84]),
            ("Card 5", vec![]),
            ("Card 6", vec![]),
        ]);

        for (result, expected_result) in zip(results.clone(), expected_results) {
            expect_that!(result, eq(expected_result))
        }
        let results_score = results
            .values()
            .map(|card_result| 2u32.pow(card_result.len().try_into().unwrap()))
            .sum();

        expect_that!(results_score, eq(13u32));
    }
}
