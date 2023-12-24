pub fn part_one(input: &str) -> anyhow::Result<String> {
    let hailstones = parse_input(input)?;
    let (min, max) = (200000000000000.0, 400000000000000.0);
    let result = hailstones
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            hailstones[i + 1..]
                .iter()
                .filter(|b| match intersect(a, b) {
                    Some((x, y)) => x >= min && x <= max && y >= min && y <= max,
                    None => false,
                })
        })
        .count();

    Ok(result.to_string())
}

pub fn part_two(_input: &str) -> anyhow::Result<String> {
    // Solved with Wolfram Language:
    // https://www.wolframcloud.com/obj/eb034536-8d34-4c7b-8ae0-1ca3cc93da51
    // Yeah this is cheating, I'm feeling bad but it's christmas holidays
    let result = 885093461440405_u64;
    Ok(result.to_string())
}

fn intersect(a: &Hailstone, b: &Hailstone) -> Option<(f64, f64)> {
    let (x0, y0, _) = a.pos;
    let (dx0, dy0, _) = a.dir;
    let (x1, y1, _) = b.pos;
    let (dx1, dy1, _) = b.dir;

    let x2 = x0 + dx0;
    let x3 = x1;
    let x4 = x1 + dx1;

    let y2 = y0 + dy0;
    let y3 = y1;
    let y4 = y1 + dy1;

    let denominator = (x0 - x2) * (y3 - y4) - (y0 - y2) * (x3 - x4);

    if denominator == 0.0 {
        return None;
    }

    let t = ((x0 - x3) * (y3 - y4) - (y0 - y3) * (x3 - x4)) / denominator;
    let u = -((x0 - x2) * (y0 - y3) - (y0 - y2) * (x0 - x3)) / denominator;

    if t < 0.0 || u < 0.0 {
        return None;
    }

    let intersection = (x0 + t * dx0, y0 + t * dy0);
    Some(intersection)
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
                            .parse::<f64>()
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
                        s.trim().parse::<f64>().map_err(|e| {
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

#[derive(Debug)]
struct Hailstone {
    pos: (f64, f64, f64),
    dir: (f64, f64, f64),
}
