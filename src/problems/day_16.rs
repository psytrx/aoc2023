use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let contraption = parse_input(input);
    let energy = compute_energy(
        &Beam {
            position: (0, 0),
            direction: Direction::Right,
        },
        &contraption,
    );
    Ok(energy.to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let contraption = parse_input(input);

    let horizontal = (0..contraption.len()).flat_map(|y| {
        let left = (0, y as i32);
        let right = (contraption[0].len() as i32 - 1, y as i32);
        [
            Beam {
                position: left,
                direction: Direction::Right,
            },
            Beam {
                position: right,
                direction: Direction::Left,
            },
        ]
    });
    let vertical = (0..contraption[0].len()).flat_map(|x| {
        let top = (x as i32, 0);
        let bottom = (x as i32, contraption.len() as i32 - 1);
        [
            Beam {
                position: top,
                direction: Direction::Down,
            },
            Beam {
                position: bottom,
                direction: Direction::Up,
            },
        ]
    });
    let beams = horizontal.chain(vertical);

    let max_energy = beams
        .collect::<Vec<_>>()
        .par_iter()
        .map(|beam| compute_energy(beam, &contraption))
        .max()
        .ok_or_else(|| anyhow::anyhow!("Failed to find max in empty iterator"))?;

    Ok(max_energy.to_string())
}

fn compute_energy(beam: &Beam, contraption: &[Vec<Tile>]) -> usize {
    let beams = vec![beam.clone()];
    let mut contraption = contraption.to_vec();

    trace_beams(&beams, &mut contraption);

    contraption
        .iter()
        .flat_map(|row| row.iter())
        .filter(|tile| !tile.beams.is_empty())
        .count()
}

fn trace_beams(beams: &[Beam], contraption: &mut [Vec<Tile>]) {
    let mut beams = beams.to_vec().clone();

    while let Some(beam) = beams.pop() {
        let (x, y) = beam.position;
        if x < 0 || y < 0 || y >= contraption.len() as i32 || x >= contraption[0].len() as i32 {
            continue;
        }

        let tile = &mut contraption[y as usize][x as usize];
        if !tile.beams.insert(beam.clone()) {
            continue;
        }

        match tile.kind {
            '.' => {
                let position = match beam.direction {
                    Direction::Up => (x, y - 1),
                    Direction::Down => (x, y + 1),
                    Direction::Left => (x - 1, y),
                    Direction::Right => (x + 1, y),
                };
                beams.push(Beam { position, ..beam });
            }
            '-' => match beam.direction {
                Direction::Left | Direction::Right => {
                    let new_x = match beam.direction {
                        Direction::Left => x - 1,
                        Direction::Right => x + 1,
                        _ => unreachable!(),
                    };
                    let position = (new_x, y);
                    beams.push(Beam { position, ..beam })
                }
                Direction::Up | Direction::Down => {
                    beams.push(Beam {
                        position: (x - 1, y),
                        direction: Direction::Left,
                    });
                    beams.push(Beam {
                        position: (x + 1, y),
                        direction: Direction::Right,
                    });
                }
            },
            '|' => match beam.direction {
                Direction::Up | Direction::Down => {
                    let new_y = match beam.direction {
                        Direction::Up => y - 1,
                        Direction::Down => y + 1,
                        _ => unreachable!(),
                    };
                    beams.push(Beam {
                        position: (x, new_y),
                        ..beam
                    })
                }
                Direction::Left | Direction::Right => {
                    beams.push(Beam {
                        position: (x, y - 1),
                        direction: Direction::Up,
                    });
                    beams.push(Beam {
                        position: (x, y + 1),
                        direction: Direction::Down,
                    });
                }
            },
            '/' => match beam.direction {
                Direction::Up => beams.push(Beam {
                    position: (x + 1, y),
                    direction: Direction::Right,
                }),
                Direction::Down => beams.push(Beam {
                    position: (x - 1, y),
                    direction: Direction::Left,
                }),
                Direction::Left => beams.push(Beam {
                    position: (x, y + 1),
                    direction: Direction::Down,
                }),
                Direction::Right => beams.push(Beam {
                    position: (x, y - 1),
                    direction: Direction::Up,
                }),
            },
            '\\' => match beam.direction {
                Direction::Up => beams.push(Beam {
                    position: (x - 1, y),
                    direction: Direction::Left,
                }),
                Direction::Down => beams.push(Beam {
                    position: (x + 1, y),
                    direction: Direction::Right,
                }),
                Direction::Left => beams.push(Beam {
                    position: (x, y - 1),
                    direction: Direction::Up,
                }),
                Direction::Right => beams.push(Beam {
                    position: (x, y + 1),
                    direction: Direction::Down,
                }),
            },
            _ => unreachable!(),
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Tile {
                    kind: c,
                    beams: std::collections::HashSet::new(),
                })
                .collect()
        })
        .collect()
}

#[derive(Clone)]
struct Tile {
    kind: char,
    beams: std::collections::HashSet<Beam>,
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Beam {
    position: (i32, i32),
    direction: Direction,
}

#[derive(Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
