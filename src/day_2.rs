pub mod part_1;
pub mod part_2;

fn parse_input_data(input_data: String) -> Vec<Vec<u32>> {
    let mut safety_reports: Vec<Vec<u32>> = Vec::with_capacity(1000);

    for line in input_data.lines() {
        let numbers_in_report: Vec<u32> = line
            .split_whitespace()
            .map(|number| {
                number
                    .parse()
                    .expect("Input data should be parsable to u32.")
            })
            .collect();

        safety_reports.push(numbers_in_report);
    }

    safety_reports
}