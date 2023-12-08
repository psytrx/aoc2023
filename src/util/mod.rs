pub fn parse_space_separated_numbers<T: std::str::FromStr>(line: &str) -> anyhow::Result<Vec<T>>
where
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    line.split_whitespace()
        .map(|s| {
            s.parse::<T>()
                .map_err(|err| anyhow::anyhow!("Failed to parse number list: '{}': {}", line, err))
        })
        .collect::<anyhow::Result<Vec<T>>>()
}

pub fn solve_quadratic_equation(a: f64, b: f64, c: f64) -> QuadraticSolution {
    let d = b.powi(2) - 4.0 * a * c;
    if d < 0.0 {
        QuadraticSolution::None
    } else if d == 0.0 {
        QuadraticSolution::OneRoot(-b / (2.0 * a))
    } else {
        let d_sqrt = d.sqrt();

        let root_1 = (-b - d_sqrt) / (2.0 * a);
        let root_2 = (-b + d_sqrt) / (2.0 * a);

        let lo = root_1.min(root_2);
        let hi = root_1.max(root_2);

        QuadraticSolution::TwoRoots(lo, hi)
    }
}

pub enum QuadraticSolution {
    None,
    OneRoot(f64),
    TwoRoots(f64, f64),
}
