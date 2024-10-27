pub fn part_one(input: &str) -> anyhow::Result<String> {
    Ok(solve(input, false)?.to_string())
}

pub fn part_two(_input: &str) -> anyhow::Result<String> {
    Ok("skipped".to_string())
    // Ok(solve(input, true)?.to_string())
}

fn solve(input: &str, ignore_slope: bool) -> anyhow::Result<usize> {
    let map = parse_input(input)?;

    let g = build_graph(&map, ignore_slope);
    let mut visited = vec![false; g.nodes.len()];
    let res = find_longest_path_tree(&g, 1, g.nodes.len() - 2, 0, 0, &mut visited);

    Ok(res)
}

fn find_longest_path_tree(
    g: &Graph,
    node_idx: usize,
    end_idx: usize,
    path_len: usize,
    max_path_len: usize,
    visited: &mut Vec<bool>,
) -> usize {
    if node_idx == end_idx {
        return path_len.max(max_path_len);
    }

    visited[node_idx] = true;
    let mut new_max_path_len = max_path_len;

    let node = &g.nodes[node_idx];
    for edge in node.edges.iter() {
        if visited[edge.to_idx] {
            continue;
        }

        new_max_path_len = new_max_path_len.max(find_longest_path_tree(
            g,
            edge.to_idx,
            end_idx,
            path_len + 1,
            new_max_path_len,
            visited,
        ));
    }

    visited[node_idx] = false;
    new_max_path_len
}

fn build_graph(map: &[Vec<u8>], ignore_slope: bool) -> Graph {
    let mut nodes = map
        .iter()
        .flat_map(|row| {
            row.iter()
                .map(|_| Node { edges: vec![] })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let cell = map[y][x];
            if cell == b'#' {
                continue;
            }

            let moves = if ignore_slope {
                vec![(1, 0), (0, 1), (-1, 0), (0, -1)]
            } else {
                match cell {
                    b'.' => vec![(1, 0), (0, 1), (-1, 0), (0, -1)],
                    b'>' => vec![(1, 0)],
                    b'v' => vec![(0, 1)],
                    b'<' => vec![(-1, 0)],
                    b'^' => vec![(0, -1)],
                    _ => unreachable!(),
                }
            };

            let curr_idx = y * map[0].len() + x;

            nodes[curr_idx].edges = moves
                .iter()
                .filter_map(|(dx, dy)| {
                    let (n_x, n_y) = (x as i32 + dx, y as i32 + dy);
                    if n_y < 0 || n_y >= map.len() as i32 || n_x < 0 || n_x >= map[0].len() as i32 {
                        return None;
                    }

                    let n_cell = map[n_y as usize][n_x as usize];
                    if n_cell == b'#' {
                        return None;
                    }

                    let to_idx = n_y as usize * map[0].len() + n_x as usize;
                    Some(Edge {
                        from_idx: curr_idx,
                        to_idx,
                        distance: 1,
                    })
                })
                .collect::<Vec<_>>();
        }
    }

    Graph { nodes }
}

struct Graph {
    nodes: Vec<Node>,
}

#[derive(Clone, Debug)]
struct Edge {
    from_idx: usize,
    to_idx: usize,
    distance: usize,
}

#[derive(Clone, Debug)]
struct Node {
    edges: Vec<Edge>,
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Vec<u8>>> {
    Ok(input
        .lines()
        .map(|line| line.as_bytes().to_owned())
        .collect())
}
