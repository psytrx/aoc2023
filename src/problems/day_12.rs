use rayon::prelude::{IntoParallelIterator, ParallelIterator};

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let sum = parse_input(input)?
        .into_iter()
        .map(|(pattern, groups)| arrangements(pattern, groups))
        .sum::<usize>();
    Ok(sum.to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let n = 5;
    let sum = parse_input(input)?
        .into_par_iter()
        .map(|(pattern, groups)| {
            let mut repeated_pattern = Vec::with_capacity(pattern.len() * n + n);
            for i in 0..n {
                if i > 0 {
                    repeated_pattern.push(b'?');
                }
                repeated_pattern.extend_from_slice(&pattern);
            }
            arrangements(repeated_pattern, groups.repeat(n))
        })
        .sum::<usize>();
    Ok(sum.to_string())
}

fn arrangements(mut pattern: Vec<u8>, groups: Vec<usize>) -> usize {
    // Allows us to skip bounds check
    pattern.push(b'.');

    // Allows us to check _ranges_ for placing springs
    // without checking each element in between.
    let broken_acc = {
        let mut acc = Vec::with_capacity(pattern.len() + 1);
        acc.push(0);

        let mut sum = 0;
        for &c in pattern.iter() {
            if c != b'.' {
                sum += 1;
            }
            acc.push(sum);
        }
        acc
    };

    // Calculate padding for shifting the pattern to the right
    let padding = pattern.len() - groups.iter().sum::<usize>() - groups.len() + 1;

    // Run a prefix sum for each group over the pattern.
    // If we can carry a positive sum to the end, we found all possible arrangements.
    // let mut table = vec![0; pattern.len() * groups.len()];
    let mut table = vec![vec![0; pattern.len()]; groups.len()];
    let mut arrangements = 0;

    let group_size = groups[0];
    let mut valid = true;
    for i in 0..padding {
        let has_trailing_pound = pattern[i + group_size] == b'#';
        if has_trailing_pound {
            // Pattern enforces a larger group than allowed
            arrangements = 0
        } else if valid {
            let spring_can_fit = broken_acc[i + group_size] - broken_acc[i] == group_size;
            if spring_can_fit {
                arrangements += 1;
            }
        }

        table[0][group_size + i] = arrangements;

        if pattern[i] == b'#' {
            // Pattern enforces a spring _before_ the current group: not a valid arrangement.
            valid = false
        }
    }

    let mut lo = group_size + 1;
    for (row, &group_size) in groups.iter().enumerate().skip(1) {
        arrangements = 0;

        let hi = lo + padding;
        for i in lo..hi {
            let has_trailing_pound = pattern[i + group_size] == b'#';
            if has_trailing_pound {
                arrangements = 0;
            } else if pattern[i - 1] != b'#'
                && broken_acc[i + group_size] - broken_acc[i] == group_size
            {
                let spring_can_fit = broken_acc[i + group_size] - broken_acc[i] == group_size;
                if spring_can_fit {
                    arrangements += table[row - 1][i - 1];
                }
            }

            table[row][i + group_size] = arrangements;
        }

        lo += group_size + 1;
    }

    arrangements
}

fn parse_input(input: &str) -> anyhow::Result<Vec<(Vec<u8>, Vec<usize>)>> {
    input
        .lines()
        .map(|line| {
            let (pattern, group_counts) = line
                .split_once(' ')
                .ok_or_else(|| anyhow::anyhow!("Failed to split line"))?;

            let groups = group_counts
                .split(',')
                .map(|s| {
                    s.parse::<usize>()
                        .map_err(|e| anyhow::anyhow!("Failed to parse grou counts: {}", e))
                })
                .collect::<anyhow::Result<_>>()?;

            Ok((pattern.as_bytes().to_vec(), groups))
        })
        .collect::<anyhow::Result<_>>()
}
