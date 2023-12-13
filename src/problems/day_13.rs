pub fn part_one(input: &str) -> anyhow::Result<String> {
    let input = parse_input(input)?;

    let reflections = input
        .iter()
        .map(|pattern| find_horizontal_reflections(pattern))
        .collect::<Vec<_>>();
    log::trace!("{:#?}", reflections);

    Ok("not implemented".to_string())
}

pub fn part_two(_input: &str) -> anyhow::Result<String> {
    Ok("not implemented".to_string())
}

fn find_horizontal_reflections(pattern: &[String]) -> Vec<usize> {
    let mut mirrors = vec![];

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
            mirrors.push(mirror_y);
        }
    }

    mirrors
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
