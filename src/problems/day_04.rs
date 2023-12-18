use crate::util::parse_space_separated_numbers;

pub fn part_one(input: &str) -> anyhow::Result<String> {
    Ok(parse_input(input)?
        .iter()
        .map(|card| card.points())
        .sum::<u32>()
        .to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let cards = parse_input(input)?;

    let mut copies = vec![1; cards.len()];

    Ok(cards
        .iter()
        .enumerate()
        .fold(0, |card_count, (i, card)| {
            let instances = copies[i];

            let points = card.matching_numbers();
            for copies in copies
                .iter_mut()
                .take((i + points as usize).min(cards.len() - 1) + 1)
                .skip(i + 1)
            {
                *copies += instances;
            }

            card_count + instances
        })
        .to_string())
}

fn parse_input(input: &str) -> anyhow::Result<Vec<ScratchCard>> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> anyhow::Result<ScratchCard> {
    let (_, number_sets) = line
        .split_once(": ")
        .ok_or_else(|| anyhow::anyhow!("Failed to split input line: '{}'", line))?;

    let (winning_numbers, numbers) = number_sets
        .split_once(" | ")
        .ok_or_else(|| anyhow::anyhow!("Failed to split number sets: '{}'", number_sets))?;

    let winning_numbers = parse_space_separated_numbers(winning_numbers)?;
    let numbers = parse_space_separated_numbers(numbers)?;

    Ok(ScratchCard {
        winning_numbers,
        numbers,
    })
}

struct ScratchCard {
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

impl ScratchCard {
    fn matching_numbers(&self) -> u32 {
        self.winning_numbers
            .iter()
            .filter(|wn| self.numbers.contains(wn))
            .count() as u32
    }

    fn points(&self) -> u32 {
        let matches = self.matching_numbers();
        if matches == 0 {
            0
        } else {
            2_u32.pow(matches - 1)
        }
    }
}
