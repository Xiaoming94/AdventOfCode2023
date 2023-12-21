use advent_of_code::day4;
use advent_of_code::utils;

fn main() -> Result<(), std::io::Error> {
    let card_input_str = utils::read_input();
    let matching_nums = day4::find_winning_card_scores(&card_input_str);
    println!("===== Results =====");
    println!("{:?}", matching_nums);
    let scores: u32 = matching_nums.values().map(day4::calc_score).sum();
    println!("Scores: {}", scores);

    let card_pile_size = day4::calc_card_pile_size(&card_input_str);
    println!("Card pile size {}", card_pile_size);
    Ok(())
}
