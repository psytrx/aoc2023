pub fn part_one(input: &str) -> anyhow::Result<String> {
    let _g = parse_input(input);
    Ok("not implemented".to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let _g = parse_input(input);
    Ok("not implemented".to_string())
}

fn parse_input(input: &str) -> anyhow::Result<Graph> {
    let mut nodes = hashbrown::HashSet::new();
    let mut edges = hashbrown::HashSet::new();

    for line in input.lines() {
        let (first, rest) = line
            .split_once(": ")
            .ok_or_else(|| anyhow::anyhow!("Failed to split line at :"))?;

        let mut connected_nodes = vec![first];
        for n in rest.split(' ') {
            connected_nodes.push(n);
        }

        for a in connected_nodes.iter() {
            nodes.insert(a.to_string());
            for b in connected_nodes.iter() {
                edges.insert((a.to_string(), b.to_string()));
                edges.insert((b.to_string(), a.to_string()));
            }
        }
    }

    Ok(Graph { nodes, edges })
}

struct Graph {
    nodes: hashbrown::HashSet<String>,
    edges: hashbrown::HashSet<(String, String)>,
}
