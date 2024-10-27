pub fn part_one(input: &str) -> anyhow::Result<String> {
    let _g = parse_input(input)?;
    // collapse(&mut g)?;
    Ok("not implemented".to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let _g = parse_input(input)?;
    Ok("not implemented".to_string())
}

#[allow(dead_code)]
fn collapse(g: &mut Graph) -> anyhow::Result<()> {
    while g.nodes.len() > 2 {
        // Find 2 neighboring nodes that have
        // more than 3 _unique_ paths between each other

        let mut merge = vec![];

        let nodes = g.nodes.clone();
        for (a_id, a) in nodes.iter() {
            if a.edges.len() <= 3 {
                continue;
            }

            for a_b_edge in a.edges.iter() {
                log::trace!("a_b_edge: {:?}", a_b_edge);

                let b_id = a_b_edge.other(a_id);
                let b = match g.nodes.get(b_id) {
                    Some(b) => b,
                    None => {
                        log::trace!("Failed to find node b '{}'", b_id);
                        continue;
                    }
                };

                if b.edges.len() <= 3 {
                    continue;
                }

                let mut exclude = hashbrown::HashSet::new();
                exclude.insert(a_b_edge.to_owned());

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

                merge.push(a_b_edge.to_owned());
                break;
            }
        }

        for edge in merge.iter() {
            log::trace!("merging {:?}", edge);

            let a = match g.nodes.remove(&edge.a) {
                Some(a) => a,
                None => {
                    log::trace!("Failed to find node a '{}'", edge.a);
                    continue;
                }
            };

            let b = match g.nodes.remove(&edge.b) {
                Some(b) => b,
                None => {
                    log::trace!("Failed to find node b '{}'", edge.b);
                    continue;
                }
            };

            let merged = Node {
                id: format!("{},{}", edge.a, edge.b),
                edges: a
                    .edges
                    .union(&b.edges)
                    .filter(|e| e != &edge)
                    .cloned()
                    .collect(),
            };

            g.nodes.insert(merged.id.to_string(), merged.clone());

            for a_edge in a.edges.iter() {
                log::trace!("a_edge: {:?}", a_edge);
                let other_id = a_edge.other(&a.id);
                let other = g.nodes.get_mut(other_id);
                if let Some(other) = other {
                    other.edges = other
                        .edges
                        .iter()
                        .filter(|e| e.a != a.id && e.b != a.id)
                        .cloned()
                        .collect();
                    other.edges.insert(Edge {
                        a: other.id.to_owned(),
                        b: merged.id.to_owned(),
                    });
                }
            }

            for b_edge in b.edges.iter() {
                log::trace!("b_edge: {:?}", b_edge);
                let other_id = b_edge.other(&b.id);
                let other = g.nodes.get_mut(other_id);
                if let Some(other) = other {
                    other.edges = other
                        .edges
                        .iter()
                        .filter(|e| e.a != b.id && e.b != b.id)
                        .cloned()
                        .collect();
                    other.edges.insert(Edge {
                        a: other.id.to_owned(),
                        b: merged.id.to_owned(),
                    });
                }
            }
        }
    }

    Ok(())
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

        let node = match g.nodes.get(node_id) {
            Some(node) => node,
            None => continue,
        };
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
