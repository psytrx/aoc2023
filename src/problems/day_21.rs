pub fn part_one(input: &str) -> anyhow::Result<String> {
    let (mut grid, start) = parse_input(input)?;

    let mut queue = std::collections::VecDeque::from([QueueNode {
        pos: start,
        steps: 0,
    }]);

    let max_steps = 64;
    let neighbor_pos = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    while let Some(curr) = queue.pop_front() {
        let (x, y) = curr.pos;

        let n = &mut grid[y as usize][x as usize];
        if n.visited > 0 {
            continue;
        }
        n.visited = curr.steps;

        if curr.steps >= max_steps {
            continue;
        }

        for (dx, dy) in neighbor_pos {
            let (n_x, n_y) = (x + dx, y + dy);
            if n_y < 0
                || n_y >= grid.len() as i32
                || n_x < 0
                || n_x >= grid[y as usize].len() as i32
            {
                continue;
            }

            let n = &grid[n_y as usize][n_x as usize];
            if n.visited == 0 && n.kind == b'.' {
                queue.push_back(QueueNode {
                    pos: (n_x, n_y),
                    steps: curr.steps + 1,
                });
            }
        }
    }

    let n_visited = grid
        .iter()
        .flatten()
        .filter(|n| n.visited > 0 && n.visited % 2 == 0)
        .count();

    Ok(n_visited.to_string())
}

pub fn part_two(_input: &str) -> anyhow::Result<String> {
    Ok("not implemented".to_string())
}

struct QueueNode {
    pos: (i32, i32),
    steps: usize,
}

struct GridCell {
    kind: u8,
    visited: usize,
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
                        kind: b,
                        visited: 0,
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
