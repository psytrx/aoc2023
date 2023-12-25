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
    while g.nodes.len() > 2 {}
}

fn parse_input(input: &str) -> anyhow::Result<Graph> {
    let mut nodes = hashbrown::HashMap::new();

    for line in input.lines() {
        let (first, rest) = line
            .split_once(": ")
            .ok_or_else(|| anyhow::anyhow!("Failed to split line at :"))?;

        let mut connected_nodes = vec![first];
        for n in rest.split(' ') {
            connected_nodes.push(n);
        }

        for a in connected_nodes.iter() {
            let entry = nodes
                .entry(a.to_string())
                .or_insert_with(|| Node { edges: vec![] });
            for b in connected_nodes.iter() {
                entry.edges.push(Edge {
                    a: a.to_string(),
                    b: b.to_string(),
                });
            }
        }
    }

    Ok(Graph { nodes })
}

struct Graph {
    nodes: hashbrown::HashMap<String, Node>,
}

struct Node {
    edges: Vec<Edge>,
}

#[derive(Eq)]
struct Edge {
    a: String,
    b: String,
}

impl std::cmp::PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.a == other.a && self.b == other.b) || (self.a == other.b && self.b == other.a)
    }
}
