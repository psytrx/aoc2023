pub fn part_one(input: &str) -> anyhow::Result<String> {
    let (workflows, parts) = parse_input(input)?;
    let accepted_parts = sort_parts(&workflows, &parts)?;
    let sum = accepted_parts
        .iter()
        .map(|part| part.x + part.m + part.a + part.s)
        .sum::<u64>();
    Ok(sum.to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let (workflows, _) = parse_input(input)?;

    let workflow_map = workflows
        .iter()
        .map(|workflow| (workflow.name.clone(), workflow))
        .collect::<hashbrown::hash_map::HashMap<_, _>>();

    let workflow = workflow_map
        .get("in")
        .ok_or_else(|| anyhow::anyhow!("Failed to find workflow 'in'"))?;

    let ranges = RangeSet {
        x: 1..4001,
        m: 1..4001,
        a: 1..4001,
        s: 1..4001,
    };

    let combinations = count_combinations(&workflow_map, workflow, ranges)?;

    Ok(combinations.to_string())
}

fn count_combinations(
    workflow_map: &hashbrown::HashMap<String, &Workflow>,
    workflow: &Workflow,
    mut ranges: RangeSet,
) -> anyhow::Result<u64> {
    let mut count = 0;

    for rule in workflow.rules.iter() {
        // TODO: eagerly cancel on empty ranges

        let (included, rest) = split_range_set(&ranges, &rule.condition);

        match &rule.action {
            Action::Reject => {
                // Drop combinations inside the range,
                // continue with the rest
                ranges = rest;
            }
            Action::Accept => {
                // Count combinations inside the range,
                // continue with the rest
                count += included.size();
                ranges = rest;
            }
            Action::Send(target_workflow) => {
                // Send combinations to next workflow,
                // continue with the rest
                let next_workflow = workflow_map.get(target_workflow).ok_or_else(|| {
                    anyhow::anyhow!("Failed to find target workflow '{}'", target_workflow)
                })?;
                count += count_combinations(workflow_map, next_workflow, included)?;
                ranges = rest;
            }
        }
    }

    Ok(count)
}

fn split_range_set(ranges: &RangeSet, condition: &Option<Condition>) -> (RangeSet, RangeSet) {
    match condition {
        None => (ranges.clone(), RangeSet::EMPTY),
        Some(condition) => match condition.operator {
            b'<' => split_range_set_before(ranges, condition.value, condition.category),
            b'>' => {
                let (rest, included) =
                    split_range_set_before(ranges, condition.value + 1, condition.category);
                (included, rest)
            }
            _ => unreachable!(),
        },
    }
}

fn split_range_set_before(ranges: &RangeSet, before: u64, category: u8) -> (RangeSet, RangeSet) {
    let front = RangeSet {
        x: if category == b'x' {
            ranges.x.start..before
        } else {
            ranges.x.clone()
        },
        m: if category == b'm' {
            ranges.m.start..before
        } else {
            ranges.m.clone()
        },
        a: if category == b'a' {
            ranges.a.start..before
        } else {
            ranges.a.clone()
        },
        s: if category == b's' {
            ranges.s.start..before
        } else {
            ranges.s.clone()
        },
    };

    let back = RangeSet {
        x: if category == b'x' {
            before..ranges.x.end
        } else {
            ranges.x.clone()
        },
        m: if category == b'm' {
            before..ranges.m.end
        } else {
            ranges.m.clone()
        },
        a: if category == b'a' {
            before..ranges.a.end
        } else {
            ranges.a.clone()
        },
        s: if category == b's' {
            before..ranges.s.end
        } else {
            ranges.s.clone()
        },
    };

    (front, back)
}

fn sort_parts(workflows: &[Workflow], parts: &[Part]) -> anyhow::Result<Vec<Part>> {
    let mut accepted = Vec::with_capacity(parts.len());

    let workflow_map = workflows
        .iter()
        .map(|w| (w.name.clone(), w))
        .collect::<hashbrown::hash_map::HashMap<_, _>>();

    let start_workflow = workflow_map
        .get("in")
        .ok_or_else(|| anyhow::anyhow!("Failed to find workflow 'in'"))?;

    for part in parts.iter() {
        let action = apply_workflows(part, &workflow_map, start_workflow)?;
        match action {
            Action::Accept => {
                accepted.push(part.clone());
            }
            Action::Reject => {}
            Action::Send(_) => unreachable!(),
        }
    }

    Ok(accepted)
}

fn apply_workflows(
    part: &Part,
    workflow_map: &hashbrown::HashMap<String, &Workflow>,
    start_workflow: &Workflow,
) -> anyhow::Result<Action> {
    let mut workflow = start_workflow;

    loop {
        let action = apply_workflow(part, workflow);
        match action {
            Action::Send(workflow_name) => {
                workflow = workflow_map
                    .get(&workflow_name)
                    .ok_or_else(|| anyhow::anyhow!("Failed to find workflow {}", workflow_name))?;
            }
            _ => return Ok(action),
        }
    }
}

fn apply_workflow(part: &Part, workflow: &Workflow) -> Action {
    for rule in workflow.rules.iter() {
        if let Some(condition) = &rule.condition {
            let operand = match condition.category {
                b'x' => part.x,
                b'm' => part.m,
                b'a' => part.a,
                b's' => part.s,
                _ => unreachable!(),
            };

            let result = match condition.operator {
                b'<' => operand < condition.value,
                b'>' => operand > condition.value,
                _ => unreachable!(),
            };

            if result {
                return rule.action.clone();
            }
        } else {
            return rule.action.clone();
        }
    }
    unreachable!()
}

fn parse_input(input: &str) -> anyhow::Result<(Vec<Workflow>, Vec<Part>)> {
    let mut workflows = Vec::with_capacity(512);
    let mut parts = Vec::with_capacity(256);

    for line in input.lines() {
        if line.is_empty() {
            // noop
        } else if line.starts_with('{') {
            parts.push(parse_part(line)?);
        } else {
            workflows.push(parse_workflow(line)?);
        }
    }

    Ok((workflows, parts))
}

fn parse_part(line: &str) -> anyhow::Result<Part> {
    let attributes = line
        .strip_prefix('{')
        .ok_or_else(|| anyhow::anyhow!("Failed to strip part prefix {{"))?
        .strip_suffix('}')
        .ok_or_else(|| anyhow::anyhow!("Failed to strip part suffix }}"))?;

    let mut part = Part {
        x: 0,
        m: 0,
        a: 0,
        s: 0,
    };

    for attr in attributes.split(',') {
        let (key, value) = attr
            .split_once('=')
            .ok_or_else(|| anyhow::anyhow!("Failed to split attribute key/value"))?;

        let value = value.parse::<u64>()?;

        match key {
            "x" => part.x = value,
            "m" => part.m = value,
            "a" => part.a = value,
            "s" => part.s = value,
            _ => anyhow::bail!("Invalid attribute key: {}", key),
        }
    }

    Ok(part)
}

fn parse_workflow(line: &str) -> anyhow::Result<Workflow> {
    let (name, rest) = line
        .split_once('{')
        .ok_or_else(|| anyhow::anyhow!("Failed to split workflow rule"))?;

    let name = name.to_string();

    let rules = rest
        .strip_suffix('}')
        .ok_or_else(|| anyhow::anyhow!("Failed to strip off }} suffix"))?
        .split(',')
        .map(|rule| {
            let (condition, workflow_action) =
                if let Some((condition, target_workflow)) = rule.split_once(':') {
                    let chars = condition.as_bytes();
                    let category = chars[0];
                    let operator = chars[1];

                    let value = condition[2..].parse::<u64>().map_err(|e| {
                        anyhow::anyhow!("Failed to parse condition value '{}': {}", condition, e)
                    })?;

                    (
                        Some(Condition {
                            category,
                            operator,
                            value,
                        }),
                        target_workflow.to_string(),
                    )
                } else {
                    (None, rule.to_string())
                };

            let action = parse_action(&workflow_action);
            Ok(WorkflowRule { condition, action })
        })
        .collect::<anyhow::Result<_>>()?;

    Ok(Workflow { name, rules })
}

fn parse_action(action: &str) -> Action {
    match action {
        "A" => Action::Accept,
        "R" => Action::Reject,
        name => Action::Send(name.to_string()),
    }
}

struct Workflow {
    name: String,
    rules: Vec<WorkflowRule>,
}

struct WorkflowRule {
    condition: Option<Condition>,
    action: Action,
}

#[derive(Clone)]
struct Condition {
    category: u8,
    operator: u8,
    value: u64,
}

#[derive(Clone)]
enum Action {
    Send(String),
    Reject,
    Accept,
}

#[derive(Debug, Clone)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

#[derive(Clone)]
struct RangeSet {
    x: std::ops::Range<u64>,
    m: std::ops::Range<u64>,
    a: std::ops::Range<u64>,
    s: std::ops::Range<u64>,
}

impl RangeSet {
    const EMPTY: Self = Self {
        x: 0..0,
        m: 0..0,
        a: 0..0,
        s: 0..0,
    };

    fn size(&self) -> u64 {
        (self.x.end - self.x.start).max(0)
            * (self.m.end - self.m.start).max(0)
            * (self.a.end - self.a.start).max(0)
            * (self.s.end - self.s.start).max(0)
    }
}
