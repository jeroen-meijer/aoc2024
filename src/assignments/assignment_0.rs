use super::prelude::*;

pub fn get_assignment() -> Assignment {
    Assignment::new(AssignmentOptions {
        day: 0,
        description: "Example: Sum of Numbers",
        run: _run,
        example_input_day_1: Some(
            "
10
20
30
40
50",
        ),
        answer_example_day_1: Some(150.into()),
        example_input_day_2: Some(
            "
5
15
25
35",
        ),
        answer_example_day_2: Some(80.into()),
        answer_day_1: Some(1000.into()),
        answer_day_2: Some(2000.into()),
    })
}

fn _run(context: AssignmentRuntimeContext) -> Result<Option<Answer>, String> {
    // Part 1: Sum all numbers
    // Part 2: Sum all numbers and multiply by 2

    let sum: u64 = context
        .data
        .iter()
        .map(|line| {
            line.parse::<u64>()
                .map_err(|e| format!("Failed to parse '{}': {}", line, e))
        })
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum();

    let result = if context.part_number == 1 {
        sum
    } else {
        // Part 2: multiply by 2
        sum * 2
    };

    Ok(Some(result.into()))
}
