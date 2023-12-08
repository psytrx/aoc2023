pub fn part_one(input: &str) -> anyhow::Result<String> {
    Ok(solve(
        input,
        vec![
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
        ],
    )?
    .to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    Ok(solve(
        input,
        vec![
            ("1", 1),
            ("one", 1),
            ("2", 2),
            ("two", 2),
            ("3", 3),
            ("three", 3),
            ("4", 4),
            ("four", 4),
            ("5", 5),
            ("five", 5),
            ("6", 6),
            ("six", 6),
            ("7", 7),
            ("seven", 7),
            ("8", 8),
            ("eight", 8),
            ("9", 9),
            ("nine", 9),
        ],
    )?
    .to_string())
}

fn solve(input: &str, patterns: Vec<(&str, i32)>) -> anyhow::Result<i32> {
    // let ac = aho_corasick::AhoCorasick::new(patterns.keys())?;
    let ac = daachorse::DoubleArrayAhoCorasick::with_values(patterns)
        .map_err(|e| anyhow::anyhow!("Failed to build aho-corasick: {}", e))?;

    Ok(input
        .lines()
        .map(|line| {
            let mut matches = ac.find_overlapping_iter(line);

            let first_digit = matches
                .next()
                .map(|m| m.value())
                .ok_or_else(|| anyhow::anyhow!("Failed to find first item"))?;

            let last_digit = matches.last().map(|m| m.value()).unwrap_or(first_digit);

            Ok(10 * first_digit + last_digit)
        })
        .collect::<anyhow::Result<Vec<i32>>>()?
        .iter()
        .sum())
}
