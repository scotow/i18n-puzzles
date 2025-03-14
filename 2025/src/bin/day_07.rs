i18n_puzzles_2025::main!();

fn run(input: &str) -> usize {
    let tzs = ["America/Halifax", "America/Santiago"].map(|tz| TimeZone::get(tz).unwrap());
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let (ts_str, correct, wrong) = line.split_whitespace().collect_tuple().unwrap();
            let ts = ts_str.parse::<Timestamp>().unwrap();
            tzs.iter()
                .find_map(|tz| {
                    if tz.preceding(ts).next().unwrap().offset()
                        == Offset::from_hours(
                            ts_str
                                .split(":00.000")
                                .nth(1)
                                .unwrap()
                                .split(':')
                                .next()
                                .unwrap()
                                .parse::<i8>()
                                .unwrap(),
                        )
                        .unwrap()
                    {
                        return Some(
                            (&ts.to_zoned(tz.clone())
                                + Span::new().minutes(
                                    -wrong.parse::<i64>().unwrap()
                                        + correct.parse::<i64>().unwrap(),
                                ))
                            .hour() as usize
                                * (i + 1),
                        );
                    }
                    None
                })
                .unwrap()
        })
        .sum()
}
