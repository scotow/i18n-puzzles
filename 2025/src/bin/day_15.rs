i18n_puzzles_2025::main!();

fn run(input: &str) -> u32 {
    let (offices, mut clients) = input.split("\n\n").map(parse).collect_tuple().unwrap();
    let start = datetime(2022, 1, 1, 0, 0, 0, 0)
        .to_zoned(TimeZone::UTC)
        .unwrap()
        .timestamp();
    for next in start.series(Span::new().minutes(30)).take(365 * 48) {
        for (ctz, ct_hs, ot) in &mut clients {
            let ct_z = next.to_zoned(ctz.clone());
            if (6..=7).contains(&ct_z.weekday().to_monday_one_offset())
                || ct_hs.contains(&ct_z.date())
                || offices.iter().any(|(tz, o_hs, _)| {
                    let in_tz = next.to_zoned(tz.clone());
                    (1..=5).contains(&in_tz.weekday().to_monday_one_offset())
                        && (time(8, 30, 0, 0)..time(17, 0, 0, 0)).contains(&in_tz.time())
                        && !o_hs.contains(&in_tz.date())
                })
            {
                continue;
            }
            *ot += 1;
        }
    }

    let (min, max) = clients
        .into_iter()
        .map(|(_, _, ot)| ot)
        .minmax()
        .into_option()
        .unwrap();
    (max - min) * 30
}

fn parse(input: &str) -> Vec<(TimeZone, Vec<Date>, u32)> {
    input
        .lines()
        .map(|o| {
            let (_, tz, hs) = o.split('\t').collect_tuple().unwrap();
            (
                TimeZone::get(tz).unwrap(),
                hs.split(';')
                    .map(|d| Date::strptime("%d %B %Y", d).unwrap())
                    .collect(),
                0,
            )
        })
        .collect()
}
