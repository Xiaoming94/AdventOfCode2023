use advent_of_code::{day6, utils};

fn main() -> Result<(), std::io::Error> {
    println!("====== ADVENT OF CODE DAY 6 =======");
    let race_data = utils::read_input();
    let race_solutions = day6::find_winning_solutions(race_data.as_str());
    println!(
        "PROBLEM 1 == Product of found solutions to races is {}",
        race_solutions.iter().product::<u64>()
    );

    Ok(())
}
