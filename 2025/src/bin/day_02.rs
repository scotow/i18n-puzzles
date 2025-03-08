i18n_puzzles_2025::main!();

fn run(input: &str) -> String {
    input
        .lines()
        .map(|l| l.parse::<Timestamp>().unwrap())
        .counts()
        .into_iter()
        .find_map(|(k, v)| (v == 4).then_some(k))
        .unwrap()
        .strftime("%FT%T%:z")
        .to_string()
}
