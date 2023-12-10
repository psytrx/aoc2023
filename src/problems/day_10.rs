pub fn part_one(input: &str) -> anyhow::Result<String> {
    let map = parse_input(input)?;

    let (mut x, mut y) = map.start;
    loop {
        (x, y) = follow_pipe(&map, (x, y))?;

        if (x, y) == map.start {
            break;
        }
    }

    Ok("not implemented".to_string())
}

pub fn part_two(_input: &str) -> anyhow::Result<String> {
    Ok("not implemented".to_string())
}

fn follow_pipe(map: &PipeMap, (x, y): (usize, usize)) -> anyhow::Result<(usize, usize)> {
    match map.tiles[y][x].kind {
        TileKind::Ground => anyhow::bail!("Cannot follow ground tile"),
        TileKind::Start => {
            let n = connected_neighbours(map, (x, y));
            log::warn!("neighbors: {:?}", n);
        }
        TileKind::Pipe(_) => {}
    }
    Ok((x, y))
}

fn connected_neighbours(map: &PipeMap, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::with_capacity(4);

    if y > 0 {
        let top = &map.tiles[y - 1][x];
        match top.kind {
            TileKind::Start
            | TileKind::Pipe(PipeKind::Vertical)
            | TileKind::Pipe(PipeKind::SouthWest)
            | TileKind::Pipe(PipeKind::SouthEast) => {
                neighbors.push((x, y - 1));
            }
            _ => {}
        }
    }

    if y < map.tiles.len() - 1 {
        let bottom = &map.tiles[y + 1][x];
        match bottom.kind {
            TileKind::Start
            | TileKind::Pipe(PipeKind::Vertical)
            | TileKind::Pipe(PipeKind::NorthWest)
            | TileKind::Pipe(PipeKind::NorthEast) => {
                neighbors.push((x, y + 1));
            }
            _ => {}
        }
    }

    if x > 0 {
        let left = &map.tiles[y][x - 1];
        match left.kind {
            TileKind::Start
            | TileKind::Pipe(PipeKind::Horizontal)
            | TileKind::Pipe(PipeKind::NorthEast)
            | TileKind::Pipe(PipeKind::SouthEast) => {
                neighbors.push((x - 1, y));
            }
            _ => {}
        }
    }

    if x < map.tiles[0].len() - 1 {
        let right = &map.tiles[y][x + 1];
        match right.kind {
            TileKind::Start
            | TileKind::Pipe(PipeKind::Horizontal)
            | TileKind::Pipe(PipeKind::NorthWest)
            | TileKind::Pipe(PipeKind::SouthWest) => {
                neighbors.push((x + 1, y));
            }
            _ => {}
        }
    }

    neighbors
}

struct PipeMap {
    tiles: Vec<Vec<Tile>>,
    start: (usize, usize),
}

struct Tile {
    kind: TileKind,
    visited: bool,
}

fn parse_input(input: &str) -> anyhow::Result<PipeMap> {
    let mut start = None;

    let map = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let kind = match c {
                        '.' => TileKind::Ground,
                        'S' => {
                            start = Some((x, y));
                            TileKind::Start
                        }
                        '|' => TileKind::Pipe(PipeKind::Vertical),
                        '-' => TileKind::Pipe(PipeKind::Horizontal),
                        'L' => TileKind::Pipe(PipeKind::NorthEast),
                        'J' => TileKind::Pipe(PipeKind::NorthWest),
                        '7' => TileKind::Pipe(PipeKind::SouthWest),
                        'F' => TileKind::Pipe(PipeKind::SouthEast),
                        _ => anyhow::bail!("Failed to parse tile: {}", c),
                    };
                    Ok(Tile {
                        kind,
                        visited: false,
                    })
                })
                .collect::<anyhow::Result<_>>()
        })
        .collect::<anyhow::Result<_>>()?;

    Ok(PipeMap {
        tiles: map,
        start: start.ok_or_else(|| anyhow::anyhow!("Failed to find start tile"))?,
    })
}

#[derive(Debug, PartialEq)]
enum TileKind {
    Ground,
    Start,
    Pipe(PipeKind),
}

#[derive(Debug, PartialEq)]
enum PipeKind {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}
