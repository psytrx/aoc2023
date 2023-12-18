pub fn part_one(input: &str) -> anyhow::Result<String> {
    let mut dish = parse_input(input);
    slide_dish_north_in_place(&mut dish);
    Ok(north_beam_load(dish).to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let mut dish = parse_input(input);

    let mut visited = hashbrown::HashMap::with_capacity(1024);
    visited.insert(dish.clone(), 0);

    let test_cycles = 1_000_000_000;

    let (cycle_start, cycle_end) = loop {
        slide_dish_north_in_place(&mut dish);
        slide_dish_west_in_place(&mut dish);
        slide_dish_south_in_place(&mut dish);
        slide_dish_east_in_place(&mut dish);

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

fn slide_dish_north_in_place(dish: &mut [Vec<u8>]) {
    for x in 0..dish[0].len() {
        let mut roll_until = 0;
        for y in 0..dish.len() {
            if dish[y][x] == b'O' {
                if y > roll_until {
                    dish[roll_until][x] = b'O';
                    dish[y][x] = b'.';
                }
                roll_until += 1;
            } else if dish[y][x] == b'#' {
                roll_until = y + 1;
            }
        }
    }
}

fn slide_dish_east_in_place(dish: &mut [Vec<u8>]) {
    for y in 0..dish.len() {
        let mut roll_until = dish[0].len() - 1;
        for x in (0..dish[0].len()).rev() {
            if dish[y][x] == b'O' {
                if x < roll_until {
                    dish[y][roll_until] = b'O';
                    dish[y][x] = b'.';
                }
                roll_until -= 1;
            } else if dish[y][x] == b'#' {
                roll_until = x - 1;
            }
        }
    }
}

fn slide_dish_south_in_place(dish: &mut [Vec<u8>]) {
    for x in 0..dish[0].len() {
        let mut roll_until = dish.len() - 1;
        for y in (0..dish.len()).rev() {
            if dish[y][x] == b'O' {
                if y < roll_until {
                    dish[roll_until][x] = b'O';
                    dish[y][x] = b'.';
                }
                roll_until -= 1;
            } else if dish[y][x] == b'#' {
                roll_until = y - 1;
            }
        }
    }
}

fn slide_dish_west_in_place(dish: &mut [Vec<u8>]) {
    for y in 0..dish.len() {
        let mut roll_until = 0;
        for x in 0..dish[0].len() {
            if dish[y][x] == b'O' {
                if x > roll_until {
                    dish[y][roll_until] = b'O';
                    dish[y][x] = b'.';
                }
                roll_until += 1;
            } else if dish[y][x] == b'#' {
                roll_until = x + 1;
            }
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>()
}
