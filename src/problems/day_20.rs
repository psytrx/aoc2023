pub fn part_one(input: &str) -> anyhow::Result<String> {
    let mut machines = parse_input(input)?;

    let (mut low_sent, mut high_sent) = (0, 0);
    for _ in 1..=1000 {
        let (low, high) = push_button(&mut machines)?;

        low_sent += low;
        high_sent += high;
    }

    let prod = low_sent * high_sent;
    Ok(prod.to_string())
}

pub fn part_two(_input: &str) -> anyhow::Result<String> {
    Ok("not implemented".to_string())
}

fn push_button(
    machines: &mut hashbrown::hash_map::HashMap<String, Machine>,
) -> anyhow::Result<(usize, usize)> {
    let mut signals = std::collections::VecDeque::new();

    signals.push_front(Signal {
        src: "button".to_string(),
        dst: "broadcaster".to_string(),
        pulse: Pulse::Low,
    });

    let mut low_sent = 1;
    let mut high_sent = 0;

    while let Some(signal) = signals.pop_front() {
        match machines.get_mut(&signal.dst) {
            None => log::warn!("Failed to find dst machine {}", signal.dst),
            Some(machine) => {
                if let Some(out_pulse) = machine.process(&signal)? {
                    let destinations = match machine {
                        Machine::Broadcaster { destinations } => destinations,
                        Machine::FlipFlop {
                            state: _,
                            destinations,
                        } => destinations,
                        Machine::Conjunction {
                            state: _,
                            destinations,
                        } => destinations,
                    };

                    for dst in destinations {
                        signals.push_back(Signal {
                            src: signal.dst.clone(),
                            dst: dst.clone(),
                            pulse: out_pulse.clone(),
                        });

                        match out_pulse {
                            Pulse::Low => low_sent += 1,
                            Pulse::High => high_sent += 1,
                        }
                    }
                }
            }
        }
    }

    Ok((low_sent, high_sent))
}

fn parse_input(input: &str) -> anyhow::Result<hashbrown::hash_map::HashMap<String, Machine>> {
    let mut inputs = hashbrown::hash_map::HashMap::new();

    let mut machines = input
        .lines()
        .map(|line| {
            let (module, destinations) = line
                .split_once(" -> ")
                .ok_or_else(|| anyhow::anyhow!("Failed to split module and destinations"))?;

            let destinations = destinations
                .split(", ")
                .map(str::to_string)
                .collect::<Vec<_>>();

            let (name, machine) = if module == "broadcaster" {
                (
                    module.to_string(),
                    Machine::Broadcaster {
                        destinations: destinations.clone(),
                    },
                )
            } else if let Some(name) = module.strip_prefix('%') {
                (
                    name.to_string(),
                    Machine::FlipFlop {
                        state: false,
                        destinations: destinations.clone(),
                    },
                )
            } else if let Some(name) = module.strip_prefix('&') {
                (
                    name.to_string(),
                    Machine::Conjunction {
                        state: hashbrown::hash_map::HashMap::new(),
                        destinations: destinations.clone(),
                    },
                )
            } else {
                unreachable!()
            };

            for dst in destinations {
                let entry = inputs.entry(dst.clone()).or_insert(vec![]);
                entry.push(name.clone());
            }

            Ok((name, machine))
        })
        .collect::<anyhow::Result<hashbrown::hash_map::HashMap<_, _>>>()?;

    for (dst, inputs) in inputs.iter() {
        match machines.get_mut(dst) {
            Some(dst_machine) => {
                if let Machine::Conjunction {
                    state,
                    destinations: _,
                } = dst_machine
                {
                    for src in inputs {
                        state.insert(src.clone(), Pulse::Low);
                    }
                }
            }
            None => log::warn!("Failed to find dst machine {}", dst),
        }
    }

    Ok(machines)
}

#[derive(Clone, Debug, PartialEq)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug)]
struct Signal {
    src: String,
    dst: String,
    pulse: Pulse,
}

impl std::fmt::Display for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} -{}-> {}",
            self.src,
            match self.pulse {
                Pulse::Low => "low",
                Pulse::High => "high",
            },
            self.dst
        )
    }
}

#[derive(Debug)]
enum Machine {
    Broadcaster {
        destinations: Vec<String>,
    },
    FlipFlop {
        state: bool,
        destinations: Vec<String>,
    },
    Conjunction {
        state: hashbrown::hash_map::HashMap<String, Pulse>,
        destinations: Vec<String>,
    },
}

impl Machine {
    fn process(&mut self, signal: &Signal) -> anyhow::Result<Option<Pulse>> {
        match self {
            Machine::Broadcaster { destinations: _ } => Ok(Some(signal.pulse.clone())),
            Machine::FlipFlop {
                state,
                destinations: _,
            } => {
                if signal.pulse == Pulse::Low {
                    *state = !*state;
                    Ok(Some(if *state { Pulse::High } else { Pulse::Low }))
                } else {
                    Ok(None)
                }
            }
            Machine::Conjunction {
                state,
                destinations: _,
            } => {
                state.insert(signal.src.clone(), signal.pulse.clone());

                // No need to check all signals
                if signal.pulse == Pulse::Low {
                    return Ok(Some(Pulse::High));
                }

                let all_high = state.iter().all(|(_, pulse)| pulse == &Pulse::High);
                Ok(Some(if all_high { Pulse::Low } else { Pulse::High }))
            }
        }
    }
}
