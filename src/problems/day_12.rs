pub fn part_one(input: &str) -> anyhow::Result<String> {
    let sum = parse_input(input)?
        .iter()
        .map(|r| arrangements(&r.pattern, &r.groups))
        .collect::<anyhow::Result<Vec<usize>>>()?
        .iter()
        .sum::<usize>();
    assert_eq!(sum, 21);
    Ok(sum.to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let sum = parse_input(input)?
        .into_iter()
        .map(|r| {
            let chars = std::iter::repeat(r.pattern)
                .take(5)
                .collect::<Vec<_>>()
                .join("?");
            let groups = r.groups.repeat(5);
            arrangements(&chars, &groups)
        })
        .collect::<anyhow::Result<Vec<usize>>>()?
        .iter()
        .sum::<usize>();
    assert_eq!(sum, 525152);
    Ok(sum.to_string())
}

fn arrangements(pattern: &str, groups: &[usize]) -> anyhow::Result<usize> {
    Ok(0)
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

#[derive(Debug)]
struct Record {
    pattern: String,
    groups: Vec<usize>,
}
