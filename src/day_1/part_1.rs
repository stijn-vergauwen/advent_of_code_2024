use super::parse_input_data;

pub fn calculate_solution(input_data: String) -> String {
    let mut input_numbers: [Vec<u32>; 2] = parse_input_data(input_data);

    for numbers in &mut input_numbers {
        numbers.sort();
    }

    let summed_distance = input_numbers[0]
        .iter()
        .zip(input_numbers[1].iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum::<u32>();

    format!("Summed distance: {}", summed_distance)
}
