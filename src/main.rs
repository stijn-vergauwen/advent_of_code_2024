mod day_1;
mod day_2;
mod utilities;

use day_2::part_2::calculate_solution;
use utilities::{load_input_from_asset_folder, save_result_to_asset_folder};

// How to use:
// 1. set the correct file to load and write with these consts (in the case that part 1 & 2 of a day always use the same input, input_part is always 1).
// 2. import the 'calculate_solution' from the correct module.

const DAY: u8 = 2;
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
