pub fn part_one(input: &str) -> anyhow::Result<String> {
    Ok(parse_input(input)?
        .iter()
        .map(|r| {
            let chars = r.pattern.chars().collect::<Vec<_>>();
            arrangements(&chars, &r.groups, 0)
        })
        .sum::<i32>()
        .to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    Ok("".to_string())
}

fn arrangements(pattern: &[char], groups: &[i32], group_size: i32) -> i32 {
    match (pattern, groups) {
        ([], []) => {
            if group_size == 0 {
                1
            } else {
                0
            }
        }
        ([], [expected, rest_groups @ ..]) => {
            if &group_size == expected {
                arrangements(pattern, rest_groups, 0)
            } else {
                0
            }
        }
        (['.', rest_pat @ ..], []) => {
            if group_size == 0 {
                arrangements(rest_pat, groups, 0)
            } else {
                0
            }
        }
        (['.', rest_pat @ ..], [expected, rest_groups @ ..]) => {
            if group_size == 0 {
                arrangements(rest_pat, groups, 0)
            } else if &group_size == expected {
                arrangements(rest_pat, rest_groups, 0)
            } else {
                0
            }
        }
        (['?', rest_pat @ ..], _) => {
            arrangements(&[&['.'], rest_pat].concat(), groups, group_size)
                + arrangements(&[&['#'], rest_pat].concat(), groups, group_size)
        }
        (['#', rest_pat @ ..], _) => arrangements(rest_pat, groups, group_size + 1),

        _ => unreachable!("unreachable pattern: {:?}, groups: {:?}", pattern, groups),
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
            s.parse::<i32>()
                .map_err(|e| anyhow::anyhow!("Failed to parse grou counts: {}", e))
        })
        .collect::<anyhow::Result<Vec<i32>>>()?;

    Ok(Record {
        pattern: pattern.to_string(),
        groups: group_counts,
    })
}

#[derive(Debug)]
struct Record {
    pattern: String,
    groups: Vec<i32>,
}
