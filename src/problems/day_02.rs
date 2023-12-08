pub fn part_one(input: &str) -> anyhow::Result<String> {
    Ok(parse_input(input)?
        .iter()
        .filter(|game| {
            game.subsets
                .iter()
                .all(|subset| subset.red <= 12 && subset.green <= 13 && subset.blue <= 14)
        })
        .map(|game| game.id)
        .sum::<u32>()
        .to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    Ok(parse_input(input)?
        .iter()
        .map(|game| {
            game.subsets
                .iter()
                .fold(CubeSubset::EMPTY, |acc, subset| acc.max(subset))
                .power()
        })
        .sum::<u32>()
        .to_string())
}

fn parse_input(input: &str) -> anyhow::Result<Vec<GameRecord>> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> anyhow::Result<GameRecord> {
    let (id, subsets) = line
        .split_once(": ")
        .ok_or_else(|| anyhow::anyhow!("Failed to split input line: '{}'", line))?;

    let id = {
        id.strip_prefix("Game ")
            .ok_or_else(|| anyhow::anyhow!("Failed to parse game id: '{}'", id))?
            .parse::<u32>()?
    };

    let subsets = subsets
        .split("; ")
        .map(parse_subset)
        .collect::<anyhow::Result<Vec<_>>>()?;

    Ok(GameRecord { id, subsets })
}

fn parse_subset(subset: &str) -> Result<CubeSubset, anyhow::Error> {
    subset
        .split(", ")
        .try_fold(CubeSubset::EMPTY, |acc, cubes| {
            let (count, color) = cubes
                .split_once(' ')
                .ok_or_else(|| anyhow::anyhow!("Failed to split cube: '{}'", cubes))?;
            let count = count.parse::<u32>()?;

            match color {
                "red" => Ok(CubeSubset { red: count, ..acc }),
                "green" => Ok(CubeSubset {
                    green: count,
                    ..acc
                }),
                "blue" => Ok(CubeSubset { blue: count, ..acc }),
                _ => anyhow::bail!("Failed to parse cube color: '{}'", color),
            }
        })
}

struct GameRecord {
    id: u32,
    subsets: Vec<CubeSubset>,
}

struct CubeSubset {
    red: u32,
    green: u32,
    blue: u32,
}

impl CubeSubset {
    const EMPTY: Self = Self {
        red: 0,
        green: 0,
        blue: 0,
    };

    fn max(&self, other: &CubeSubset) -> CubeSubset {
        CubeSubset {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}
