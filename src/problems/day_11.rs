pub fn part_one(input: &str) -> anyhow::Result<String> {
    let map = parse_input(input);
    let galaxies = flat_expand(map);

    let galaxy_pairs = galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, a)| galaxies.iter().skip(i + 1).map(move |b| (a, b)));

    let distances = galaxy_pairs
        .map(|(a, b)| {
            let (a_x, a_y) = a.pos;
            let (b_x, b_y) = b.pos;
            let dist = (a_x - b_x).abs() + (a_y - b_y).abs();
            (a.id, b.id, dist)
        })
        .collect::<Vec<_>>();

    let sum = distances.iter().map(|(_, _, d)| d).sum::<i32>();

    Ok(sum.to_string())
}

pub fn part_two(_input: &str) -> anyhow::Result<String> {
    Ok("not implemented".to_string())
}

fn flat_expand(map: Vec<Vec<CellKind>>) -> Vec<Galaxy> {
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
                    pos: (x as i32 + x_offset, y as i32 + y_offset),
                    ..*g
                });
            }

            if col_empty[x] {
                x_offset += 1;
            }
        }

        if row_empty[y] {
            y_offset += 1
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
                            pos: (x as i32, y as i32),
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
    id: i32,
    pos: (i32, i32),
}
