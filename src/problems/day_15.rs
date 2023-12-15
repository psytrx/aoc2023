pub fn part_one(input: &str) -> anyhow::Result<String> {
    Ok(input.trim().split(',').map(hash).sum::<usize>().to_string())
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

    let mut boxes: Vec<std::collections::LinkedList<LabelledLens>> =
        vec![std::collections::LinkedList::new(); 256];

    for op in operations.iter() {
        match op {
            Operation::Remove(label) => {
                let box_ = &mut boxes[hash(label)];
                for (i, (lens_label, _)) in box_.iter().enumerate() {
                    if lens_label == label {
                        // unstable feature: linked_list_remove
                        box_.remove(i);
                        break;
                    }
                }
            }
            Operation::Move(label, focal_length) => {
                let box_ = &mut boxes[hash(label)];

                let mut replaced = false;
                for (lens_label, lens_focal_length) in box_.iter_mut() {
                    if lens_label == label {
                        *lens_focal_length = *focal_length;
                        replaced = true;
                        break;
                    }
                }

                if !replaced {
                    let labelled_lens = (label.clone(), *focal_length);
                    box_.push_back(labelled_lens);
                }
            }
        }
    }

    let focusing_power = boxes
        .iter()
        .enumerate()
        .map(|(box_idx, box_)| {
            box_.iter()
                .enumerate()
                .map(|(lens_idx, (_, focal_length))| (box_idx + 1) * (lens_idx + 1) * focal_length)
                .sum::<usize>()
        })
        .sum::<usize>();

    Ok(focusing_power.to_string())
}

fn hash(step: &str) -> usize {
    let mut hash = 0_usize;
    for &b in step.as_bytes().iter() {
        hash += b as usize;
        hash *= 17;
        hash %= 256;
    }
    hash
}

type LabelledLens = (String, usize);

#[derive(Debug)]
enum Operation {
    Remove(String),
    Move(String, usize),
}
