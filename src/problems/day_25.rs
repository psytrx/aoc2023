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
        for (a_id, a) in g.nodes.iter() {
            if a.edges.len() <= 3 {
                continue;
            }

            for edge in a.edges.iter() {
                let b_id = if &edge.a == a_id { &edge.b } else { &edge.a };
                let b = &g.nodes[b_id];
                if b.edges.len() <= 3 {
                    continue;
                }

                let mut validated = hashbrown::HashSet::new();
                validated.insert(edge);

                match find_path(g, a, b, &validated) {
                    _ => unreachable!(),
                }
            }
        }

        break;
    }
}

fn find_path(
    g: &Graph,
    a: &Node,
    b: &Node,
    exclude: &hashbrown::HashSet<&Edge>,
) -> Option<Vec<Edge>> {
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

struct Node {
    id: String,
    edges: hashbrown::HashSet<Edge>,
}

#[derive(Eq, Clone)]
struct Edge {
    a: String,
    b: String,
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
