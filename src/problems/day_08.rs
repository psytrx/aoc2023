pub fn part_one(input: &str) -> anyhow::Result<String> {
    let map = parse_input(input)?;
    Ok(find_exit(&map, node_id_hash("AAA")).to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let map = parse_input(input)?;
    Ok(map
        .connections
        .iter()
        .enumerate()
        .filter_map(|(from, &(left, right))| {
            if left != 0 && right != 0 && (from & 0b11111) == 0 {
                Some(find_exit(&map, from))
            } else {
                None
            }
        })
        .fold(1, |a, b| num::Integer::lcm(&a, &b))
        .to_string())
}

fn find_exit(map: &Map, mut from_node: usize) -> i64 {
    let mut instruction_idx = 0;
    while (from_node & 0b11111) != 25 {
        from_node = follow_instruction(map, from_node, instruction_idx);
        instruction_idx += 1;
    }
    instruction_idx as i64
}

fn follow_instruction(map: &Map, current: usize, instruction_idx: usize) -> usize {
    let instruction = map.instructions[instruction_idx % map.instructions.len()];

    let (left, right) = map.connections[current];
    if instruction == 'L' {
        left
    } else {
        right
    }
}

fn parse_input(input: &str) -> anyhow::Result<Map> {
    input.lines().try_fold(Map::new(), |mut map, line| {
        Ok(if line.is_empty() {
            map
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
            map
        } else {
            map.instructions = line.chars().collect();
            map
        })
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
            instructions: Vec::with_capacity(512),
            connections: vec![(0, 0); node_id_hash("ZZZ") + 1],
        }
    }
}
