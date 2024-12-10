pub mod part_1;
pub mod part_2;

#[derive(Clone)]
struct Report {
    levels: Vec<u32>,
}

impl Report {
    fn new(levels: Vec<u32>) -> Self {
        Self { levels }
    }

    /// Returns an iterator over each pair of levels, incrementing by 1 per iteration.
    fn level_pairs(&self) -> impl Iterator<Item = (u32, u32)> {
        let length = self.levels.len() - 1;
        let mut pairs = Vec::with_capacity(length);

        for index in 0..length {
            pairs.push((self.levels[index], self.levels[index + 1]));
        }

        pairs.into_iter()
    }
}

fn parse_input_data(input_data: String) -> Vec<Report> {
    let mut safety_reports: Vec<Report> = Vec::with_capacity(1000);

    for line in input_data.lines() {
        let report_levels: Vec<u32> = line
            .split_whitespace()
            .map(|number| {
                number
                    .parse()
                    .expect("Input data should be parsable to u32.")
            })
            .collect();

        safety_reports.push(Report::new(report_levels));
    }

    safety_reports
}
