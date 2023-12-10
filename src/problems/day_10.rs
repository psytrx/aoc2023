pub fn part_one(input: &str) -> anyhow::Result<String> {
    let mut map = parse_input(input)?;
    let tile_loop = find_loop(&mut map);
    Ok((tile_loop.len() / 2).to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let mut map = parse_input(input)?;
    find_loop(&mut map);
    Ok(map
        .tiles
        .iter()
        .flatten()
        .filter(|&tile| inside_loop(&map, tile))
        .count()
        .to_string())
}

fn inside_loop(map: &PipeMap, tile: &Tile) -> bool {
    if tile.visited {
        false
    } else {
        // Helps us choose the side closer to the border
        let x_range = if tile.pos.0 < 70 {
            0..tile.pos.0
        } else {
            tile.pos.0 + 1..140
        };

        let intersections = map.tiles[tile.pos.1][x_range]
            .iter()
            .filter(|tile| tile.visited && "|LJ".contains(tile.kind))
            .count();

        intersections % 2 == 1
    }
}

fn find_loop(map: &mut PipeMap) -> Vec<Tile> {
    let mut vertices = Vec::with_capacity(8096);
    let (mut x, mut y) = map.start;

    loop {
        let current = &mut map.tiles[y][x];
        current.visited = true;
        vertices.push(current.clone());

        match unvisited_neighbor(map, (x, y)) {
            Some(next_tile) => {
                (x, y) = next_tile.pos;
            }
            None => break,
        }
    }

    vertices
}

fn unvisited_neighbor(map: &PipeMap, (x, y): (usize, usize)) -> Option<&Tile> {
    let current = &map.tiles[y][x];

    if y > 0 && current.connects_top() {
        let top = &map.tiles[y - 1][x];
        if !top.visited && top.connects_bottom() {
            return Some(top);
        }
    }

    if x < map.tiles[0].len() - 1 && current.connects_right() {
        let right = &map.tiles[y][x + 1];
        if !right.visited && right.connects_left() {
            return Some(right);
        }
    }

    if y < map.tiles.len() - 1 && current.connects_bottom() {
        let bottom = &map.tiles[y + 1][x];
        if !bottom.visited && bottom.connects_top() {
            return Some(bottom);
        }
    }

    if x > 0 && current.connects_left() {
        let left = &map.tiles[y][x - 1];
        if !left.visited && left.connects_right() {
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
                    if c == 'S' {
                        start = Some((x, y));
                    }
                    Tile {
                        pos: (x, y),
                        kind: c,
                        visited: false,
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

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
    kind: char,
    visited: bool,
}

impl Tile {
    fn connects_top(&self) -> bool {
        "|LJS".contains(self.kind)
    }

    fn connects_bottom(&self) -> bool {
        "|F7S".contains(self.kind)
    }

    fn connects_left(&self) -> bool {
        "-J7S".contains(self.kind)
    }

    fn connects_right(&self) -> bool {
        "-LFS".contains(self.kind)
    }
}
