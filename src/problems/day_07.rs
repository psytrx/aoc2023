use std::fmt::{Debug, Write};

pub fn part_one(input: &str) -> anyhow::Result<String> {
    solve(input, false)
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    solve(input, true)
}

fn solve(input: &str, jokers: bool) -> anyhow::Result<String> {
    let mut data = parse_input(input, jokers)?;
    data.sort_by_key(|(hand, _)| hand.sortable_hash);

    Ok(data
        .into_iter()
        .enumerate()
        .map(|(i, (_, bid))| {
            let rank = (i + 1) as i32;
            bid * rank
        })
        .sum::<i32>()
        .to_string())
}

fn parse_input(input: &str, jokers: bool) -> anyhow::Result<Vec<(Hand, i32)>> {
    input
        .lines()
        .map(|line| {
            let (cards, bid) = line
                .split_once(' ')
                .ok_or_else(|| anyhow::anyhow!("Failed to split line: '{}'", line))?;

            let hand = Hand::from(cards, jokers);
            let bid = bid.parse::<i32>()?;

            Ok((hand, bid))
        })
        .collect::<anyhow::Result<Vec<(Hand, i32)>>>()
}

struct Hand {
    cards: Vec<Card>,
    sortable_hash: i32,
}

impl Hand {
    fn new(cards: Vec<Card>, jokers: bool) -> Self {
        let sortable_hash = {
            let kind = Self::kind(&cards, jokers) as i32;
            cards.iter().fold(kind, |key, card| {
                (key << 4)
                    + if jokers && card.strength == Self::JOKER_STRENGTH {
                        0
                    } else {
                        card.strength
                    }
            })
        };
        Self {
            cards,
            sortable_hash,
        }
    }

    const JOKER_STRENGTH: i32 = 11;

    fn kind(cards: &[Card], jokers: bool) -> HandKind {
        let mut card_counts = [0; 15];
        let mut unique_cards = 0;
        let mut max_count = 0;

        for card in cards.iter() {
            let count = &mut card_counts[card.strength as usize];
            if *count == 0 && (!jokers || card.strength != Self::JOKER_STRENGTH) {
                unique_cards += 1;
            }

            *count += 1;
            if !jokers || card.strength != Self::JOKER_STRENGTH {
                max_count = max_count.max(*count);
            }
        }

        // add jokers
        let max_count = if jokers {
            let n_jokers = card_counts[Self::JOKER_STRENGTH as usize];
            max_count + n_jokers
        } else {
            max_count
        };

        match (unique_cards, max_count) {
            (_, 5) => HandKind::FiveOfAKind,
            (_, 4) => HandKind::FourOfAKind,
            (2, 3) => HandKind::FullHouse,
            (3, 3) => HandKind::ThreeOfAKind,
            (3, 2) => HandKind::TwoPair,
            (4, 2) => HandKind::OnePair,
            (5, 1) => HandKind::HighCard,
            _ => unreachable!(
                "Invalid hand kind: {:?}, unique: {}, max_count: {}, jokers: {}",
                cards, unique_cards, max_count, jokers
            ),
        }
    }

    fn from(cards: &str, jokers: bool) -> Self {
        let cards = cards.chars().map(Card::new).collect::<Vec<_>>();
        Self::new(cards, jokers)
    }
}

impl Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}",
            self.cards.iter().fold(String::new(), |mut output, card| {
                let _ = write!(output, "{card:?}");
                output
            })
        ))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
enum HandKind {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Clone, Copy)]
struct Card {
    label: char,
    strength: i32,
}

impl Card {
    fn new(label: char) -> Self {
        let strength = match label {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            '9' => 9,
            '8' => 8,
            '7' => 7,
            '6' => 6,
            '5' => 5,
            '4' => 4,
            '3' => 3,
            '2' => 2,
            _ => unreachable!("Invalid card: {:?}", label),
        };
        Self { label, strength }
    }
}

impl Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.label))
    }
}
