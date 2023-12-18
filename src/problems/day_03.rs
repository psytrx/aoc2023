pub fn part_one(input: &str) -> anyhow::Result<String> {
    let lines = parse_input(input)?;
    Ok(lines
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            let line_lo = i.saturating_sub(1);
            let line_hi = (i + 1).min(lines.len() - 1);

            line.numbers
                .iter()
                .filter(|number| {
                    let col_lo = number.start.saturating_sub(1);
                    let col_hi = number.end + 1;

                    lines[line_lo..=line_hi].iter().any(|line| {
                        line.symbols
                            .iter()
                            .any(|symbol| (col_lo..=col_hi).contains(&symbol.pos))
                    })
                })
                .map(|number| number.number)
                .collect::<Vec<_>>()
        })
        .sum::<i32>()
        .to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let lines = parse_input(input)?;
    Ok(lines
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.symbols
                .iter()
                .filter_map(|symbol| {
                    if symbol.kind == b'*' {
                        let line_lo = i.saturating_sub(1);
                        let line_hi = i + 1;

                        let numbers_around = lines[line_lo..=line_hi]
                            .iter()
                            .flat_map(|line| {
                                line.numbers.iter().filter_map(|number| {
                                    let col_lo = number.start.saturating_sub(1);
                                    let col_hi = number.end + 1;
                                    (col_lo..=col_hi)
                                        .contains(&symbol.pos)
                                        .then_some(number.number)
                                })
                            })
                            .collect::<Vec<_>>();

                        (numbers_around.len() == 2)
                            .then_some(numbers_around.iter().product::<i32>())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .sum::<i32>()
        .to_string())
}

fn parse_input(input: &str) -> anyhow::Result<Vec<SchematicLine>> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> anyhow::Result<SchematicLine> {
    let mut digits = Vec::with_capacity(4);
    let mut symbols = Vec::with_capacity(16);
    let mut numbers = Vec::with_capacity(16);

    line.as_bytes().iter().enumerate().try_for_each(|(i, &c)| {
        let is_digit = c.is_ascii_digit();
        let is_symbol = !is_digit && c != b'.';

        let is_whitespace = !is_digit && !is_symbol;

        if is_digit {
            // append digit to number
            match char::to_digit(c as char, 10) {
                Some(digit) => {
                    digits.push(digit);
                }
                None => anyhow::bail!("Failed to parse digit: '{}'", c),
            }
        }

        if is_symbol {
            // append symbol to symbol span
            symbols.push(Symbol { kind: c, pos: i });
        }

        if (is_whitespace || is_symbol) && !digits.is_empty() {
            // commit number span
            let number = digits.iter().fold(0, |acc, &digit| acc * 10 + digit as i32);
            let start = i - digits.len();
            let end = i - 1;

            numbers.push(NumberSpan { number, start, end });
            digits.clear();
        }

        Ok(())
    })?;

    // append trailing number span
    if !digits.is_empty() {
        let number = digits.iter().fold(0, |acc, &digit| acc * 10 + digit as i32);
        let start = line.len() - digits.len();
        let end = line.len() - 1;
        numbers.push(NumberSpan { number, start, end });
    }

    Ok(SchematicLine { numbers, symbols })
}

struct SchematicLine {
    numbers: Vec<NumberSpan>,
    symbols: Vec<Symbol>,
}

struct NumberSpan {
    number: i32,
    start: usize,
    end: usize,
}

struct Symbol {
    kind: u8,
    pos: usize,
}
