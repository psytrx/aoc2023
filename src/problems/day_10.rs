pub fn part_one(input: &str) -> anyhow::Result<String> {
    let mut map = parse_input(input)?;
    let tile_loop = find_loop(&mut map);
    Ok((tile_loop.len() / 2).to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let mut map = parse_input(input)?;
    find_loop(&mut map);

    let inside = map
        .tiles
        .iter()
        .flatten()
        .filter(|&tile| !tile.visited && inside_loop(&map, tile))
        .count();

    Ok(inside.to_string())
}

fn inside_loop(map: &PipeMap, tile: &Tile) -> bool {
    // Helps us choose the shorter range to the border
    let x_range = if tile.pos.0 < 70 {
        0..tile.pos.0
    } else {
        tile.pos.0 + 1..140
    };

    let intersections = map.tiles[tile.pos.1][x_range]
        .iter()
        .filter(|tile| {
            tile.visited
                && (tile.kind == TileKind::Pipe(PipeKind::Vertical)
                    || tile.kind == TileKind::Pipe(PipeKind::NorthEast)
                    || tile.kind == TileKind::Pipe(PipeKind::NorthWest))
        })
        .count();

    intersections % 2 == 1
}

fn find_loop(map: &mut PipeMap) -> Vec<Tile> {
    let mut vertices = Vec::with_capacity(8096);
    let (mut x, mut y) = map.start;

    loop {
        let current_tile = &mut map.tiles[y][x];
        current_tile.visited = true;
        vertices.push(current_tile.clone());

        match find_unvisited_neighbor(map, (x, y)) {
            Some(next_tile) => {
                (x, y) = next_tile.pos;
            }
            None => break,
        }
    }

    vertices
}

fn find_unvisited_neighbor(map: &PipeMap, (x, y): (usize, usize)) -> Option<&Tile> {
    let current = &map.tiles[y][x];

    if y > 0 {
        let top = &map.tiles[y - 1][x];
        if !top.visited && top.connects_bottom() && current.connects_top() {
            return Some(top);
        }
    }

    if x < map.tiles[0].len() - 1 {
        let right = &map.tiles[y][x + 1];
        if !right.visited && right.connects_left() && current.connects_right() {
            return Some(right);
        }
    }

    if y < map.tiles.len() - 1 {
        let bottom = &map.tiles[y + 1][x];
        if !bottom.visited && bottom.connects_top() && current.connects_bottom() {
            return Some(bottom);
        }
    }

    if x > 0 {
        let left = &map.tiles[y][x - 1];
        if !left.visited && left.connects_right() && current.connects_left() {
            return Some(left);
        }
    }

    None
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
                        pos: (x, y),
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

struct PipeMap {
    tiles: Vec<Vec<Tile>>,
    start: (usize, usize),
}

#[derive(Debug, Clone, PartialEq)]
struct Tile {
    pos: (usize, usize),
    kind: TileKind,
    visited: bool,
}

impl Tile {
    fn connects_top(&self) -> bool {
        matches!(
            self.kind,
            TileKind::Start
                | TileKind::Pipe(PipeKind::Vertical)
                | TileKind::Pipe(PipeKind::NorthEast)
                | TileKind::Pipe(PipeKind::NorthWest)
        )
    }

    fn connects_bottom(&self) -> bool {
        matches!(
            self.kind,
            TileKind::Start
                | TileKind::Pipe(PipeKind::Vertical)
                | TileKind::Pipe(PipeKind::SouthWest)
                | TileKind::Pipe(PipeKind::SouthEast)
        )
    }

    fn connects_left(&self) -> bool {
        matches!(
            self.kind,
            TileKind::Start
                | TileKind::Pipe(PipeKind::Horizontal)
                | TileKind::Pipe(PipeKind::NorthWest)
                | TileKind::Pipe(PipeKind::SouthWest)
        )
    }

    fn connects_right(&self) -> bool {
        matches!(
            self.kind,
            TileKind::Start
                | TileKind::Pipe(PipeKind::Horizontal)
                | TileKind::Pipe(PipeKind::NorthEast)
                | TileKind::Pipe(PipeKind::SouthEast)
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
enum TileKind {
    Ground,
    Start,
    Pipe(PipeKind),
}

#[derive(Debug, Clone, PartialEq)]
enum PipeKind {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}
