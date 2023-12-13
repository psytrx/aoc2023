pub fn part_one(input: &str) -> anyhow::Result<String> {
    Ok(solve(parse_input(input)?, 0)?.to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    Ok(solve(parse_input(input)?, 1)?.to_string())
}

fn solve(patterns: Vec<Vec<String>>, required_differences: usize) -> anyhow::Result<usize> {
    Ok(patterns
        .iter()
        .map(|pattern| {
            find_reflection(pattern, required_differences)
                .map(|r| match r {
                    Reflection::Horizontal(v) => 100 * v,
                    Reflection::Vertical(v) => v,
                })
                .ok_or_else(|| anyhow::anyhow!("Failed to find reflection for pattern"))
        })
        .collect::<anyhow::Result<Vec<_>>>()?
        .iter()
        .sum::<usize>())
}

fn transpose(pattern: &[String]) -> Vec<String> {
    let mut transposed = vec![String::with_capacity(pattern.len()); pattern[0].len()];
    for s in pattern {
        for (i, c) in s.chars().enumerate() {
            transposed[i].push(c);
        }
    }
    transposed
}

fn find_reflection(pattern: &[String], required_differences: usize) -> Option<Reflection> {
    find_horizontal_reflection(pattern, required_differences).or_else(|| {
        let transposed = transpose(pattern);
        find_horizontal_reflection(&transposed, required_differences).map(|r| r.into_rotated())
    })
}

fn find_horizontal_reflection(
    pattern: &[String],
    required_differences: usize,
) -> Option<Reflection> {
    for mirror_y in 1..pattern.len() {
        let height = mirror_y.min(pattern.len() - mirror_y);
        let mut differences = 0;
        for offset_y in 0..height {
            let a = &pattern[mirror_y + offset_y];
            let b = &pattern[mirror_y - offset_y - 1];

            for (a, b) in a.chars().zip(b.chars()) {
                if a != b {
                    differences += 1;
                    if differences > required_differences {
                        break;
                    }
                }
            }

            if differences > required_differences {
                break;
            }
        }

        if differences == required_differences {
            return Some(Reflection::Horizontal(mirror_y));
        }
    }

    None
}

#[derive(Debug)]
enum Reflection {
    Horizontal(usize),
    Vertical(usize),
}

impl Reflection {
    fn into_rotated(self) -> Reflection {
        match self {
            Reflection::Horizontal(v) => Reflection::Vertical(v),
            Reflection::Vertical(v) => Reflection::Horizontal(v),
        }
    }
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Vec<String>>> {
    input.lines().try_fold(vec![vec![]], |mut acc, line| {
        if line.is_empty() {
            acc.push(vec![]);
            Ok(acc)
        } else {
            let last = acc
                .last_mut()
                .ok_or_else(|| anyhow::anyhow!("Failed to get last row in acc"))?;
            last.push(line.to_string());
            Ok(acc)
        }
    })
}
