use advent_of_code::day8;

use googletest::prelude::*;

#[cfg(test)]
mod day8_acceptance_test {
    use super::*;

    const INPUT_1: &str = "RL\n\
                             \n\
                           AAA = (BBB, CCC)\n\
                           BBB = (DDD, EEE)\n\
                           CCC = (ZZZ, GGG)\n\
                           DDD = (DDD, DDD)\n\
                           EEE = (EEE, EEE)\n\
                           GGG = (GGG, GGG)\n\
                           ZZZ = (ZZZ, ZZZ)";

    const INPUT_2: &str = "LLR\n\
                              \n\
                           AAA = (BBB, BBB)\n\
                           BBB = (AAA, ZZZ)\n\
                           ZZZ = (ZZZ, ZZZ)";
    #[googletest::test]
    #[ignore]
    fn problem1_input1() {
        let result = day8::find_path_through_network(INPUT_1);
        expect_that!(result, eq(2u32));
    }

    #[googletest::test]
    #[ignore]
    fn problem1_input2() {
        let result = day8::find_path_through_network(INPUT_2);
        expect_that!(result, eq(6u32));
    }
}
