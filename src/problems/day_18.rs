pub fn part_one(input: &str) -> anyhow::Result<String> {
    let instructions = parse_input(input)?;

    let mut vertices = Vec::with_capacity(instructions.len());
    let (mut x, mut y) = (0, 0);
    for instruction in instructions {
        vertices.push((x, y));
        (x, y) = match instruction.direction.as_str() {
            "R" => (x + instruction.length, y),
            "D" => (x, y + instruction.length),
            "L" => (x - instruction.length, y),
            "U" => (x, y - instruction.length),
            _ => unreachable!(),
        }
    }
    vertices.push((x, y));

    let mut area = 0;
    for i in 0..vertices.len() {
        let (x1, y1) = vertices[i];
        let (x2, y2) = vertices[(i + 1) % vertices.len()];
        area += x1 * y2 - x2 * y1;
    }
    area /= 2;

    Ok(area.to_string())
}

pub fn part_two(_input: &str) -> anyhow::Result<String> {
    Ok("not implemented".to_string())
}

fn parse_input(input: &str) -> anyhow::Result<Vec<DigInstruction>> {
    input
        .lines()
        .map(|line| match line.split(' ').collect::<Vec<&str>>()[..] {
            [direction, length, color] => Ok(DigInstruction {
                direction: direction.to_string(),
                length: length.parse()?,
                color: color.to_string(),
            }),
            _ => anyhow::bail!("Failed to parse dig instruction"),
        })
        .collect::<anyhow::Result<_>>()
}

struct DigInstruction {
    direction: String,
    length: i32,
    color: String,
}
