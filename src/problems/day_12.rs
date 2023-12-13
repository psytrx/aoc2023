use memoize::memoize;
use rayon::prelude::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let sum = parse_input(input)?
        .into_par_iter()
        .map(arrangements)
        .sum::<usize>();
    Ok(sum.to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let sum = parse_input(input)?
        .into_par_iter()
        .map(|r| {
            let pattern = std::iter::repeat(r.pattern)
                .take(5)
                .collect::<Vec<_>>()
                .join("?");
            let groups = r.groups.repeat(5);
            arrangements(Record { pattern, groups })
        })
        .sum::<usize>();
    Ok(sum.to_string())
}

#[memoize]
fn arrangements(rec: Record) -> usize {
    if rec.pattern.is_empty() && rec.groups.is_empty() {
        1
    } else if rec.pattern.is_empty() && !rec.groups.is_empty()
        || rec.groups.is_empty() && rec.pattern.contains('#')
    {
        0
    } else if let Some(stripped) = rec.pattern.strip_prefix('.') {
        arrangements(Record {
            pattern: stripped.to_string(),
            groups: rec.groups,
        })
    } else if rec.pattern.starts_with('#') {
        let expected_group = rec.groups[0];

        if rec.pattern.len() < expected_group {
            0
        } else {
            let prefix = &rec.pattern[..expected_group];
            if prefix.contains('.') {
                0
            } else if rec.pattern.len() == expected_group {
                if rec.groups.len() == 1 {
                    1
                } else {
                    0
                }
            } else {
                let c = rec.pattern.as_bytes()[expected_group];
                if c == b'.' || c == b'?' {
                    let rec_suffix = rec.pattern[expected_group + 1..].to_string();
                    let group_suffix = rec.groups[1..].to_vec();
                    arrangements(Record {
                        pattern: rec_suffix,
                        groups: group_suffix,
                    })
                } else {
                    0
                }
            }
        }
    } else if rec.pattern.starts_with('?') {
        // branch out into both possibilities
        let dot = Record {
            pattern: ".".to_string() + &rec.pattern[1..],
            groups: rec.groups.clone(),
        };
        let pound = Record {
            pattern: "#".to_string() + &rec.pattern[1..],
            groups: rec.groups.clone(),
        };
        arrangements(dot) + arrangements(pound)
    } else {
        unreachable!()
    }
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Record>> {
    input
        .lines()
        .map(parse_line)
        .collect::<anyhow::Result<Vec<Record>>>()
}

fn parse_line(line: &str) -> anyhow::Result<Record> {
    let (pattern, group_counts) = line
        .split_once(' ')
        .ok_or_else(|| anyhow::anyhow!("Failed to split line"))?;

    let group_counts = group_counts
        .split(',')
        .map(|s| {
            s.parse::<usize>()
                .map_err(|e| anyhow::anyhow!("Failed to parse grou counts: {}", e))
        })
        .collect::<anyhow::Result<Vec<usize>>>()?;

    Ok(Record {
        pattern: pattern.to_string(),
        groups: group_counts,
    })
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Record {
    pattern: String,
    groups: Vec<usize>,
}
