i18n_puzzles_2025::main!();

fn run(input: &str) -> i64 {
    input
        .split("\n\n")
        .map(|t| {
            t.lines()
                .map(|l| {
                    Zoned::strptime(" %Q %b %d, %Y, %R", l.split_once(':').unwrap().1).unwrap()
                })
                .tuple_windows()
                .map(|(d1, d2)| d1.duration_until(&d2))
                .sum::<SignedDuration>()
        })
        .sum::<SignedDuration>()
        .as_mins()
}
