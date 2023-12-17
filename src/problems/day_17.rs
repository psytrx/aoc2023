pub fn part_one(input: &str) -> anyhow::Result<String> {
    let grid = parse_input(input);
    Ok("not implemented".to_string())
}

pub fn part_two(_input: &str) -> anyhow::Result<String> {
    Ok("not implemented".to_string())
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Vec<i32>>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<i32>().map_err(anyhow::Error::from))
                .collect::<anyhow::Result<_>>()
        })
        .collect::<anyhow::Result<_>>()
}
