pub fn part_one(input: &str) -> anyhow::Result<String> {
    let map = parse_input(input)?;
    Ok(find_exit(&map, node_id_hash("AAA"))?.to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let map = parse_input(input)?;

    let starting_nodes = map
        .connections
        .iter()
        .enumerate()
        .filter_map(|(from, &(left, right))| {
            if left != 0 && right != 0 && (from & 0b11111) == 0 {
                Some(from)
            } else {
                None
            }
        });

    Ok(starting_nodes
        .map(|node| find_exit(&map, node))
        .collect::<anyhow::Result<Vec<i64>>>()?
        .into_iter()
        .reduce(|a, b| num::Integer::lcm(&a, &b))
        .ok_or_else(|| anyhow::anyhow!("Failed to reduce to lcm"))?
        .to_string())
}

fn find_exit(map: &Map, mut from_node: usize) -> anyhow::Result<i64> {
    let mut instruction_idx = 0;
    let trailing_hash = hash_char('Z');

    while (from_node & 0b11111) != trailing_hash {
        from_node = follow_instruction(map, from_node, instruction_idx)?;
        instruction_idx += 1;
    }

    Ok(instruction_idx as i64)
}

fn follow_instruction(map: &Map, current: usize, instruction_idx: usize) -> anyhow::Result<usize> {
    let instruction = map.instructions[instruction_idx % map.instructions.len()];

    let (left, right) = map.connections[current];
    match instruction {
        'L' => Ok(left),
        'R' => Ok(right),
        _ => anyhow::bail!("Failed to map invalid instruction: {}", instruction),
    }
}

fn parse_input(input: &str) -> anyhow::Result<Map> {
    input.lines().try_fold(Map::new(), |mut map, line| {
        if line.is_empty() {
            Ok(map)
        } else if let Some((from, to_list)) = line.split_once(" = ") {
            let (left, right) = to_list
                .strip_prefix('(')
                .ok_or_else(|| anyhow::anyhow!("Failed to strip ( prefix"))?
                .strip_suffix(')')
                .ok_or_else(|| anyhow::anyhow!("Failed to strip ) suffix"))?
                .split_once(", ")
                .ok_or_else(|| anyhow::anyhow!("Failed to split on , "))?;

            let hash = node_id_hash(from);
            let left = node_id_hash(left);
            let right = node_id_hash(right);

            map.connections[hash] = (left, right);
            Ok(map)
        } else {
            map.instructions = line.chars().collect();
            Ok(map)
        }
    })
}

fn hash_char(c: char) -> usize {
    c as usize - 'A' as usize
}

fn node_id_hash(node_id: &str) -> usize {
    node_id.chars().fold(0, |acc, c| (acc << 5) | hash_char(c))
}

#[derive(Debug)]
struct Map {
    instructions: Vec<char>,
    connections: Vec<(usize, usize)>,
}

impl Map {
    fn new() -> Self {
        Self {
            instructions: Vec::new(),
            connections: vec![(0, 0); node_id_hash("ZZZ") + 1],
        }
    }
}
