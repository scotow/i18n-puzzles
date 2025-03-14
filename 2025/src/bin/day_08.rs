i18n_puzzles_2025::main!();

fn run(input: &str) -> usize {
    input
        .lines()
        .filter(|l| {
            let ascii = deunicode::deunicode(l).to_lowercase();
            (4..=12).contains(&l.chars().count())
                && l.chars().any(|c| c.is_ascii_digit())
                && ascii
                    .chars()
                    .any(|c| ['a', 'e', 'i', 'o', 'u'].contains(&c))
                && ascii
                    .chars()
                    .any(|c| !['a', 'e', 'i', 'o', 'u'].contains(&c) && c.is_alphabetic())
                && ascii.chars().counts().values().all(|&n| n == 1)
        })
        .count()
}
