use advent_of_code::{day8, utils};

fn main() -> Result<(), std::io::Error> {
    println!("======= ADVENT OF CODE DAY 8 =======");
    let graph_input = utils::read_input();
    let results = day8::find_path_through_network(graph_input.as_str());
    println!(
        "PROBLEM1 == number of steps to complete the walk is: {}",
        results
    );

    Ok(())
}
