use advent_of_code::day7;

use googletest::prelude::*;

#[cfg(test)]
mod day7_acceptance_test {
    use super::*;

    const INPUT: &str = "32T3K 765\n\
                         T55J5 684\n\
                         KK677 28\n\
                         KTJJT 220\n\
                         QQQJA 483";

    #[googletest::test]
    fn problem1() {
        let result = day7::compute_hands_bid_value(INPUT);
        expect_that!(result, eq(6440));
    }

    #[googletest::test]
    fn problem2() {
        let result = day7::compute_hands_bid_jokers(INPUT);
        expect_that!(result, eq(5905));
    }
}

#[cfg(test)]
mod day7_problem1_tests {
    use super::*;

    #[googletest::test]
    fn tc_1hand() {
        let input = "31234 12";
        let result = day7::compute_hands_bid_value(input);
        expect_that!(result, eq(12));
    }

    #[googletest::test]
    fn tc_1hand2() {
        let input = "KKKJJ 15";
        let result = day7::compute_hands_bid_value(input);
        expect_that!(result, eq(15));
    }

    #[googletest::test]
    fn tc_2hands_1stbigger() {
        let input = "KKKJJ 15\n\
                     QQQ12 12";
        let result = day7::compute_hands_bid_value(input);
        expect_that!(result, eq(42));
    }

    #[googletest::test]
    fn tc_2hands_2ndbigger() {
        let input = "KKJJ2 15\n\
                     QQQ77 12";
        let result = day7::compute_hands_bid_value(input);
        expect_that!(result, eq(39));
    }

    #[googletest::test]
    fn tc_2hands_one_underordered() {
        let input = "Q2JJJ 15\n\
                     QTQQT 12";
        let result = day7::compute_hands_bid_value(input);
        expect_that!(result, eq(39));
    }
}

#[cfg(test)]
mod day7_problem2_tests {
    use super::*;

    #[googletest::test]
    fn tc_1hand() {
        let input = "31234 12";
        let result = day7::compute_hands_bid_jokers(input);
        expect_that!(result, eq(12));
    }

    #[googletest::test]
    fn tc_1hand2() {
        let input = "KKKJJ 15";
        let result = day7::compute_hands_bid_jokers(input);
        expect_that!(result, eq(15));
    }

    #[googletest::test]
    fn tc_2hands_1stbigger() {
        let input = "KKKJJ 15\n\
                     QQQ12 12";
        let result = day7::compute_hands_bid_jokers(input);
        expect_that!(result, eq(42));
    }

    #[googletest::test]
    fn tc_2hands_2ndbigger() {
        let input = "KKJJ2 15\n\
                     QQQ77 12";
        let result = day7::compute_hands_bid_jokers(input);
        expect_that!(result, eq(42));
    }

    #[googletest::test]
    fn tc_2hands_one_underordered() {
        let input = "Q2JJJ 15\n\
                     QTQQT 12";
        let result = day7::compute_hands_bid_jokers(input);
        expect_that!(result, eq(42));
    }
}
