i18n_puzzles_2025::main!();

fn run(input: &str) -> usize {
    input
        .lines()
        .filter(|l| {
            (4..=12).contains(&l.chars().count())
                && l.chars().any(|c| c.is_ascii_digit())
                && l.chars().any(|c| c.is_uppercase())
                && l.chars().any(|c| c.is_lowercase())
                && l.chars().any(|c| !c.is_ascii())
        })
        .count()
}
