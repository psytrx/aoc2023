use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

pub fn part_one(input: &str) -> anyhow::Result<String> {
    Ok(compute_energy(
        &Beam {
            position: (0, 0),
            direction: 0,
            origin: ((0, 0), 0),
        },
        &parse_input(input),
    )
    .to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let contraption = parse_input(input);
    let beams = {
        let horizontal = (0..contraption.len()).flat_map(|y| {
            let left = (0, y as i32);
            let right = (contraption[0].len() as i32 - 1, y as i32);
            [
                Beam {
                    position: left,
                    direction: 0,
                    origin: (left, 0),
                },
                Beam {
                    position: right,
                    direction: 2,
                    origin: (right, 2),
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
                    origin: (top, 1),
                },
                Beam {
                    position: bottom,
                    direction: 3,
                    origin: (bottom, 3),
                },
            ]
        });
        horizontal.chain(vertical)
    };

    Ok(beams
        .collect::<Vec<_>>()
        .par_iter()
        .map(|beam| compute_energy(beam, &contraption))
        .max()
        .ok_or_else(|| anyhow::anyhow!("Failed to find max in empty iterator"))?
        .to_string())
}

fn compute_energy(beam: &Beam, contraption: &[Vec<Tile>]) -> usize {
    let mut contraption = contraption.to_vec();
    trace_beams(&[beam.clone()], &mut contraption);

    contraption
        .iter()
        .flat_map(|row| row.iter())
        .filter(|tile| !tile.beams.is_empty())
        .count()
}

fn trace_beams(beams: &[Beam], contraption: &mut [Vec<Tile>]) {
    let mut beams = beams.to_vec();

    while let Some(beam) = beams.pop() {
        let (x, y) = beam.position;
        if x < 0 || y < 0 || y >= contraption.len() as i32 || x >= contraption[0].len() as i32 {
            continue;
        }

        let tile = &mut contraption[y as usize][x as usize];
        if !tile.beams.insert(beam.clone()) {
            continue;
        }

        for ((dx, dy), dir) in interact(tile.kind, beam.direction) {
            beams.push(Beam {
                position: (x + dx, y + dy),
                direction: dir,
                origin: if dir == beam.direction {
                    beam.origin
                } else {
                    (beam.position, dir)
                },
            });
        }
    }
}

fn interact(tile_kind: u8, direction: u8) -> Vec<((i32, i32), u8)> {
    let mut beams = Vec::with_capacity(2);
    match tile_kind {
        b'.' => match direction {
            0 => beams.push(((1, 0), direction)),
            1 => beams.push(((0, 1), direction)),
            2 => beams.push(((-1, 0), direction)),
            3 => beams.push(((0, -1), direction)),
            _ => unreachable!(),
        },
        b'-' => match direction {
            0 => beams.push(((1, 0), direction)),
            2 => beams.push(((-1, 0), direction)),
            1 | 3 => {
                beams.push(((-1, 0), 2));
                beams.push(((1, 0), 0));
            }
            _ => unreachable!(),
        },
        b'|' => match direction {
            0 | 2 => {
                beams.push(((0, -1), 3));
                beams.push(((0, 1), 1));
            }
            1 => beams.push(((0, 1), direction)),
            3 => beams.push(((0, -1), direction)),
            _ => unreachable!(),
        },
        b'/' => match direction {
            0 => beams.push(((0, -1), 3)),
            1 => beams.push(((-1, 0), 2)),
            2 => beams.push(((0, 1), 1)),
            3 => beams.push(((1, 0), 0)),
            _ => unreachable!(),
        },
        b'\\' => match direction {
            0 => beams.push(((0, 1), 1)),
            1 => beams.push(((1, 0), 0)),
            2 => beams.push(((0, -1), 3)),
            3 => beams.push(((-1, 0), 2)),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };
    beams
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

#[derive(Clone, Eq, PartialEq)]
struct Beam {
    position: (i32, i32),
    direction: u8,
    origin: ((i32, i32), u8),
}

impl std::hash::Hash for Beam {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.direction.hash(state);
    }
}
