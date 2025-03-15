i18n_puzzles_2025::main!();

fn run(input: &str) -> String {
    input
        .lines()
        .flat_map(|l| {
            let (date, persons) = l.split_once(": ").unwrap();
            persons.split(", ").map(move |p| (p, date))
        })
        .into_group_map()
        .into_iter()
        .filter_map(|(p, dates)| {
            ["%d", "%m", "%y"]
                .into_iter()
                .permutations(3)
                .map(|f| f.join("-"))
                .find_map(|f| {
                    (dates.iter().all(|d| Date::strptime(&f, d).is_ok())
                        && dates
                            .iter()
                            .any(|d| Date::strptime(&f, d).unwrap() == date(2001, 9, 11)))
                    .then_some(p)
                })
        })
        .sorted()
        .join(" ")
}
