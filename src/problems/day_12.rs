pub fn part_one(input: &str) -> anyhow::Result<String> {
    let records = parse_input(input)?;
    log::trace!("records: {:#?}", records);

    let temp = records.iter().map(arrangements).collect::<Vec<_>>();
    log::trace!("temp: {:#?}", temp);

    Ok("not implemented".to_string())
}

pub fn part_two(_input: &str) -> anyhow::Result<String> {
    Ok("not implemented".to_string())
}

fn arrangements(r: &Record) -> i32 {
    let mut combinations: Vec<String> = vec!["".to_string()];

    for c in r.pattern.chars() {
        if c == '?' {
            combinations = combinations
                .iter()
                .flat_map(|acc| {
                    let mut a = acc.clone();
                    a.push('.');

                    let mut b = acc.clone();
                    b.push('#');

                    vec![a, b]
                })
                .collect();
        } else {
            combinations = combinations
                .iter_mut()
                .map(|acc| {
                    acc.push(c);
                    acc.to_string()
                })
                .collect();
        }
    }

    combinations
        .iter()
        .filter(|c| count_groups(c) == r.group_counts)
        .count() as i32
}

fn count_groups(pattern: &str) -> Vec<i32> {
    let mut groups = Vec::with_capacity(8);
    let mut current = 0;

    for c in pattern.chars() {
        if c == '#' {
            current += 1;
        } else if current > 0 {
            groups.push(current);
            current = 0;
        }
    }

    if current > 0 {
        groups.push(current);
    }

    groups
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
        group_counts,
    })
}

#[derive(Debug)]
struct Record {
    pattern: String,
    group_counts: Vec<i32>,
}

#[test]
fn test_count_groups() {
    assert_eq!(count_groups("#.#.###"), vec![1, 1, 3]);
    assert_eq!(count_groups(".#...#....###."), vec![1, 1, 3]);
    assert_eq!(count_groups(".#.###.#.######"), vec![1, 3, 1, 6]);
    assert_eq!(count_groups("####.#...#..."), vec![4, 1, 1]);
    assert_eq!(count_groups("#....######..#####."), vec![1, 6, 5]);
    assert_eq!(count_groups(".###.##....#"), vec![3, 2, 1]);
}
