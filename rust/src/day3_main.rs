use advent_of_code::utils;

use advent_of_code::day3;
fn main() -> Result<(), std::io::Error>
{
    let schematic_input = utils::read_input();
    let results = day3::find_partnumbers(&schematic_input);
    println!("Found schematic numbers {:?}", results);
    println!("Resulting sum {:?}", results.iter().sum::<u32>());
    Ok(())
}