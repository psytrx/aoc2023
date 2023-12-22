pub fn part_one(input: &str) -> anyhow::Result<String> {
    let map = parse_input(input)?;
    let cost = find_longest_path(
        &map,
        (1, 0),
        (21, 22),
        0,
        0,
        vec![vec![false; map[0].len()]; map.len()],
    );
    Ok(cost.to_string())
}

fn find_longest_path(
    map: &[Vec<char>],
    cell: (usize, usize),
    end: (usize, usize),
    mut path_len: usize,
    max_path_len: usize,
    mut visited: Vec<Vec<bool>>,
) -> usize {
    if cell == end {
        return path_len.max(max_path_len);
    }

    let (x, y) = cell;
    visited[y][x] = true;
    path_len += 1;

    let moves = match map[y][x] {
        '#' => vec![],
        '.' => vec![(1, 0), (0, 1), (-1, 0), (0, -1)],
        '>' => vec![(1, 0)],
        'v' => vec![(0, 1)],
        '<' => vec![(-1, 0)],
        '^' => vec![(0, -1)],
        _ => unreachable!(),
    };

    let mut new_max_path_len = max_path_len;
    for (dx, dy) in moves {
        let x = x as i32 + dx;
        let y = y as i32 + dy;
        if y < 0 || y >= map.len() as i32 || x < 0 || x >= map[0].len() as i32 {
            continue;
        }

        let (x, y) = (x as usize, y as usize);
        if !visited[y][x] {
            new_max_path_len = new_max_path_len.max(find_longest_path(
                map,
                (x, y),
                end,
                path_len,
                new_max_path_len,
                visited.clone(),
            ));
        }
    }

    new_max_path_len
}

pub fn part_two(_input: &str) -> anyhow::Result<String> {
    Ok("not implemented".to_string())
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Vec<char>>> {
    Ok(input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>().to_owned())
        .collect())
}
