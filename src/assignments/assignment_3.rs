use anyhow::Context;
use itertools::Itertools;
use regex::Regex;

use super::prelude::*;

pub fn get_assignment() -> Assignment {
    Assignment::new(AssignmentOptions {
        day: 3,
        description: "Day 3",
        run: _run,
        example_input_day_1: Some(
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
        ),
        answer_example_day_1: Some(161.into()),
        example_input_day_2: Some(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        ),
        answer_example_day_2: Some(48.into()),
        answer_day_1: Some(159892596.into()),
        answer_day_2: Some(92626942.into()),
    })
}

struct _Match {
    index: usize,
    payload: _MatchType,
}

enum _MatchType {
    Mul(u64, u64),
    Enable,
    Disable,
}

fn _run(context: AssignmentRuntimeContext) -> Result<Option<Answer>> {
    let regex_mul = Regex::new(r"mul\((\d+),(\d+)\)")?;
    let regex_enable = Regex::new(r"do\(\)")?;
    let regex_disable = Regex::new(r"don't\(\)")?;

    let is_toggling_enabled = context.part_number == 2;

    let full_data = context.data.into_iter().join(" ");

    let raw_multiplications = regex_mul
        .captures_iter(&full_data)
        .map(|c| {
            let index = c.get(0).unwrap().start();
            // Index 0 matches the full string, while the remaining indices contain the groups
            vec![1, 2]
                .into_iter()
                .map(|i| c.get(i as usize).unwrap().as_str())
                .map(|m| {
                    m.parse::<u64>()
                        .context(format!("while parsing \"{}\" into a u64", m))
                        .unwrap()
                })
                .collect_tuple()
                .map(|(a, b)| (index, (a, b)))
                .unwrap()
        })
        .collect_vec();

    if !is_toggling_enabled {
        let result = raw_multiplications
            .into_iter()
            .map(|(_, (a, b))| a * b)
            .sum::<u64>();
        return Ok(Some(result.into()));
    }

    let enables = regex_enable
        .captures_iter(&full_data)
        .map(|c| _Match {
            index: c.get(0).unwrap().start(),
            payload: _MatchType::Enable,
        })
        .collect_vec();

    let disables = regex_disable
        .captures_iter(&full_data)
        .map(|c| _Match {
            index: c.get(0).unwrap().start(),
            payload: _MatchType::Disable,
        })
        .collect_vec();

    let mut all_instructions = raw_multiplications
        .into_iter()
        .map(|(index, (a, b))| _Match {
            index,
            payload: _MatchType::Mul(a, b),
        })
        .collect_vec();

    all_instructions.extend(enables);
    all_instructions.extend(disables);

    all_instructions.sort_by_key(|m| m.index);

    let payloads = all_instructions.into_iter().map(|i| i.payload);

    let mut is_multiplying_enabled = true;

    let mut total = 0u64;

    for payload in payloads {
        match payload {
            _MatchType::Mul(a, b) => {
                if !is_multiplying_enabled {
                    continue;
                }

                total += a * b
            }
            _MatchType::Enable => is_multiplying_enabled = true,
            _MatchType::Disable => is_multiplying_enabled = false,
        }
    }

    Ok(Some(total.into()))
}
