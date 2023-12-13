pub fn part_one(input: &str) -> anyhow::Result<String> {
    Ok(parse_input(input)?
        .iter()
        .map(|pattern| {
            find_reflection(pattern)
                .map(|r| match r {
                    Reflection::Horizontal(v) => 100 * v,
                    Reflection::Vertical(v) => v,
                })
                .ok_or_else(|| anyhow::anyhow!("Failed to find reflection for pattern"))
        })
        .collect::<anyhow::Result<Vec<_>>>()?
        .iter()
        .sum::<usize>()
        .to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    Ok(parse_input(input)?
        .iter()
        .map(|pattern| {
            find_reflection(pattern)
                .map(|r| {
                    // foo
                    1
                })
                .ok_or_else(|| anyhow::anyhow!("Failed to find reflection for pattern"))
        })
        .collect::<anyhow::Result<Vec<_>>>()?
        .iter()
        .sum::<usize>()
        .to_string())
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

fn find_reflection(pattern: &[String]) -> Option<Reflection> {
    find_horizontal_reflection(pattern).or_else(|| {
        let transposed = transpose(pattern);
        find_horizontal_reflection(&transposed).map(|r| r.into_rotated())
    })
}

fn find_horizontal_reflection(pattern: &[String]) -> Option<Reflection> {
    for mirror_y in 1..pattern.len() {
        let height = mirror_y.min(pattern.len() - mirror_y);
        let mut is_mirrored = true;

        for offset_y in 0..height {
            let a = &pattern[mirror_y + offset_y];
            let b = &pattern[mirror_y - offset_y - 1];

            let equal = a.chars().zip(b.chars()).all(|(c_a, c_b)| c_a == c_b);
            if !equal {
                is_mirrored = false;
                break;
            }
        }

        if is_mirrored {
            return Some(Reflection::Horizontal(mirror_y));
        }
    }

    None
}

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
