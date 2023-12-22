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
    let result = tower
        .nodes
        .iter()
        .map(|node| {
            let mut queue = std::collections::VecDeque::from([node]);
            let mut removed = hashbrown::HashSet::new();

            while let Some(node) = queue.pop_front() {
                if !removed.insert(node.idx) {
                    continue;
                }

                for &out_idx in &node.outputs {
                    let out_node = &tower.nodes[out_idx];
                    if out_node
                        .inputs
                        .iter()
                        .all(|&in_idx| removed.contains(&in_idx))
                    {
                        queue.push_back(out_node);
                    }
                }
            }

            removed.len() - 1
        })
        .sum::<usize>();
    Ok(result.to_string())
}

fn compress_tower(mut bricks: Vec<Brick>) -> CompressedTower {
    bricks.sort_by_key(|brick| *brick.z.start());

    let (dim_x, dim_y) = bricks.iter().fold((0, 0), |(x, y), brick| {
        (x.max(*brick.x.end() + 1), y.max(*brick.y.end() + 1))
    });
    let mut grid: Vec<Vec<(usize, Option<usize>)>> = vec![vec![(0, None); dim_y]; dim_x];
    let mut edges = hashbrown::HashSet::new();

    for brick in bricks.iter_mut() {
        let mut max_z = 0;
        let brick_height = brick.z.end() - brick.z.start() + 1;

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
                        edges.insert((upper_brick_idx, brick.idx));
                    }
                }

                grid[x][y] = (max_z + brick_height, Some(brick.idx));
            }
        }
    }

    let nodes = bricks
        .iter()
        .enumerate()
        .map(|(brick_idx, _)| {
            let inputs = edges
                .iter()
                .filter_map(|&(from_idx, to_idx)| {
                    if to_idx == brick_idx {
                        Some(from_idx)
                    } else {
                        None
                    }
                })
                .collect();
            let outputs = edges
                .iter()
                .filter_map(|&(from_idx, to_idx)| {
                    if from_idx == brick_idx {
                        Some(to_idx)
                    } else {
                        None
                    }
                })
                .collect();
            TowerNode {
                idx: brick_idx,
                inputs,
                outputs,
            }
        })
        .collect();

    CompressedTower { nodes }
}

struct CompressedTower {
    nodes: Vec<TowerNode>,
}

struct TowerNode {
    idx: usize,
    inputs: Vec<usize>,
    outputs: Vec<usize>,
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Brick>> {
    input
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            let (from, to) = line
                .split_once('~')
                .ok_or_else(|| anyhow::anyhow!("Failed to split line into from/to"))?;

            let (from_x, from_y, from_z) = parse_coordinates(from)?;
            let (to_x, to_y, to_z) = parse_coordinates(to)?;

            Ok(Brick {
                idx,
                x: from_x..=to_x,
                y: from_y..=to_y,
                z: from_z..=to_z,
            })
        })
        .collect::<anyhow::Result<_>>()
}

#[derive(Clone)]
struct Brick {
    idx: usize,
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
