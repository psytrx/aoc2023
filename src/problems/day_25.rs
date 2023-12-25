pub fn part_one(input: &str) -> anyhow::Result<String> {
    let mut g = parse_input(input)?;
    collapse(&mut g);
    Ok("not implemented".to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let _g = parse_input(input)?;
    Ok("not implemented".to_string())
}

fn collapse(g: &mut Graph) {
    while g.nodes.len() > 2 {
        // Find 2 neighboring nodes that have
        // more than 3 _unique_ paths between each other

        let mut merge = vec![];

        for (a_id, a) in g.nodes.iter() {
            if a.edges.len() <= 3 {
                continue;
            }

            for e in a.edges.iter() {
                let b_id = e.other(a_id);
                let b = &g.nodes[b_id];

                if b.edges.len() <= 3 {
                    continue;
                }

                let mut exclude = hashbrown::HashSet::new();
                exclude.insert(e.to_owned());

                let mut found_paths = true;
                for _ in 1..=3 {
                    match find_path(g, a_id, b_id, &exclude) {
                        Some(path) => {
                            for e in path {
                                exclude.insert(e.to_owned());
                            }
                        }
                        None => {
                            found_paths = false;
                            break;
                        }
                    }
                }

                if !found_paths {
                    continue;
                }

                merge.push((a_id, b_id.to_owned()));

                break; // Can we just continue instead of break?
            }
        }

        for (a_id, b_id) in merge.iter() {
            // Merge b into a
            // let a = g.nodes.get_mut(&a_id);
        }
    }
}

fn find_path(
    g: &Graph,
    start_id: &str,
    end_id: &str,
    exclude: &hashbrown::HashSet<Edge>,
) -> Option<Vec<Edge>> {
    let mut heap = std::collections::BinaryHeap::from([(std::cmp::Reverse(0), start_id, vec![])]);
    let mut visited: hashbrown::HashSet<&str> = hashbrown::HashSet::from([start_id]);

    while let Some((std::cmp::Reverse(dist), node_id, path)) = heap.pop() {
        if node_id == end_id {
            return Some(path);
        }

        visited.insert(node_id);

        let node = &g.nodes[node_id];
        let edges = node.edges.iter().filter(|&edge| {
            let neighbor_id = edge.other(node_id);
            !visited.contains(neighbor_id) && !exclude.contains(edge)
        });

        for edge in edges {
            let new_dist = dist + 1;
            let new_node_id = edge.other(node_id);
            let mut new_path = path.clone();
            new_path.push(edge.clone());

            heap.push((std::cmp::Reverse(new_dist), new_node_id, new_path));
        }
    }

    None
}

fn parse_input(input: &str) -> anyhow::Result<Graph> {
    let mut nodes = hashbrown::HashMap::new();

    for line in input.lines() {
        let (from_id, rest) = line
            .split_once(": ")
            .ok_or_else(|| anyhow::anyhow!("Failed to split line at :"))?;

        for to_id in rest.split(' ') {
            let edge = Edge {
                a: from_id.to_owned(),
                b: to_id.to_owned(),
            };

            let from = nodes.entry(from_id.to_string()).or_insert_with(|| Node {
                id: from_id.to_string(),
                edges: hashbrown::HashSet::new(),
            });
            from.edges.insert(edge.clone());

            let to = nodes.entry(to_id.to_string()).or_insert_with(|| Node {
                id: to_id.to_string(),
                edges: hashbrown::HashSet::new(),
            });
            to.edges.insert(edge);
        }
    }

    Ok(Graph { nodes })
}

struct Graph {
    nodes: hashbrown::HashMap<String, Node>,
}

#[derive(Clone)]
struct Node {
    id: String,
    edges: hashbrown::HashSet<Edge>,
}

#[derive(Eq, Clone, PartialOrd, Ord, Debug)]
struct Edge {
    a: String,
    b: String,
}

impl Edge {
    fn other(&self, id: &str) -> &str {
        if self.a == id {
            &self.b
        } else {
            &self.a
        }
    }
}

impl std::cmp::PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.a == other.a && self.b == other.b) || (self.a == other.b && self.b == other.a)
    }
}

impl std::hash::Hash for Edge {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let (a, b) = if self.a <= self.b {
            (&self.a, &self.b)
        } else {
            (&self.b, &self.a)
        };

        a.hash(state);
        b.hash(state);
    }
}
