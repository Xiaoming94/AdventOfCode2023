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

#[cfg(test)]
mod problem1_tests {
    use super::*;

    #[googletest::test]
    fn tc_1node_left() {
        let graph_str = "L\n\
                          \n\
                          AAA = (ZZZ, ZZZ)\n;
                          ZZZ = (ZZZ, ZZZ)";

        let result = day8::find_path_through_network(graph_str);
        expect_that!(result, eq(1));
    }

    #[googletest::test]
    fn tc_1node_right() {
        let graph_str = "R\n\
                          \n\
                          AAA = (ZZZ, ZZZ)\n
                          ZZZ = (ZZZ, ZZZ)";

        let result = day8::find_path_through_network(graph_str);
        expect_that!(result, eq(1));
    }

    #[googletest::test]
    fn tc_2nodes_2right() {
        let graph_str = "RR\n\
                          \n\
                          AAA = (ZZZ, BBB)\n\
                          BBB = (AAA, ZZZ)\n\
                          ZZZ = (ZZZ, ZZZ)";

        let result = day8::find_path_through_network(graph_str);
        expect_that!(result, eq(2));
    }

    #[googletest::test]
    fn tc_2nodes_1right() {
        let graph_str = "R\n\
                          \n\
                          AAA = (ZZZ, BBB)\n\
                          BBB = (AAA, ZZZ)\n\
                          ZZZ = (ZZZ, ZZZ)";

        let result = day8::find_path_through_network(graph_str);
        expect_that!(result, eq(2));
    }

    #[googletest::test]
    fn tc_2nodes_1left() {
        let graph_str = "L\n\
                          \n\
                          AAA = (ZZZ, BBB)\n\
                          BBB = (AAA, ZZZ)\n\
                          ZZZ = (ZZZ, ZZZ)";

        let result = day8::find_path_through_network(graph_str);
        expect_that!(result, eq(1));
    }

    #[googletest::test]
    fn tc_2nodes_2left() {
        let graph_str = "LL\n\
                          \n\
                          AAA = (ZZZ, BBB)\n\
                          BBB = (AAA, ZZZ)\n\
                          ZZZ = (ZZZ, ZZZ)";

        let result = day8::find_path_through_network(graph_str);
        expect_that!(result, eq(1));
    }

    #[googletest::test]
    fn tc_2nodes_2left_2step() {
        let graph_str = "LL\n\
                          \n\
                          AAA = (BBB, BBB)\n\
                          BBB = (ZZZ, ZZZ)\n\
                          ZZZ = (ZZZ, ZZZ)";

        let result = day8::find_path_through_network(graph_str);
        expect_that!(result, eq(2));
    }

    #[googletest::test]
    fn tc_3nodes_alternating() {
        let graph_str = "RL\n\
                          \n\
                          AAA = (BBB, CCC)\n\
                          BBB = (AAA, ZZZ)\n\
                          CCC = (ZZZ, BBB)\n\
                          ZZZ = (ZZZ, ZZZ)";

        let result = day8::find_path_through_network(graph_str);
        expect_that!(result, eq(2));
    }

    #[googletest::test]
    fn tc_3nodes_alternating_with_period() {
        let graph_str = "RL\n\
                          \n\
                          AAA = (BBB, CCC)\n\
                          BBB = (AAA, ZZZ)\n\
                          CCC = (BBB, AAA)\n\
                          ZZZ = (ZZZ, ZZZ)";

        let result = day8::find_path_through_network(graph_str);
        expect_that!(result, eq(3));
    }

    #[googletest::test]
    fn tc_3nodes_rightrightleft() {
        let graph_str = "RRL\n\
                          \n\
                          AAA = (BBB, CCC)\n\
                          BBB = (AAA, ZZZ)\n\
                          CCC = (BBB, AAA)\n\
                          ZZZ = (ZZZ, ZZZ)";

        let result = day8::find_path_through_network(graph_str);
        expect_that!(result, eq(4));
    }
}
