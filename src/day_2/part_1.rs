use super::parse_input_data;

#[allow(unused)]
pub fn calculate_solution(input_data: String) -> String {
    let mut safety_reports: Vec<Vec<u32>> = parse_input_data(input_data);

    let report_results: Vec<bool> = safety_reports
        .iter()
        .map(|report_numbers| check_safety_of_report(report_numbers))
        .collect();

    let safe_reports = report_results.iter().filter(|result| **result).count();

    format!("Safe reports: {}", safe_reports)
}

fn check_safety_of_report(report_numbers: &Vec<u32>) -> bool {
    let has_consistent_direction =
        report_numbers.is_sorted_by(|a, b| a < b) || report_numbers.is_sorted_by(|a, b| a > b);

    if !has_consistent_direction {
        return false;
    }

    let mut previous_number: Option<u32> = None;

    for number in report_numbers.iter() {
        if let Some(previous_number) = previous_number {
            if previous_number.abs_diff(*number) > 3 {
                return false;
            }
        }

        previous_number = Some(*number);
    }

    true
}
