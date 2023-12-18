pub fn part_one(input: &str) -> anyhow::Result<String> {
    Ok(
        north_beam_load(rotate_dish_cw(slide_dish_west(rotate_dish_ccw(
            parse_input(input),
        ))))
        .to_string(),
    )
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let mut dish = parse_input(input);

    let mut visited = hashbrown::HashMap::with_capacity(1024);
    visited.insert(dish.clone(), 0);

    let test_cycles = 1_000_000_000;

    let (cycle_start, cycle_end) = loop {
        dish = rotate_dish_ccw(dish);
        for _ in 0..4 {
            dish = slide_dish_west(dish);
            dish = rotate_dish_cw(dish);
        }
        dish = rotate_dish_cw(dish);

        if let Some(prev) = visited.insert(dish.clone(), visited.len()) {
            break (prev, visited.len());
        }
    };

    let cycle_offset = test_cycles - cycle_start;
    let cycle_len = cycle_end - cycle_start;
    let rem = cycle_offset % cycle_len;
    let target = cycle_start + rem;
    let (dish, _) = visited
        .iter()
        .find(|(_, &i)| i == target)
        .ok_or_else(|| anyhow::anyhow!("Failed to find target dish"))?;

    Ok(north_beam_load(dish.clone()).to_string())
}

fn north_beam_load(dish: Vec<String>) -> usize {
    dish.iter()
        .enumerate()
        .map(|(i, row)| {
            let num_rounded_rocks = row.chars().filter(|&c| c == 'O').count();
            let load_per_rock = dish.len() - i;
            load_per_rock * num_rounded_rocks
        })
        .sum::<usize>()
}

fn rotate_dish_cw(dish: Vec<String>) -> Vec<String> {
    let dish_len = dish.len();
    let row_len = dish[0].len();
    let mut rotated = Vec::with_capacity(row_len);

    for i in 0..row_len {
        let mut row = String::with_capacity(dish_len);
        for string in dish.iter().rev() {
            row.push(string.as_bytes()[i] as char);
        }
        rotated.push(row);
    }
    rotated
}

fn rotate_dish_ccw(dish: Vec<String>) -> Vec<String> {
    let dish_len = dish.len();
    let row_len = dish[0].len();
    let mut rotated = Vec::with_capacity(row_len);

    for i in (0..row_len).rev() {
        let mut row = String::with_capacity(dish_len);
        for string in dish.iter() {
            row.push(string.as_bytes()[i] as char);
        }
        rotated.push(row);
    }
    rotated
}

fn slide_dish_west(dish: Vec<String>) -> Vec<String> {
    dish.iter()
        .map(|row| slide_row_west(row.to_string()))
        .collect()
}

#[memoize::memoize]
fn slide_row_west(row: String) -> String {
    let mut slided = row.to_string();
    while slided.contains(".O") {
        slided = slided.replace(".O", "O.");
    }
    slided
}

fn parse_input(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>()
}
