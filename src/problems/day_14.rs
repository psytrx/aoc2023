pub fn part_one(input: &str) -> anyhow::Result<String> {
    Ok(load(&rotate_dish_cw(&slide_dish_west(&rotate_dish_ccw(
        &parse_input(input),
    ))))
    .to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    Ok("foo".to_string())
}

fn load(dish: &[String]) -> usize {
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
        let mut row = String::with_capacity(dish.len());
        for j in (0..dish.len()).rev() {
            row.push(dish[j].chars().nth(i).unwrap());
        }
        rotated.push(row);
    }
    rotated
}

fn slide_dish_west(dish: &[String]) -> Vec<String> {
    dish.iter().map(|row| slide_row_west(row)).collect()
}

fn slide_row_west(row: &str) -> String {
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
