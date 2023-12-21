use advent_of_code::day5;
use advent_of_code::utils;

fn main() -> Result<(), std::io::Error> {
    println!("======= RESULTS FOR DAY5 =======");
    let input_almanac = utils::read_input();
    let smallest_location = day5::find_lowest_location(input_almanac.as_str());
    println!("Lowest seed location: {}", smallest_location);
    Ok(())
}
