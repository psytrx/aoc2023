use crate::util::{parse_space_separated_numbers, solve_quadratic_equation, QuadraticSolution};

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let (times, distances) = input
        .split_once('\n')
        .ok_or_else(|| anyhow::anyhow!("Failed to split input lines"))?;

    let times = parse_space_separated_numbers::<u64>(
        times
            .strip_prefix("Time: ")
            .ok_or_else(|| anyhow::anyhow!("Failed to strip time prefix"))?,
    )?;
    let distances = parse_space_separated_numbers::<u64>(
        distances
            .strip_prefix("Distance: ")
            .ok_or_else(|| anyhow::anyhow!("Failed to strip distance prefix"))?,
    )?;

    Ok(times
        .iter()
        .zip(distances)
        .map(|(&time, distance)| number_of_ways(time, distance))
        .collect::<anyhow::Result<Vec<u64>>>()?
        .iter()
        .product::<u64>()
        .to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let (time, distance) = input
        .split_once('\n')
        .ok_or_else(|| anyhow::anyhow!("Failed to split input lines"))?;

    let time = time
        .strip_prefix("Time: ")
        .ok_or_else(|| anyhow::anyhow!("Failed to strip time prefix"))?
        .replace(' ', "")
        .trim()
        .parse::<u64>()?;

    let distance = distance
        .strip_prefix("Distance: ")
        .ok_or_else(|| anyhow::anyhow!("Failed to strip distance prefix"))?
        .replace(' ', "")
        .trim()
        .parse::<u64>()?;

    Ok(number_of_ways(time, distance)?.to_string())
}

fn number_of_ways(total_time: u64, distance_to_beat: u64) -> anyhow::Result<u64> {
    let (lo, hi) =
        match solve_quadratic_equation(-1.0, total_time as f64, -(distance_to_beat as f64)) {
            QuadraticSolution::None => anyhow::bail!("Failed to find bounds"),
            QuadraticSolution::OneRoot(m) => (m.ceil() as u64, m.floor() as u64),
            QuadraticSolution::TwoRoots(lo, hi) => (lo.ceil() as u64, hi.floor() as u64),
        };
    Ok(hi - lo + 1)
}
