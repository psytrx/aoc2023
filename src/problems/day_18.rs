pub fn part_one(input: &str) -> anyhow::Result<String> {
    let instructions = parse_input(input)?;
    Ok(calculate_area(instructions)?.to_string())
}

fn calculate_area(instructions: Vec<DigInstruction>) -> anyhow::Result<i32> {
    let (mut x, mut y) = (0, 0);

    let mut inner_area = 0;
    let mut border_area = 0;

    for instruction in instructions {
        let (new_x, new_y) = match instruction.direction.as_str() {
            "R" => (x + instruction.length, y),
            "D" => (x, y + instruction.length),
            "L" => (x - instruction.length, y),
            "U" => (x, y - instruction.length),
            _ => anyhow::bail!("Invalid direction: {}", instruction.direction),
        };

        inner_area += (x - new_x) * (y + new_y);
        border_area += instruction.length;

        (x, y) = (new_x, new_y);
    }

    Ok((inner_area + border_area) / 2 + 1)
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
