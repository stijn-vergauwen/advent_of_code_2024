pub mod part_1;
pub mod part_2;

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