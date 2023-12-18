pub fn part_one(input: &str) -> anyhow::Result<String> {
    parse_input(input)?;
    Ok("not implemented".to_string())
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

        let value = value.parse::<i32>()?;

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

                    let value = condition[2..].parse::<i32>().map_err(|e| {
                        anyhow::anyhow!("Failed to parse condition value: {}", condition)
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

pub fn part_two(_input: &str) -> anyhow::Result<String> {
    Ok("not implemented".to_string())
}

struct Workflow {
    name: String,
    rules: Vec<WorkflowRule>,
}

struct WorkflowRule {
    condition: Option<Condition>,
    action: Action,
}

struct Condition {
    category: u8,
    operator: u8,
    value: i32,
}

enum Action {
    Send(String),
    Reject,
    Accept,
}

struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}
