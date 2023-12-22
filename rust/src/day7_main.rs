use advent_of_code::{day7, utils};

fn main() -> Result<(), std::io::Error> {
    println!("======= ADVENT OF CODE DAY 7 =======");
    let hands_bid_data = utils::read_input();
    let result = day7::compute_hands_bid_value(hands_bid_data.as_str());
    println!("PROBLEM 1 === product of hands rank: {}", result);

    let joker_results = day7::compute_hands_bid_jokers(hands_bid_data.as_str());
    println!(
        "PROBLEM 2 === product of hands rank with Jokers: {}",
        joker_results
    );
    Ok(())
}
