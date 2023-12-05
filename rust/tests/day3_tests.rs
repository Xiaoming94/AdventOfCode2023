use googletest::prelude::*;
use advent_of_code::day3;

#[cfg(test)]
mod day3_acceptance_tests {
    use super::*;

    const SCHEMATIC: &str = "467..114..\n\
                             ...*......\n\
                             ..35..633.\n\
                             ......#...\n\
                             617*......\n\
                             .....+.58.\n\
                             ..592.....\n\
                             ......755.\n\
                             ...$.*....\n\
                             .664.598..";
    #[googletest::test]
    fn problem1_test()
    {
        let results = day3::find_partnumbers(SCHEMATIC);
        let results_sum = results.iter().sum::<u32>();
        expect_that!(results_sum, eq(4361));
    }

    #[googletest::test]
    #[ignore]
    fn problem2_test()
    {
        let results = day3::find_gear_ratios(SCHEMATIC);
        let results_sum = results.iter().sum::<u32>();
        expect_that!(results_sum, eq(467835))
    }
}

#[cfg(test)]
mod day3_problem1_verification_tests {
    use super::*;

    #[googletest::test]
    fn tc_1line_no_symbols() {
        let input = "...42....";

        let results = day3::find_partnumbers(input);
        expect_that!(results.iter().sum::<u32>(), eq(0));
    }
    
    #[googletest::test]
    fn tc_1line_one_symbol() {
        let input = "...42*...";

        let results = day3::find_partnumbers(input);
        expect_that!(results.iter().sum::<u32>(), eq(42));
    }
    
    #[googletest::test]
    fn tc_1line_one_symbol_2_numbers() {
        let input = "...4*6...";

        let results = day3::find_partnumbers(input);
        expect_that!(results.iter().sum::<u32>(), eq(10));
    }
    
    #[googletest::test]
    fn tc_1line_one_symbol_threechardigit() {
        let input = "...142*...";

        let results = day3::find_partnumbers(input);
        expect_that!(results.iter().sum::<u32>(), eq(142));
    }
 
    #[googletest::test]
    fn tc_1line_symbol_toofar() {
        let input = "...142.*..";

        let results = day3::find_partnumbers(input);
        expect_that!(results.iter().sum::<u32>(), eq(0));
    }
 
    #[googletest::test]
    fn tc_1line_symbol_before() {
        let input = "...*42...";

        let results = day3::find_partnumbers(input);
        expect_that!(results.iter().sum::<u32>(), eq(42));
    }

    #[googletest::test]
    fn tc_2lines_1number_sameline() {
        let input = "...*42...\n\
                     ....#....";

        let results = day3::find_partnumbers(input);
        expect_that!(results.iter().sum::<u32>(), eq(42));
    }

    #[googletest::test]
    fn tc_2lines_1above_1beside() {
        let input = "....42...\n\
                     ....#12...";

        let results = day3::find_partnumbers(input);
        expect_that!(results.iter().sum::<u32>(), eq(54));
    }

    #[googletest::test]
    fn tc_2lines_1diagonal_1beside() {
        let input = "..42.....\n\
                     ....#12...";

        let results = day3::find_partnumbers(input);
        expect_that!(results.iter().sum::<u32>(), eq(54));
    }

    #[googletest::test]
    fn tc_2lines_1number_endingat_endl() {
        let input = ".......42\n\
                     5...12#..";

        let results = day3::find_partnumbers(input);
        expect_that!(results.iter().sum::<u32>(), eq(54));

    }
}