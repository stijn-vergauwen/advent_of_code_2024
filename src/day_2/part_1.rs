use super::{parse_input_data, Report};

#[allow(unused)]
pub fn calculate_solution(input_data: String) -> String {
    let mut safety_reports: Vec<Report> = parse_input_data(input_data);

    let safe_reports = safety_reports
        .iter()
        .filter(|report| check_safety_of_report(report))
        .count();

    format!("Safe reports: {}", safe_reports)
}

fn check_safety_of_report(report: &Report) -> bool {
    let has_consistent_direction =
        report.levels.is_sorted_by(|a, b| a < b) || report.levels.is_sorted_by(|a, b| a > b);

    if !has_consistent_direction {
        return false;
    }

    report
        .level_pairs()
        .all(|(previous, current)| previous.abs_diff(current) <= 3)
}
