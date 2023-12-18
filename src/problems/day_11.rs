pub fn part_one(input: &str) -> anyhow::Result<String> {
    Ok(solve(parse_input(input), 2).to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    Ok(solve(parse_input(input), 1_000_000).to_string())
}

fn solve(map: Vec<Vec<Option<(i64, i64)>>>, expansion_factor: i64) -> i64 {
    let galaxies = flat_expand(map, expansion_factor);
    galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, (a_x, a_y))| {
            galaxies
                .iter()
                .skip(i + 1)
                .map(move |(b_x, b_y)| (a_x - b_x).abs() + (a_y - b_y).abs())
        })
        .sum()
}

fn flat_expand(map: Vec<Vec<Option<(i64, i64)>>>, expansion_factor: i64) -> Vec<(i64, i64)> {
    let mut col_empty = [true; 140];
    let mut row_empty = [true; 140];

    for row in map.iter() {
        for cell in row.iter() {
            if let &Some((x, y)) = cell {
                col_empty[x as usize] = false;
                row_empty[y as usize] = false;
            }
        }
    }

    let mut galaxies = Vec::with_capacity(512);

    let mut y_offset = 0;
    for (y, row) in map.iter().enumerate() {
        let mut x_offset = 0;
        for (x, col) in row.iter().enumerate() {
            if let Some((x, y)) = col {
                galaxies.push((x + x_offset, y + y_offset));
            }

            if col_empty[x] {
                x_offset += expansion_factor - 1;
            }
        }

        if row_empty[y] {
            y_offset += expansion_factor - 1;
        }
    }

    galaxies
}

fn parse_input(input: &str) -> Vec<Vec<Option<(i64, i64)>>> {
    let mut id = 0;
    input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(|(x, &cell)| {
                    if cell == b'#' {
                        id += 1;
                        Some((x as i64, y as i64))
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect()
}
