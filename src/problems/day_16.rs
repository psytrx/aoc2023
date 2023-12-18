use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let contraption = parse_input(input);
    let energy = compute_energy(
        &Beam {
            position: (0, 0),
            direction: 0,
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
                direction: 0,
            },
            Beam {
                position: right,
                direction: 2,
            },
        ]
    });
    let vertical = (0..contraption[0].len()).flat_map(|x| {
        let top = (x as i32, 0);
        let bottom = (x as i32, contraption.len() as i32 - 1);
        [
            Beam {
                position: top,
                direction: 1,
            },
            Beam {
                position: bottom,
                direction: 3,
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
            b'.' => {
                let position = match beam.direction {
                    0 => (x + 1, y),
                    1 => (x, y + 1),
                    2 => (x - 1, y),
                    3 => (x, y - 1),
                    _ => unreachable!(),
                };
                beams.push(Beam { position, ..beam });
            }
            b'-' => match beam.direction {
                0 | 2 => {
                    let new_x = match beam.direction {
                        2 => x - 1,
                        0 => x + 1,
                        _ => unreachable!(),
                    };
                    let position = (new_x, y);
                    beams.push(Beam { position, ..beam })
                }
                1 | 3 => {
                    beams.push(Beam {
                        position: (x - 1, y),
                        direction: 2,
                    });
                    beams.push(Beam {
                        position: (x + 1, y),
                        direction: 0,
                    });
                }
                _ => unreachable!(),
            },
            b'|' => match beam.direction {
                0 | 2 => {
                    beams.push(Beam {
                        position: (x, y - 1),
                        direction: 3,
                    });
                    beams.push(Beam {
                        position: (x, y + 1),
                        direction: 1,
                    });
                }
                1 | 3 => {
                    let new_y = match beam.direction {
                        3 => y - 1,
                        1 => y + 1,
                        _ => unreachable!(),
                    };
                    beams.push(Beam {
                        position: (x, new_y),
                        ..beam
                    })
                }
                _ => unreachable!(),
            },
            b'/' => match beam.direction {
                0 => beams.push(Beam {
                    position: (x, y - 1),
                    direction: 3,
                }),
                1 => beams.push(Beam {
                    position: (x - 1, y),
                    direction: 2,
                }),
                2 => beams.push(Beam {
                    position: (x, y + 1),
                    direction: 1,
                }),
                3 => beams.push(Beam {
                    position: (x + 1, y),
                    direction: 0,
                }),
                _ => unreachable!(),
            },
            b'\\' => match beam.direction {
                0 => beams.push(Beam {
                    position: (x, y + 1),
                    direction: 1,
                }),
                1 => beams.push(Beam {
                    position: (x + 1, y),
                    direction: 0,
                }),
                2 => beams.push(Beam {
                    position: (x, y - 1),
                    direction: 3,
                }),
                3 => beams.push(Beam {
                    position: (x - 1, y),
                    direction: 2,
                }),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|&c| Tile {
                    kind: c,
                    beams: hashbrown::hash_set::HashSet::new(),
                })
                .collect()
        })
        .collect()
}

#[derive(Clone)]
struct Tile {
    kind: u8,
    beams: hashbrown::hash_set::HashSet<Beam>,
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Beam {
    position: (i32, i32),
    direction: u8,
}
