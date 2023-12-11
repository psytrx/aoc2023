pub fn part_one(input: &str) -> anyhow::Result<String> {
    Ok(solve(parse_input(input), 2).to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    Ok(solve(parse_input(input), 1_000_000).to_string())
}

fn solve(map: Vec<Vec<CellKind>>, expansion_factor: i64) -> i64 {
    let galaxies = flat_expand(map, expansion_factor);
    galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, a)| galaxies.iter().skip(i + 1).map(move |b| (a, b)))
        .map(|(a, b)| {
            let (a_x, a_y) = a.pos;
            let (b_x, b_y) = b.pos;
            let dist = (a_x - b_x).abs() + (a_y - b_y).abs();
            (a.id, b.id, dist)
        })
        .collect::<Vec<_>>()
        .iter()
        .map(|(_, _, d)| d)
        .sum::<i64>()
}

fn flat_expand(map: Vec<Vec<CellKind>>, expansion_factor: i64) -> Vec<Galaxy> {
    let mut col_empty = [true; 140];
    let mut row_empty = [true; 140];

    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if let CellKind::Galaxy(_) = cell {
                col_empty[x] = false;
                row_empty[y] = false;
            }
        }
    }

    let mut galaxies = Vec::with_capacity(512);

    let mut y_offset = 0;
    for (y, row) in map.iter().enumerate() {
        let mut x_offset = 0;
        for (x, col) in row.iter().enumerate() {
            if let CellKind::Galaxy(g) = col {
                galaxies.push(Galaxy {
                    pos: (x as i64 + x_offset, y as i64 + y_offset),
                    ..*g
                });
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

fn parse_input(input: &str) -> Vec<Vec<CellKind>> {
    let mut id = 0;
    input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, cell)| {
                    if cell == '#' {
                        id += 1;
                        CellKind::Galaxy(Galaxy {
                            id,
                            pos: (x as i64, y as i64),
                        })
                    } else {
                        CellKind::Empty
                    }
                })
                .collect()
        })
        .collect()
}

#[derive(Debug)]
enum CellKind {
    Empty,
    Galaxy(Galaxy),
}

#[derive(Debug)]
struct Galaxy {
    id: i64,
    pos: (i64, i64),
}
