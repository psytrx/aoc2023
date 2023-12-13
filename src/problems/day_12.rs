use memoize::memoize;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let sum = parse_input(input)?
        .into_par_iter()
        .map(|(pattern, groups)| arrangements(pattern, groups))
        .sum::<usize>();
    Ok(sum.to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let sum = parse_input(input)?
        .into_par_iter()
        .map(|(pattern, groups)| {
            let pattern = std::iter::repeat(pattern)
                .take(5)
                .collect::<Vec<_>>()
                .join("?");
            let groups = groups.repeat(5);
            arrangements(pattern, groups)
        })
        .sum::<usize>();
    Ok(sum.to_string())
}

#[memoize]
fn arrangements(pattern: String, groups: Vec<usize>) -> usize {
    if pattern.is_empty() && groups.is_empty() {
        1
    } else if pattern.is_empty() && !groups.is_empty() || groups.is_empty() && pattern.contains('#')
    {
        0
    } else if let Some(stripped) = pattern.strip_prefix('.') {
        let pattern = stripped.to_string();
        arrangements(pattern, groups)
    } else if pattern.starts_with('#') {
        let expected_group = groups[0];

        if pattern.len() < expected_group {
            0
        } else {
            let prefix = &pattern[..expected_group];
            if prefix.contains('.') {
                0
            } else if pattern.len() == expected_group {
                if groups.len() == 1 {
                    1
                } else {
                    0
                }
            } else {
                let c = pattern.as_bytes()[expected_group];
                if c == b'.' || c == b'?' {
                    let pattern = pattern[expected_group + 1..].to_string();
                    let groups = groups[1..].to_vec();
                    arrangements(pattern, groups)
                } else {
                    0
                }
            }
        }
    } else if let Some(pat_suffix) = pattern.strip_prefix('?') {
        let dot = {
            let pattern = ".".to_string() + pat_suffix;
            let groups = groups.clone();
            arrangements(pattern, groups)
        };
        let pound = {
            let pattern = "#".to_string() + pat_suffix;
            arrangements(pattern, groups)
        };
        dot + pound
    } else {
        unreachable!()
    }
}

fn parse_input(input: &str) -> anyhow::Result<Vec<(String, Vec<usize>)>> {
    input
        .lines()
        .map(|line| {
            let (pattern, group_counts) = line
                .split_once(' ')
                .ok_or_else(|| anyhow::anyhow!("Failed to split line"))?;

            let groups = group_counts
                .split(',')
                .map(|s| {
                    s.parse::<usize>()
                        .map_err(|e| anyhow::anyhow!("Failed to parse grou counts: {}", e))
                })
                .collect::<anyhow::Result<Vec<usize>>>()?;

            Ok((pattern.to_string(), groups))
        })
        .collect::<anyhow::Result<Vec<(String, Vec<usize>)>>>()
}
