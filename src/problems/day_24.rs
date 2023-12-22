pub fn part_one(input: &str) -> anyhow::Result<String> {
    let hailstones = parse_input(input)?;
    Ok("not implemented".to_string())
}

pub fn part_two(_input: &str) -> anyhow::Result<String> {
    Ok("not implemented".to_string())
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Hailstone>> {
    input
        .lines()
        .map(|line| {
            let (pos, dir) = line
                .split_once(" @ ")
                .ok_or_else(|| anyhow::anyhow!("Failed to split hailstone at @"))?;

            let pos = {
                let pos = pos
                    .split(", ")
                    .map(|s| {
                        s.trim()
                            .parse::<i64>()
                            .map_err(|e| anyhow::anyhow!("Failed to parse position '{}': {}", s, e))
                    })
                    .collect::<anyhow::Result<Vec<_>>>()?;
                match pos[..] {
                    [x, y, z] => (x, y, z),
                    _ => anyhow::bail!("Failed to parse hailstone position"),
                }
            };

            let dir = {
                let dir = dir
                    .split(", ")
                    .map(|s| {
                        s.trim().parse::<i64>().map_err(|e| {
                            anyhow::anyhow!("Failed to parse direction '{}': {}", s, e)
                        })
                    })
                    .collect::<anyhow::Result<Vec<_>>>()?;
                match dir[..] {
                    [x, y, z] => (x, y, z),
                    _ => anyhow::bail!("Failed to parse hailstone direction"),
                }
            };

            Ok(Hailstone { pos, dir })
        })
        .collect::<anyhow::Result<_>>()
}

struct Hailstone {
    pos: (i64, i64, i64),
    dir: (i64, i64, i64),
}
