use std::collections::{HashMap, hash_map};

use itertools::Itertools;

use super::prelude::*;

pub fn get_assignment() -> Assignment {
    Assignment::new(AssignmentOptions {
        day: 1,
        description: "Historian Hysteria",
        run: _run,
        example_input_day_1: Some(
            "
3   4
4   3
2   5
1   3
3   9
3   3",
        ),
        answer_example_day_1: Some(11.into()),
        example_input_day_2: Some(
            "
3   4
4   3
2   5
1   3
3   9
3   3",
        ),
        answer_example_day_2: Some(31.into()),
        answer_day_1: Some(2367773.into()),
        answer_day_2: Some(21271939.into()),
    })
}

fn _run(context: AssignmentRuntimeContext) -> Result<Option<Answer>, String> {
    let (mut left, mut right): (Vec<_>, Vec<_>) = context
        .data
        .iter()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .unzip();

    left.sort();
    right.sort();

    if context.part_number == 1 {
        let dist = (0..left.len()).map(|i| (left[i] as i64 - right[i] as i64).abs());
        let answer = dist.sum::<i64>() as u64;

        return Ok(Some(answer.into()));
    }

    let occurrences_right = right.iter().fold(HashMap::new(), |mut map, item| {
        *map.entry(item).or_insert(0) += 1;
        map
    });

    let similarities_left = left
        .iter()
        .map(|i| i * occurrences_right.get(i).unwrap_or(&0));

    let similarity_score = similarities_left.sum::<u32>() as u64;

    Ok(Some(similarity_score.into()))
}
