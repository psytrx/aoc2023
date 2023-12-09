use crate::util::parse_space_separated_numbers;

pub fn part_one(input: &str) -> anyhow::Result<String> {
    Ok(solve(input, &extrapolate_forward)?.to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    Ok(solve(input, &extrapolate_backward)?.to_string())
}

fn solve(input: &str, extrapolate: &impl Fn(&[Vec<i32>]) -> i32) -> anyhow::Result<i32> {
    Ok(parse_input(input)?
        .into_iter()
        .map(history)
        .collect::<anyhow::Result<Vec<_>>>()?
        .iter_mut()
        .map(|hist| {
            hist.reverse();
            extrapolate(hist)
        })
        .sum::<i32>())
}

fn extrapolate_forward(hist: &[Vec<i32>]) -> i32 {
    hist[1..].iter().fold(0, |acc, current| {
        let current_val = current[current.len() - 1];
        acc + current_val
    })
}

fn extrapolate_backward(hist: &[Vec<i32>]) -> i32 {
    hist[1..].iter().fold(0, |acc, current| {
        let current_val = current[0];
        current_val - acc
    })
}

fn history(seq: Vec<i32>) -> anyhow::Result<Vec<Vec<i32>>> {
    let mut history = vec![seq];

    let mut contains_non_zeros = true;
    while contains_non_zeros {
        let last = &history[history.len() - 1];

        contains_non_zeros = false;
        history.push(
            last.windows(2)
                .map(|window| match window {
                    &[a, b] => {
                        let diff = b - a;
                        if diff < 0 && !contains_non_zeros {
                            contains_non_zeros = true;
                        }
                        Ok(diff)
                    }
                    _ => {
                        anyhow::bail!(
                            "Failed to parse sequence window, expected 2 elements in window"
                        )
                    }
                })
                .collect::<anyhow::Result<Vec<i32>>>()?,
        );
    }

    Ok(history)
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Vec<i32>>> {
    input
        .lines()
        .map(parse_space_separated_numbers)
        .collect::<anyhow::Result<Vec<_>, _>>()
}
