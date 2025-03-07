i18n_puzzles_2025::main!();

fn run(input: &str) -> usize {
    input
        .lines()
        .map(|l| ((l.len() <= 160) as usize * 11 + (l.chars().count() <= 140) as usize * 7).min(13))
        .sum()
}
