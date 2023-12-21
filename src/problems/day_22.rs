pub fn part_one(input: &str) -> anyhow::Result<String> {
    let mut bricks = parse_input(input)?;

    let mut dropped = true;
    while dropped {
        dropped = false;
        bricks = bricks
            .iter()
            .map(|brick| {
                // Idea: If we could sort the bricks by z,
                // we can reduce complexity from O(n^2) to O(n log n)
                let can_drop =
                    brick.z.lo > 1 && !bricks.iter().any(|other| brick.is_supported_by(other));
                if can_drop {
                    dropped = true;
                    // log::debug!("dropping brick: {:?}", brick);

                    let new_z = Range::new(brick.z.lo - 1, brick.z.hi - 1);
                    Brick {
                        id: brick.id.to_owned(),
                        x: brick.x.to_owned(),
                        y: brick.y.to_owned(),
                        z: new_z,
                    }
                } else {
                    brick.to_owned()
                }
            })
            .collect();
    }

    // log::trace!("{:#?}", bricks);

    Ok("not implemented".to_string())
}

pub fn part_two(_input: &str) -> anyhow::Result<String> {
    Ok("not implemented".to_string())
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Brick>> {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let (from, to) = line
                .split_once('~')
                .ok_or_else(|| anyhow::anyhow!("Failed to split line into from/to"))?;

            let (from_x, from_y, from_z) = parse_coordinates(from)?;
            let (to_x, to_y, to_z) = parse_coordinates(to)?;

            let id = ((b'A' + i as u8) as char).to_string();

            Ok(Brick {
                id,
                x: Range::new(from_x, to_x),
                y: Range::new(from_y, to_y),
                z: Range::new(from_z, to_z),
            })
        })
        .collect::<anyhow::Result<_>>()
}

#[derive(Clone)]
struct Brick {
    id: String,
    x: Range,
    y: Range,
    z: Range,
}

impl Brick {
    fn is_supported_by(&self, other: &Self) -> bool {
        self.z.lo == other.z.hi + 1 && self.x.overlaps(&other.x) && self.y.overlaps(&other.y)
    }
}

impl std::fmt::Debug for Brick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{}~{},{},{}   <- {}",
            self.x.lo, self.y.lo, self.z.lo, self.x.hi, self.y.hi, self.z.hi, self.id
        )
    }
}

#[derive(Clone)]
struct Range {
    lo: i32,
    hi: i32,
}

impl Range {
    fn new(lo: i32, hi: i32) -> Self {
        Self { lo, hi }
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.lo <= other.hi && self.hi >= other.lo
    }
}

fn parse_coordinates(s: &str) -> anyhow::Result<(i32, i32, i32)> {
    match s
        .split(',')
        .map(|s| {
            s.parse::<i32>()
                .map_err(|e| anyhow::anyhow!("Failed to parse coordinate '{}': {}", s, e))
        })
        .collect::<anyhow::Result<Vec<_>>>()?[..]
    {
        [x, y, z] => Ok((x, y, z)),
        _ => {
            anyhow::bail!("Invalid length of coordinates, expected {}", 3)
        }
    }
}
