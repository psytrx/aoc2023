pub fn part_one(input: &str) -> anyhow::Result<String> {
    let mut dish = parse_input(input);
    rotate_dish_ccw_in_place(&mut dish);

    let mut dish = slide_dish_west(dish);
    rotate_dish_cw_in_place(&mut dish);

    Ok(north_beam_load(dish).to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let mut dish = parse_input(input);

    let mut visited = hashbrown::HashMap::with_capacity(1024);
    visited.insert(dish.clone(), 0);

    let test_cycles = 1_000_000_000;

    let (cycle_start, cycle_end) = loop {
        rotate_dish_ccw_in_place(&mut dish);
        for _ in 0..4 {
            dish = slide_dish_west(dish);
            rotate_dish_cw_in_place(&mut dish);
            // dish = rotate_dish_cw(dish);
        }
        // dish = rotate_dish_cw(dish);
        rotate_dish_cw_in_place(&mut dish);

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

fn north_beam_load(dish: Vec<Vec<u8>>) -> usize {
    dish.iter()
        .enumerate()
        .map(|(i, row)| {
            let num_rounded_rocks = row.iter().filter(|&c| c == &b'O').count();
            let load_per_rock = dish.len() - i;
            load_per_rock * num_rounded_rocks
        })
        .sum::<usize>()
}

fn rotate_dish_cw_in_place(dish: &mut Vec<Vec<u8>>) {
    let n = dish.len();
    for layer in 0..n / 2 {
        let (lo, hi) = (layer, n - 1 - layer);
        for i in lo..hi {
            let offset = i - lo;

            // save top
            let top = dish[lo][i];

            dish[lo][i] = dish[hi - offset][lo];
            dish[hi - offset][lo] = dish[hi][hi - offset];
            dish[hi][hi - offset] = dish[i][hi];
            dish[i][hi] = top;
        }
    }
}

fn rotate_dish_ccw_in_place(dish: &mut Vec<Vec<u8>>) {
    let n = dish.len();
    for layer in 0..n / 2 {
        let (lo, hi) = (layer, n - 1 - layer);
        for i in lo..hi {
            let offset = i - lo;

            // save top
            let top = dish[lo][i];

            dish[lo][i] = dish[i][hi];
            dish[i][hi] = dish[hi][hi - offset];
            dish[hi][hi - offset] = dish[hi - offset][lo];
            dish[hi - offset][lo] = top;
        }
    }
}

fn slide_dish_west(dish: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    dish.into_iter()
        .map(|row| slide_row_west(row.to_vec()))
        .collect()
}

#[memoize::memoize]
fn slide_row_west(row: Vec<u8>) -> Vec<u8> {
    let mut row = row;
    let mut slid = true;
    while slid {
        slid = false;
        for i in 0..row.len() - 1 {
            if row[i] == b'.' && row[i + 1] == b'O' {
                row.swap(i, i + 1);
                slid = true;
            }
        }
    }
    row
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>()
}
