use super::parse_input_data;

#[allow(unused)]
pub fn calculate_solution(input_data: String) -> String {
    let mut input_numbers: [Vec<u32>; 2] = parse_input_data(input_data);

    let similarity_score = input_numbers[0]
        .iter()
        .map(|left_number| {
            let occurrences = input_numbers[1]
                .iter()
                .filter(|right_number| *left_number == **right_number)
                .count() as u32;

            left_number * occurrences
        })
        .sum::<u32>();

    format!("Similarity score: {}", similarity_score)
}
