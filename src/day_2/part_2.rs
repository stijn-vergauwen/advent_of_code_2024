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
    count_unsafe_report_levels(report) <= 1
}

fn count_unsafe_report_levels(report: &Report) -> u32 {
    let report_sign = get_sign_of_report(report);
    let mut unsafe_levels = 0;

    for (previous_level, current_level) in report.level_pairs() {
        let level_sign = get_sign_of_level(previous_level, current_level);
        let difference = previous_level.abs_diff(current_level);

        if level_sign != report_sign {
            unsafe_levels += 1;
        }

        if difference < 1 || difference > 3 {
            unsafe_levels += 1;
        }
    }

    unsafe_levels
}

fn get_sign_of_report(report: &Report) -> i32 {
    report
        .level_pairs()
        .map(|(previous_level, current_level)| get_sign_of_level(previous_level, current_level))
        .sum::<i32>()
        .signum()
}

fn get_sign_of_level(previous_level: u32, current_level: u32) -> i32 {
    (current_level as i32 - previous_level as i32).signum()
}
