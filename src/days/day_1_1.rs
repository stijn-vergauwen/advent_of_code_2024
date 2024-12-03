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

fn parse_input_data(input_data: String) -> [Vec<u32>; 2] {
    let mut number_arrays = [Vec::with_capacity(1000), Vec::with_capacity(1000)];

    for line in input_data.lines() {
        let line_segments: Vec<u32> = line
            .split_whitespace()
            .map(|segment| {
                segment
                    .parse()
                    .expect("Input data should be parsable to u32.")
            })
            .collect();

        assert_eq!(2, line_segments.len());

        number_arrays[0].push(line_segments[0]);
        number_arrays[1].push(line_segments[1]);
    }

    number_arrays
}
