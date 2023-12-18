pub fn part_one(input: &str) -> anyhow::Result<String> {
    Ok(minimize_heat_loss(parse_input(input)?, 1, 3).to_string())
}

fn minimize_heat_loss(grid: Vec<Vec<i32>>, min_dist: i32, max_dist: i32) -> i32 {
    let mut visited = vec![vec![[false, false]; grid[0].len()]; grid.len()];
    let mut heat_loss = vec![vec![[i32::MAX, i32::MAX]; grid[0].len()]; grid.len()];

    let mut heap = std::collections::BinaryHeap::new();
    heap.push((std::cmp::Reverse(0), (0, 0), 0));
    heap.push((std::cmp::Reverse(0), (0, 0), 1));

    let (end_x, end_y) = (grid[0].len() - 1, grid.len() - 1);

    while let Some((std::cmp::Reverse(total_heat_loss), (x, y), dir)) = heap.pop() {
        if x == end_x && y == end_y {
            return total_heat_loss;
        }

        if visited[y][x][dir] {
            continue;
        }
        visited[y][x][dir] = true;

        for sign in [-1_i32, 1_i32] {
            let mut heat_loss_sum = 0;
            for dist in 1..=max_dist {
                let x = x as i32 + sign * ((dir as i32 + 1) % 2) * dist;
                let y = y as i32 + sign * (dir as i32 % 2) * dist;

                if y >= 0 && y < grid.len() as i32 && x >= 0 && x < grid[0].len() as i32 {
                    heat_loss_sum += grid[y as usize][x as usize];

                    if dist >= min_dist {
                        let new_total_heat_loss = total_heat_loss + heat_loss_sum;
                        let min_heat_loss = &mut heat_loss[y as usize][x as usize][dir % 2];
                        if new_total_heat_loss < *min_heat_loss {
                            *min_heat_loss = new_total_heat_loss;
                            let new_dir = (dir + 1) % 2;
                            heap.push((
                                std::cmp::Reverse(new_total_heat_loss),
                                (x as usize, y as usize),
                                new_dir,
                            ));
                        }
                    }
                }
            }
        }
    }

    unreachable!()
}

pub fn part_two(_input: &str) -> anyhow::Result<String> {
    Ok("not implemented".to_string())
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Vec<i32>>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<i32>().map_err(anyhow::Error::from))
                .collect::<anyhow::Result<_>>()
        })
        .collect::<anyhow::Result<_>>()
}
