use advent_of_code::day5;

use googletest::prelude::*;

#[cfg(test)]
mod acceptance_tests {
    use super::*;
    const INPUT: &str = "seeds: 79 14 55 13\n\
\n\
                        seed-to-soil map:\n\
                        50 98 2\n\
                        52 50 48\n\
\n\
                        soil-to-fertilizer map:\n\
                        0 15 37\n\
                        37 52 2\n\
                        39 0 15\n\
\n\
                        fertilizer-to-water map:\n\
                        49 53 8\n\
                        0 11 42\n\
                        42 0 7\n\
                        57 7 4\n\
\n\
                        water-to-light map:\n\
                        88 18 7\n\
                        18 25 70\n\
\n\
                        light-to-temperature map:\n\
                        45 77 23\n\
                        81 45 19\n\
                        68 64 13\n\
\n\
                        temperature-to-humidity map:\n\
                        0 69 1\n\
                        1 0 69\n\
\n\
                        humidity-to-location map:\n\
                        60 56 37\n\
                        56 93 4";
    #[googletest::test]
    #[ignore]
    fn problem1() {
        let result = day5::find_lowest_location(INPUT);
        expect_that!(result, eq(35));
    }
}

#[cfg(test)]
mod problem1_tests {
    use super::*;

    #[googletest::test]
    fn tc_only_ones () {
        let input = "seeds: 1\n\
                        seed-to-soil map:\n\
                        1 1 1\n\
\n\
                        soil-to-fertilizer map:\n\
                        1 1 1\n\
\n\
                        fertilizer-to-water map:\n\
                        1 1 1\n\
\n\
                        water-to-light map:\n\
                        1 1 1\n\
\n\
                        light-to-temperature map:\n\
                        1 1 1\n\
\n\
                        temperature-to-humidity map:\n\
                        1 1 1\n\
\n\
                        humidity-to-location map:\n\
                        1 1 1";

       let result = day5::find_lowest_location(input);
       expect_that!(result, eq(1)); 
    }
    
    #[googletest::test]
    fn tc_single_line_maps_only () {
        //Path 3 -> 6 -> 4 -> 4 -> 4 -> 8 -> 9
        let input = "seeds: 3\n\
                        seed-to-soil map:\n\
                        6 3 12\n\
\n\
                        soil-to-fertilizer map:\n\
                        4 6 2\n\
\n\
                        fertilizer-to-water map:\n\
                        1 1 1\n\
\n\
                        water-to-light map:\n\
                        4 7 4\n\
\n\
                        light-to-temperature map:\n\
                        1 1 1\n\
\n\
                        temperature-to-humidity map:\n\
                        5 1 13\n\
\n\
                        humidity-to-location map:\n\
                        3 2 6";

       let result = day5::find_lowest_location(input);
       expect_that!(result, eq(9)); 
    }

    #[googletest::test]
    fn tc_single_line_maps_2_seeds () {
        //Path seed 3 -> 6 -> 4 -> 4 -> 4 -> 8 -> 9
        //Path seed 2 -> 2 -> 2 -> 2 -> 2 -> 6 -> 7
        let input = "seeds: 3 2\n\
                        seed-to-soil map:\n\
                        6 3 12\n\
\n\
                        soil-to-fertilizer map:\n\
                        4 6 2\n\
\n\
                        fertilizer-to-water map:\n\
                        1 1 1\n\
\n\
                        water-to-light map:\n\
                        4 7 4\n\
\n\
                        light-to-temperature map:\n\
                        1 1 1\n\
\n\
                        temperature-to-humidity map:\n\
                        5 1 13\n\
\n\
                        humidity-to-location map:\n\
                        3 2 6";

       let result = day5::find_lowest_location(input);
       expect_that!(result, eq(7)); 
    }

    #[googletest::test]
    fn tc_2lines_on_some_maps_2_seeds () {
        //Path seed 3 -> 6 -> 4 -> 13 -> 13 -> 17 -> 17
        //Path seed 2 -> 2 -> 2 -> 2 -> 5 -> 9 -> 9
        let input = "seeds: 3 2\n\
                        seed-to-soil map:\n\
                        6 3 12\n\
\n\
                        soil-to-fertilizer map:\n\
                        4 6 2\n\
\n\
                        fertilizer-to-water map:\n\
                        1 1 1\n\
                        12 3 6\n\
\n\
                        water-to-light map:\n\
                        4 7 4\n\
\n\
                        light-to-temperature map:\n\
                        1 1 1\n\
                        5 2 5\n\
\n\
                        temperature-to-humidity map:\n\
                        5 1 13\n\
\n\
                        humidity-to-location map:\n\
                        3 2 6";

       let result = day5::find_lowest_location(input);
       expect_that!(result, eq(9)); 
    }
}