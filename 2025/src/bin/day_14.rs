i18n_puzzles_2025::main!();

fn run(input: &str) -> u128 {
    input
        .lines()
        .map(|l| l.split(" × ").map(parse).product::<u128>())
        .sum::<u128>()
        / 10_890_000_00
}

fn parse(input: &str) -> u128 {
    kanji_number_parser::parse(input.chars().dropping_back(1).collect())
        .unwrap()
        .to_string()
        .parse::<u128>()
        .unwrap()
        * match input.chars().last().unwrap() {
            '毛' => 1,
            '厘' => 10,
            '分' => 100,
            '寸' => 1_000,
            '尺' => 10_000,
            '間' => 60_000,
            '丈' => 100_000,
            '町' => 3_600_000,
            '里' => 129_600_000,
            _ => unreachable!(),
        }
}
