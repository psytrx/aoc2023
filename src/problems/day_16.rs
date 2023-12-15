pub fn part_one(input: &str) -> anyhow::Result<String> {
    let mut contraption = parse_input(input);

    trace_beams(
        vec![Beam {
            position: (0, 0),
            direction: Direction::Right,
        }],
        &mut contraption,
    );

    let count = contraption
        .iter()
        .flat_map(|row| row.iter())
        .filter(|tile| !tile.beams.is_empty())
        .count();
    log::trace!("count: {}", count);

    let display = contraption
        .iter()
        .map(|row| {
            row.iter()
                .map(|tile| match tile.kind {
                    '|' => '|',
                    '-' => '-',
                    '/' => '/',
                    '\\' => '\\',
                    '.' => match tile.beams.len() {
                        0 => '.',
                        1 => {
                            let beam = tile.beams.iter().next().unwrap();
                            match beam.direction {
                                Direction::Up => '^',
                                Direction::Down => 'v',
                                Direction::Left => '<',
                                Direction::Right => '>',
                            }
                        }
                        n => std::char::from_digit(n as u32, 10).unwrap(),
                    },
                    _ => unreachable!(),
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n");
    log::trace!("display:\n{}", display);

    Ok("not implemented".to_string())
}

fn trace_beams(mut beams: Vec<Beam>, contraption: &mut Vec<Vec<Tile>>) {
    while let Some(beam) = beams.pop() {
        let (x, y) = beam.position;
        if x < 0 || y < 0 || x >= contraption[0].len() as i32 || y >= contraption.len() as i32 {
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
                beams.push(Beam {
                    position,
                    direction: beam.direction,
                });
            }
            '-' => match beam.direction {
                Direction::Left | Direction::Right => {
                    let new_x = match beam.direction {
                        Direction::Left => x - 1,
                        Direction::Right => x + 1,
                        _ => unreachable!(),
                    };
                    let position = (new_x, y);
                    beams.push(Beam {
                        position,
                        direction: beam.direction,
                    })
                }
                Direction::Up | Direction::Down => {
                    let new_y = match beam.direction {
                        Direction::Up => y - 1,
                        Direction::Down => y + 1,
                        _ => unreachable!(),
                    };
                    beams.push(Beam {
                        position: (x - 1, new_y),
                        direction: Direction::Left,
                    });
                    beams.push(Beam {
                        position: (x + 1, new_y),
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
                        direction: beam.direction,
                    })
                }
                Direction::Left | Direction::Right => {
                    let new_x = match beam.direction {
                        Direction::Left => x - 1,
                        Direction::Right => x + 1,
                        _ => unreachable!(),
                    };
                    beams.push(Beam {
                        position: (new_x, y - 1),
                        direction: Direction::Up,
                    });
                    beams.push(Beam {
                        position: (new_x, y + 1),
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

pub fn part_two(_input: &str) -> anyhow::Result<String> {
    Ok("not implemented".to_string())
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
