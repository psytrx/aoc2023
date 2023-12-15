pub fn part_one(input: &str) -> anyhow::Result<String> {
    Ok(input.trim().split(',').map(hash).sum::<u64>().to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let operations = input
        .trim()
        .split(',')
        .map(|operation| {
            if let Some(label) = operation.strip_suffix('-') {
                Ok(Operation::Remove(label.to_string()))
            } else if let Some((label, focal_length)) = operation.split_once('=') {
                let focal_length = focal_length.parse::<usize>()?;
                Ok(Operation::Move(label.to_string(), focal_length))
            } else {
                unreachable!()
            }
        })
        .collect::<anyhow::Result<Vec<Operation>>>()?;
    log::trace!("{:?}", operations);
    Ok("not implemented".to_string())
}

fn hash(step: &str) -> u64 {
    let mut hash = 0_u64;
    for &b in step.as_bytes().iter() {
        hash += b as u64;
        hash *= 17;
        hash %= 256;
    }
    hash
}

#[derive(Debug)]
enum Operation {
    Remove(String),
    Move(String, usize),
}
