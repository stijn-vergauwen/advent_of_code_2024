mod day_1;
mod utilities;

use day_1::part_2::calculate_solution;
use utilities::{load_input_from_asset_folder, save_result_to_asset_folder};

const DAY: u8 = 1;
const INPUT_PART: u8 = 1;
const RESULT_PART: u8 = 2;

fn main() {
    let input_data = load_input_from_asset_folder(DAY, INPUT_PART)
        .expect("Input file should be present in asset folder and should be able to be read.");

    let result = calculate_solution(input_data);

    println!("{}", result);

    save_result_to_asset_folder(result, DAY, RESULT_PART)
        .expect("Result file should be able to be written in asset folder.");
}
