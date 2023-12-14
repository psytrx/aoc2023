pub fn part_one(input: &str) -> anyhow::Result<String> {
    Ok(
        north_beam_load(&rotate_dish_cw(&slide_dish_west(&rotate_dish_ccw(
            &parse_input(input),
        ))))
        .to_string(),
    )
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let mut dish = parse_input(input);

    let mut hashes = hashbrown::HashSet::with_capacity(1024);
    let mut loads = Vec::with_capacity(1024);
    let mut cycle_start = None;

    let test_cycles = 1_000_000_000;

    for i in 0..test_cycles {
        dish = rotate_dish_ccw(&dish);
        for _ in 0..4 {
            dish = slide_dish_west(&dish);
            dish = rotate_dish_cw(&dish);
        }
        dish = rotate_dish_cw(&dish);

        if !hashes.insert(dish.clone()) {
            if cycle_start.is_none() {
                cycle_start = Some(i);
                hashes.clear();
                hashes.insert(dish.clone());
            } else {
                break;
            }
        }

        if cycle_start.is_some() {
            loads.push(north_beam_load(&dish));
        }
    }

    let cycle_start = cycle_start.ok_or_else(|| anyhow::anyhow!("Failed to find cycle start"))?;
    let cycle_index = (test_cycles - cycle_start) % loads.len() - 1;
    Ok(loads[cycle_index].to_string())
}

fn north_beam_load(dish: &[String]) -> usize {
    dish.iter()
        .enumerate()
        .map(|(i, row)| {
            let num_rounded_rocks = row.chars().filter(|&c| c == 'O').count();
            let load_per_rock = dish.len() - i;
            load_per_rock * num_rounded_rocks
        })
        .sum::<usize>()
}

fn rotate_dish_ccw(dish: &[String]) -> Vec<String> {
    rotate_dish_cw(&rotate_dish_cw(&rotate_dish_cw(dish)))
}

fn rotate_dish_cw(dish: &[String]) -> Vec<String> {
    let mut rotated = Vec::with_capacity(dish[0].len());
    for i in 0..dish[0].len() {
        let mut row = Vec::with_capacity(dish.len());
        for string in dish.iter().rev() {
            row.push(string.as_bytes()[i] as char);
        }
        rotated.push(row.into_iter().collect());
    }
    rotated
}

fn slide_dish_west(dish: &[String]) -> Vec<String> {
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
