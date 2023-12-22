use std::iter::zip;

type RaceTable = Vec<(f64, f64)>;

fn build_race_table(race_data: &str) -> RaceTable {
    if let Some((times, distances)) = race_data.split_once("\n") {
        fn build_table(table_data: &str) -> Vec<f64> {
            if let Some((_, numbers)) = table_data.split_once(":") {
                numbers
                    .split_whitespace()
                    .map(|digit| digit.parse::<f64>().unwrap())
                    .collect()
            } else {
                Vec::new()
            }
        }

        let race_times = build_table(times);
        let race_distances = build_table(distances);
        zip(race_times, race_distances).collect()
    } else {
        panic!("Invalid input data")
    }
}

fn join_race_data(race_data: &str) -> (f64, f64) {
    if let Some((times, distances)) = race_data.split_once("\n") {
        fn join_data(table_data: &str) -> f64 {
            if let Some((_, numbers)) = table_data.split_once(":") {
                let joined_numbers: String = numbers.split_whitespace().collect();
                joined_numbers.parse::<f64>().unwrap()
            } else {
                0.0
            }
        }

        let race_time = join_data(times);
        let race_distance = join_data(distances);
        (race_time, race_distance)
    } else {
        panic!("Invalid input data")
    }
}

// The main player for solving the problems in Advent of Code 2023 day 6
// The solution no matter the numbers can be described as finding the integers within
// the range of t1 and t2
// Where t1 and t2 are solutions for the quadratic equation t(time - t) = distance

// This function looks like one of my python functions from Uni during my masters time.....
fn find_race_solutions(time: f64, distance: f64) -> Option<u64> {
    println!("time: {time}, distance: {distance}");
    let root: f64 = ((time / 2.0).powf(2.0) - distance).sqrt();
    if root.is_nan() {
        None
    } else if root.eq(&0.0) {
        Some(1)
    } else {
        let pos_term = time / 2.0;
        let mut upper_limit = (pos_term + root).floor();
        let mut lower_limit = (pos_term - root).ceil();

        println!("upper_value: {}, lower_value {}", upper_limit, lower_limit);

        println!("raced_distance: {}", (upper_limit * (time - upper_limit)));
        while (upper_limit * (time - upper_limit)) <= distance {
            upper_limit -= 1.0;
        }

        println!("raced_distance: {}", (lower_limit * (time - lower_limit)));
        while (lower_limit * (time - lower_limit)) <= distance {
            lower_limit += 1.0;
        }

        Some((upper_limit - lower_limit + 1.0) as u64)
    }
}

pub fn find_winning_solutions(race_data: &str) -> Vec<u64> {
    let races: RaceTable = build_race_table(race_data);
    races
        .into_iter()
        .map(|(time, distance)| find_race_solutions(time, distance))
        .filter(|solutions| solutions.is_some())
        .map(|solutions| solutions.unwrap())
        .collect()
}

pub fn find_joined_race_solution(race_data: &str) -> u64 {
    let (time, distance) = join_race_data(race_data);
    if let Some(result) = find_race_solutions(time, distance) {
        result
    } else {
        panic!("data yielded no good solution, {}", race_data);
    }
}
