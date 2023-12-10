pub fn part_one(input: &str) -> anyhow::Result<String> {
    let mut map = parse_input(input)?;
    let tile_loop = find_loop(&mut map);
    Ok((tile_loop.len() / 2).to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let mut map = parse_input(input)?;
    let tile_loop = find_loop(&mut map);
    let vertices = tile_loop.iter().filter(|tile| match &tile.kind {
        TileKind::Ground => false,
        TileKind::Start => true,
        TileKind::Pipe(kind) => !matches!(kind, PipeKind::Vertical | PipeKind::Horizontal),
    });

    Ok("not implemented".to_string())
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

    if x < map.tiles[0].len() - 1 {
        let right = &map.tiles[y][x + 1];
        if !right.visited && right.connects_left() && current.connects_right() {
            return Some(right);
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

#[derive(Clone)]
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

#[derive(Clone, PartialEq)]
enum TileKind {
    Ground,
    Start,
    Pipe(PipeKind),
}

#[derive(Clone, PartialEq)]
enum PipeKind {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}
