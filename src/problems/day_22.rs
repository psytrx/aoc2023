pub fn part_one(input: &str) -> anyhow::Result<String> {
    let tower = compress_tower(parse_input(input)?);
    let result = tower
        .nodes
        .iter()
        .filter(|node| {
            node.outputs.iter().all(|&out_idx| {
                let out_node = &tower.nodes[out_idx];
                out_node.inputs.len() > 1
            })
        })
        .count();
    Ok(result.to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let tower = compress_tower(parse_input(input)?);
    // let bricks = collapse_tower(parse_input(input)?);
    let result = -1;
    Ok(result.to_string())
}

fn compress_tower(mut bricks: Vec<Brick>) -> CompressedTower {
    bricks.sort_by_key(|brick| *brick.z.start());

    let (dim_x, dim_y) = bricks.iter().fold((0, 0), |(x, y), brick| {
        (x.max(*brick.x.end() + 1), y.max(*brick.y.end() + 1))
    });
    let mut grid: Vec<Vec<(usize, Option<usize>)>> = vec![vec![(0, None); dim_y]; dim_x];
    let mut edges = hashbrown::HashSet::new();

    for (brick_idx, brick) in bricks.iter_mut().enumerate() {
        let mut max_z = 0;

        // Find the current max z
        for x in brick.x.clone() {
            for y in brick.y.clone() {
                let (z, _) = grid[x][y];
                max_z = max_z.max(z);
            }
        }

        // Update z, and store top brick ID
        for x in brick.x.clone() {
            for y in brick.y.clone() {
                let (z, upper_brick_idx) = grid[x][y];

                if z == max_z {
                    // Current brick is directly supported by upper_brick
                    if let Some(upper_brick_idx) = upper_brick_idx {
                        edges.insert((upper_brick_idx, brick_idx));
                    }
                }

                let brick_height = brick.z.end() - brick.z.start() + 1;
                grid[x][y] = (max_z + brick_height, Some(brick_idx));
            }
        }
    }

    let nodes = bricks
        .iter()
        .enumerate()
        .map(|(brick_idx, _)| {
            let inputs = edges
                .iter()
                .filter(|(_, to)| *to == brick_idx)
                .map(|(from, _)| *from)
                .collect();
            let outputs = edges
                .iter()
                .filter(|(from, _)| *from == brick_idx)
                .map(|(_, to)| *to)
                .collect();
            TowerNode { inputs, outputs }
        })
        .collect();

    CompressedTower { grid, edges, nodes }
}

struct CompressedTower {
    grid: Vec<Vec<(usize, Option<usize>)>>,
    edges: hashbrown::HashSet<(usize, usize)>,
    nodes: Vec<TowerNode>,
}

struct TowerNode {
    inputs: Vec<usize>,
    outputs: Vec<usize>,
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Brick>> {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let (from, to) = line
                .split_once('~')
                .ok_or_else(|| anyhow::anyhow!("Failed to split line into from/to"))?;

            let (from_x, from_y, from_z) = parse_coordinates(from)?;
            let (to_x, to_y, to_z) = parse_coordinates(to)?;

            Ok(Brick {
                i,
                x: from_x..=to_x,
                y: from_y..=to_y,
                z: from_z..=to_z,
            })
        })
        .collect::<anyhow::Result<_>>()
}

#[derive(Clone)]
struct Brick {
    i: usize,
    x: std::ops::RangeInclusive<usize>,
    y: std::ops::RangeInclusive<usize>,
    z: std::ops::RangeInclusive<usize>,
}

fn parse_coordinates(s: &str) -> anyhow::Result<(usize, usize, usize)> {
    match s
        .split(',')
        .map(|s| {
            s.parse::<usize>()
                .map_err(|e| anyhow::anyhow!("Failed to parse coordinate '{}': {}", s, e))
        })
        .collect::<anyhow::Result<Vec<_>>>()?[..]
    {
        [x, y, z] => Ok((x, y, z)),
        _ => {
            anyhow::bail!("Invalid length of coordinates, expected {}", 3)
        }
    }
}
