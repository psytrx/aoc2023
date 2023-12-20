pub fn part_one(input: &str) -> anyhow::Result<String> {
    let (mut grid, start) = parse_input(input)?;

    let mut queue = std::collections::VecDeque::new();
    queue.push_back(QueueNode {
        pos: start,
        steps: 0,
    });

    let max_steps = 16;
    let neighbor_pos = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut n_visited = 0;

    while let Some(curr) = queue.pop_front() {
        log::trace!("len: {}", queue.len());
        let (x, y) = curr.pos;

        let n = &mut grid[y as usize][x as usize];

        if curr.steps == max_steps && !n.visited {
            n.visited = true;
            n_visited += 1;
            break;
        }

        if curr.steps >= max_steps {
            continue;
        }

        let unqueued_neighbors = neighbor_pos.iter().filter_map(|(dx, dy)| {
            let (n_x, n_y) = (x + dx, y + dy);
            if n_y < 0
                || n_y >= grid.len() as i32
                || n_x < 0
                || n_x >= grid[y as usize].len() as i32
            {
                None
            } else {
                let n = &grid[n_y as usize][n_x as usize];
                if n.kind == b'.' {
                    Some(n)
                } else {
                    None
                }
            }
        });

        for n in unqueued_neighbors {
            queue.push_back(QueueNode {
                pos: n.pos,
                steps: curr.steps + 1,
            });
        }
    }

    Ok(n_visited.to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let _grid = parse_input(input)?;
    Ok("not implemented".to_string())
}

struct QueueNode {
    pos: (i32, i32),
    steps: usize,
}

struct GridCell {
    pos: (i32, i32),
    kind: u8,
    visited: bool,
}

type ParsedGrid = (Vec<Vec<GridCell>>, (i32, i32));

fn parse_input(input: &str) -> anyhow::Result<ParsedGrid> {
    let mut start = None;

    let grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(|(x, &b)| {
                    if b == b'S' {
                        start = Some((x as i32, y as i32));
                    }
                    let b = if b == b'#' { b'#' } else { b'.' };
                    GridCell {
                        pos: (x as i32, y as i32),
                        kind: b,
                        visited: false,
                    }
                })
                .collect()
        })
        .collect();

    Ok((
        grid,
        start.ok_or_else(|| anyhow::anyhow!("Failed to find start node"))?,
    ))
}
