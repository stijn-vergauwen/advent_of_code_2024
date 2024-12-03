mod days;
mod utilities;

use days::day_1_1::calculate_solution;
use utilities::{load_input_from_asset_folder, save_result_to_asset_folder};

const DAY: u8 = 1;
const CHALLENGE: u8 = 1;

fn main() {
    let input_data = load_input_from_asset_folder(DAY, CHALLENGE)
        .expect("Input file should be present in asset folder and should be able to be read.");

    let result = calculate_solution(input_data);

    println!("{}", result);

    save_result_to_asset_folder(result, DAY, CHALLENGE)
        .expect("Result file should be able to be written in asset folder.");
}
