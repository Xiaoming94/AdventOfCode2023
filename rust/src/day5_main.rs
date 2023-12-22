use advent_of_code::day5;
use advent_of_code::utils;

fn main() -> Result<(), std::io::Error> {
    println!("======= RESULTS FOR DAY5 =======");
    let input_almanac = utils::read_input();
    let smallest_location_1 = day5::find_lowest_location(input_almanac.as_str());
    let smallest_location_2 = day5::find_lowest_location_v2(input_almanac.as_str());
    println!("== PROBLEM 1 Lowest location number: {}", smallest_location_1);
    println!("== PROBLEM 2 Lowest location number: {}", smallest_location_2);
    Ok(())
}
