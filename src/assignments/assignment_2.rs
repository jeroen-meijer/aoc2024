use std::convert::identity;

use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use super::prelude::*;

pub fn get_assignment() -> Assignment {
    Assignment::new(AssignmentOptions {
        day: 2,
        description: "Day 2",
        run: _run,
        example_input_day_1: Some(
            "
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        ),
        answer_example_day_1: Some(2.into()),
        example_input_day_2: Some(
            "
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        ),
        answer_example_day_2: Some(4.into()),
        answer_day_1: Some(369.into()),
        answer_day_2: Some(428.into()),
    })
}

fn _run(context: AssignmentRuntimeContext) -> Result<Option<Answer>> {
    let data = context
        .data
        .par_iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(str::parse::<u64>)
                .collect::<Result<Vec<u64>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;

    let results = data
        .iter()
        .map(|report| (report, _check_report(report, context.part_number != 1)))
        .collect_vec();

    let valid_report_count = results.iter().filter(|t| t.1).collect_vec().len();

    Ok(Some(valid_report_count.into()))
}

#[derive(PartialEq)]
enum _CountDirection {
    Ascending,
    Descending,
}

impl From<(&u64, &u64)> for _CountDirection {
    fn from(value: (&u64, &u64)) -> Self {
        if value.0 > value.1 {
            _CountDirection::Descending
        } else {
            _CountDirection::Ascending
        }
    }
}

fn _check_report(report: &Vec<u64>, allow_permutations: bool) -> bool {
    const MIN_ABS_LEVEL_INCREASE: u8 = 1;
    const MAX_ABS_LEVEL_INCREASE: u8 = 3;

    fn check_report_permutation(report: Vec<u64>) -> bool {
        let mut current_count_direction: Option<_CountDirection> = None;

        for (a, b) in report.iter().tuple_windows() {
            let this_dir = (a, b).into();
            if let Some(cur_dir) = current_count_direction.as_ref()
                && cur_dir != &this_dir
            {
                return false;
            }

            current_count_direction = Some(this_dir);

            let diff = a.abs_diff(*b);
            if diff < MIN_ABS_LEVEL_INCREASE.into() || diff > MAX_ABS_LEVEL_INCREASE.into() {
                return false;
            }
        }

        return true;
    }

    if !allow_permutations {
        return check_report_permutation(report.to_owned());
    }

    let permutations = (0..report.len()).map(|i| {
        let mut permutation = report.clone();
        permutation.remove(i);
        permutation
    });

    let any_is_valid = permutations.map(check_report_permutation).any(identity);

    any_is_valid
}
