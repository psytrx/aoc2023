mod problems;
mod util;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = <Args as clap::Parser>::parse();

    let mut total_duration = std::time::Duration::ZERO;

    let days_to_run = days_to_run(args.force_all);
    if days_to_run.is_empty() {
        log::warn!("No problems to run. Did you forget to create input files?");
    } else {
        for _ in 0..args.n {
            for &day in days_to_run.iter() {
                let (input, one, two) = load_problem_set(day)?;

                let answers = if args.validate {
                    load_answers(day)?
                } else {
                    (None, None)
                };

                total_duration +=
                    run_solution(day, 1, &input, one, args.show_solutions, answers.0)?;
                total_duration +=
                    run_solution(day, 2, &input, two, args.show_solutions, answers.1)?;
            }
        }
    }

    log::debug!("Total duration: {:?}", total_duration);

    Ok(())
}

fn load_problem_set(day: i32) -> anyhow::Result<(String, SolutionFn, SolutionFn)> {
    let (one, two): (SolutionFn, SolutionFn) = match day {
        0 => (problems::day_00::part_one, problems::day_00::part_two),
        1 => (problems::day_01::part_one, problems::day_01::part_two),
        2 => (problems::day_02::part_one, problems::day_02::part_two),
        3 => (problems::day_03::part_one, problems::day_03::part_two),
        4 => (problems::day_04::part_one, problems::day_04::part_two),
        5 => (problems::day_05::part_one, problems::day_05::part_two),
        6 => (problems::day_06::part_one, problems::day_06::part_two),
        7 => (problems::day_07::part_one, problems::day_07::part_two),
        8 => (problems::day_08::part_one, problems::day_08::part_two),
        9 => (problems::day_09::part_one, problems::day_09::part_two),
        10 => (problems::day_10::part_one, problems::day_10::part_two),
        _ => anyhow::bail!("No problem set mapped for day {}", day),
    };

    let input = {
        let path = format!("./input/{:02}.txt", day);
        std::fs::read_to_string(path)?
    };

    Ok((input, one, two))
}

fn run_solution(
    day: i32,
    part: i32,
    input: &str,
    f: SolutionFn,
    show_solution: bool,
    expected_answer: Option<String>,
) -> anyhow::Result<std::time::Duration> {
    let (answer, duration) = {
        measure_time::debug_time!("day {:02}/part {}", day, part);

        let t0 = std::time::Instant::now();
        let answer = f(input)?;
        let duration = t0.elapsed();

        (answer, duration)
    };

    if show_solution {
        log::info!("day {:02}/part {} => {}", day, part, answer);
    }

    if let Some(expected) = expected_answer {
        if answer != expected {
            log::error!(
                "day {:02}/part {} => expected: {}, actual: {}",
                day,
                part,
                expected,
                answer
            );
        }
    }

    Ok(duration)
}

fn load_answers(day: i32) -> anyhow::Result<(Option<String>, Option<String>)> {
    let path = format!("./answers/{:02}.txt", day);
    let contents = std::fs::read_to_string(path);
    contents.map_or(Ok((None, None)), |contents| {
        let non_empty_lines: Vec<_> = contents
            .lines()
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .collect();
        match non_empty_lines.as_slice() {
            [] => Ok((None, None)),
            [one] => Ok((Some(one.to_string()), None)),
            [one, two] => Ok((Some(one.to_string()), Some(two.to_string()))),
            _ => anyhow::bail!("Invalid answer file for day {}", day),
        }
    })
}

type SolutionFn = fn(&str) -> anyhow::Result<String>;

fn days_to_run(force_all: bool) -> Vec<i32> {
    let days_to_run: Vec<_> = {
        let days_with_input_file = (1..=25).filter(|day| {
            let input_path = format!("./input/{:02}.txt", day);
            std::fs::metadata(input_path).is_ok()
        });

        if force_all {
            days_with_input_file.collect()
        } else {
            days_with_input_file
                .filter(|day| {
                    let answer_path = format!("./answers/{:02}.txt", day);
                    let answer_count = std::fs::read_to_string(answer_path)
                        .unwrap_or_default()
                        .lines()
                        .filter(|line| !line.is_empty() && !line.starts_with('#'))
                        .count();
                    answer_count < 2
                })
                .collect()
        }
    };
    days_to_run
}

#[derive(clap::Parser, Debug)]
struct Args {
    /// Runs all solutions, even if they have been solved already
    #[arg(short, long = "force-all")]
    force_all: bool,

    /// Number of times to run all solutions. Used for benchmarking
    #[arg(short, long, default_value = "1")]
    n: u32,

    /// Validates the solutions agains the answers in the /answers directory
    #[arg(short, long)]
    validate: bool,

    /// Prints solutions to stdout
    #[arg(short, long = "show-solutions")]
    show_solutions: bool,
}
