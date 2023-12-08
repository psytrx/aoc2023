use crate::util::parse_space_separated_numbers;

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let almanac = parse_input(input)?;
    Ok(solve(
        &almanac,
        &almanac
            .seeds
            .iter()
            .map(|&seed| Range::new(seed, seed))
            .collect::<Vec<_>>(),
    )?
    .to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let almanac = parse_input(input)?;
    Ok(solve(
        &almanac,
        &almanac
            .seeds
            .chunks(2)
            .map(|chunk| match chunk {
                &[lo, len] => Ok(Range::new(lo, lo + len - 1)),
                _ => anyhow::bail!("Failed to get chunk of 2 from {:?}", chunk),
            })
            .collect::<anyhow::Result<Vec<_>>>()?,
    )?
    .to_string())
}

fn solve(almanac: &Almanac, seeds: &[Range]) -> Result<i64, anyhow::Error> {
    let splits = transform_splits(&almanac.maps, seeds);
    splits
        .into_iter()
        .map(|split| split.lo)
        .min()
        .ok_or_else(|| anyhow::anyhow!("Failed to get min of empty iterator"))
}

fn transform_splits(maps: &[Vec<MappedRange>], seeds: &[Range]) -> Vec<Range> {
    let mut splits = seeds.to_owned();
    for map in maps.iter() {
        let mut active_splits = splits.clone();
        let mut new_splits = Vec::with_capacity(splits.len());

        while let Some(split) = active_splits.pop() {
            let mut was_mapped = false;

            for mapped_range in map.iter() {
                let split_lo_inside = mapped_range.range.contains(split.lo);
                let split_hi_inside = mapped_range.range.contains(split.hi);
                let around =
                    split.contains(mapped_range.range.lo) && split.contains(mapped_range.range.hi);

                if split_lo_inside && split_hi_inside {
                    // full overlap

                    // shift full split
                    let lo = split.lo + mapped_range.offset;
                    let hi = split.hi + mapped_range.offset;
                    new_splits.push(Range::new(lo, hi));

                    // mapped ranges can't overlap, so we are done with this split
                    was_mapped = true;
                    break;
                } else if split_lo_inside {
                    // left overlap

                    // split off & shift left
                    let lo = split.lo + mapped_range.offset;
                    let hi = mapped_range.range.hi + mapped_range.offset;
                    new_splits.push(Range::new(lo, hi));

                    // shorten right split, still needs to be handled
                    let lo = mapped_range.range.hi + 1;
                    let hi = split.hi;
                    active_splits.push(Range::new(lo, hi));

                    was_mapped = true;
                } else if split_hi_inside {
                    // right overlap

                    // shorten left split, still needs to be handled
                    let lo = split.lo;
                    let hi = mapped_range.range.lo - 1;
                    active_splits.push(Range::new(lo, hi));

                    // split off & shift right
                    let lo = mapped_range.range.lo + mapped_range.offset;
                    let hi = split.hi + mapped_range.offset;
                    new_splits.push(Range::new(lo, hi));

                    was_mapped = true;
                } else if around {
                    // mapped range is fully inside the split

                    // split off left outer, still needs to be handled by other ranges
                    let lo = split.lo;
                    let hi = mapped_range.range.lo - 1;
                    active_splits.push(Range::new(lo, hi));

                    // split off & shift inner
                    let lo = mapped_range.range.lo + mapped_range.offset;
                    let hi = mapped_range.range.hi + mapped_range.offset;
                    new_splits.push(Range::new(lo, hi));

                    // split off right outer, still needs to be handled by other ranges
                    let lo = mapped_range.range.hi + 1;
                    let hi = split.hi;
                    active_splits.push(Range::new(lo, hi));

                    was_mapped = true;
                } else {
                    // no overlap, ignore
                }
            }

            if !was_mapped {
                // no mapped ranges, copy split to next map
                new_splits.push(split);
            }
        }

        splits = merge_ranges(new_splits);
    }

    splits
}

fn merge_ranges(mut ranges: Vec<Range>) -> Vec<Range> {
    if ranges.is_empty() {
        return vec![];
    }

    // Sort the ranges based on their starting points
    ranges.sort_by_key(|r| r.lo);

    let mut merged = Vec::new();
    let mut current = ranges[0].clone();

    for range in ranges.into_iter().skip(1) {
        if range.lo <= current.hi {
            // Extend the current range if there's an overlap
            current.hi = current.hi.max(range.hi);
        } else {
            // No overlap, add the current range to the merged list and start a new range
            merged.push(current);
            current = range;
        }
    }

    // Add the last range
    merged.push(current);

    merged
}

fn parse_input(input: &str) -> anyhow::Result<Almanac> {
    input.lines().try_fold(Almanac::EMPTY, |mut almanac, line| {
        if let Some(seeds) = line.strip_prefix("seeds: ") {
            almanac.seeds = parse_space_separated_numbers::<i64>(seeds)?;
            Ok(almanac)
        } else if line.ends_with(" map:") {
            almanac.maps.push(vec![]);
            Ok(almanac)
        } else if line.is_empty() {
            Ok(almanac)
        } else if let Ok(&[dst, src, len]) = parse_space_separated_numbers::<i64>(line).as_deref() {
            if let Some(last_map) = almanac.maps.last_mut() {
                let mapped_range = MappedRange {
                    range: Range::new(src, src + len - 1),
                    offset: dst - src,
                };
                last_map.push(mapped_range);
                Ok(almanac)
            } else {
                anyhow::bail!("Failed to get last map in empty almanac");
            }
        } else {
            anyhow::bail!("Failed to parse input line: '{}'", line);
        }
    })
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<i64>,
    maps: Vec<Vec<MappedRange>>,
}

impl Almanac {
    const EMPTY: Self = Self {
        seeds: Vec::new(),
        maps: Vec::new(),
    };
}

#[derive(Debug)]
struct MappedRange {
    range: Range,
    offset: i64,
}

#[derive(Debug, Clone)]
struct Range {
    lo: i64,
    hi: i64,
}

impl Range {
    fn new(lo: i64, hi: i64) -> Self {
        Self { lo, hi }
    }

    fn contains(&self, value: i64) -> bool {
        self.lo <= value && value <= self.hi
    }
}
