pub fn part_one(input: &str) -> anyhow::Result<String> {
    let tower = compress_tower(parse_input(input)?);
    // log::trace!("tower: {:?}", tower);

    let display = tower
        .iter()
        .map(|col| col.iter().map(|(z, _)| z.to_string()).collect::<String>())
        .collect::<Vec<_>>();
    log::trace!("tower:\n{:#?}", display);

    // let bricks = collapse_tower(parse_input(input)?);
    //
    // let removable_bricks = bricks.iter().filter(|brick| {
    //     is_supporting(brick, &bricks)
    //         .iter()
    //         .all(|supported| supported_by(supported, &bricks).len() > 1)
    // });

    let result = -1;
    Ok(result.to_string())
}

fn compress_tower(mut bricks: Vec<Brick>) -> Vec<Vec<(usize, Option<usize>)>> {
    bricks.sort_by_key(|brick| *brick.z.start());

    let mut grid = {
        let (dim_x, dim_y) = bricks.iter().fold((0, 0), |(x, y), brick| {
            (x.max(*brick.x.end() + 1), y.max(*brick.y.end() + 1))
        });
        vec![vec![(0, None); dim_x]; dim_y]
    };

    let mut supports = hashbrown::HashSet::new();

    for brick in bricks.iter_mut() {
        let mut max_z = 0;

        // Find the current max z
        for x in brick.x.clone() {
            for y in brick.y.clone() {
                let (z, _) = grid[x][y];
                max_z = max_z.max(z);
            }
        }

        // Update z, and store top brick ID
        for x in brick.x.clone() {
            for y in brick.y.clone() {
                let (z, upper_brick) = grid[x][y];

                if z == max_z {
                    // Current brick is directly supported by upper_brick
                    if let Some(upper_brick) = upper_brick {
                        supports.insert((upper_brick, brick.id));
                    }
                }

                grid[x][y] = (max_z + 1, Some(brick.id));
            }
        }
    }

    log::trace!("supports: {:?}", supports);

    grid
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let tower = compress_tower(parse_input(input)?);
    // let bricks = collapse_tower(parse_input(input)?);
    let result = -1;
    Ok(result.to_string())
}

// fn is_supporting(brick: &Brick, bricks: &[Brick]) -> Vec<Brick> {
//     bricks
//         .iter()
//         .filter(|other| other.is_supported_by(brick))
//         .map(|brick| brick.to_owned())
//         .collect()
// }

// fn supported_by(brick: &Brick, bricks: &[Brick]) -> Vec<Brick> {
//     bricks
//         .iter()
//         .filter(|other| brick.is_supported_by(other))
//         .map(|brick| brick.to_owned())
//         .collect()
// }

// fn collapse_tower(mut bricks: Vec<Brick>) -> Vec<Brick> {
//     let mut dropped = true;
//     while dropped {
//         dropped = false;
//         bricks = bricks
//             .iter()
//             .map(|brick| {
//                 // Idea: If we could sort the bricks by z,
//                 // we can reduce complexity from O(n^2) to O(n log n)
//                 let can_drop =
//                     brick.z.lo > 1 && !bricks.iter().any(|other| brick.is_supported_by(other));
//                 if can_drop {
//                     dropped = true;
//                     // log::debug!("dropping brick: {:?}", brick);
//
//                     let new_z = Range::new(brick.z.lo - 1, brick.z.hi - 1);
//                     Brick {
//                         id: brick.id.to_owned(),
//                         x: brick.x.to_owned(),
//                         y: brick.y.to_owned(),
//                         z: new_z,
//                     }
//                 } else {
//                     brick.to_owned()
//                 }
//             })
//             .collect();
//     }
//     bricks
// }

fn parse_input(input: &str) -> anyhow::Result<Vec<Brick>> {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let (from, to) = line
                .split_once('~')
                .ok_or_else(|| anyhow::anyhow!("Failed to split line into from/to"))?;

            let (from_x, from_y, from_z) = parse_coordinates(from)?;
            let (to_x, to_y, to_z) = parse_coordinates(to)?;

            Ok(Brick {
                id: i,
                x: from_x..=to_x,
                y: from_y..=to_y,
                z: from_z..=to_z,
            })
        })
        .collect::<anyhow::Result<_>>()
}

#[derive(Clone)]
struct Brick {
    id: usize,
    x: std::ops::RangeInclusive<usize>,
    y: std::ops::RangeInclusive<usize>,
    z: std::ops::RangeInclusive<usize>,
}

fn parse_coordinates(s: &str) -> anyhow::Result<(usize, usize, usize)> {
    match s
        .split(',')
        .map(|s| {
            s.parse::<usize>()
                .map_err(|e| anyhow::anyhow!("Failed to parse coordinate '{}': {}", s, e))
        })
        .collect::<anyhow::Result<Vec<_>>>()?[..]
    {
        [x, y, z] => Ok((x, y, z)),
        _ => {
            anyhow::bail!("Invalid length of coordinates, expected {}", 3)
        }
    }
}
