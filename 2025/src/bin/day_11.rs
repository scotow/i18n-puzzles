i18n_puzzles_2025::main!();

fn run(input: &str) -> usize {
    input
        .lines()
        .flat_map(|l| {
            (0..24).find_map(|n| {
                let r = rotate(l, n);
                ["Οδυσσευς", "Οδυσσεως", "Οδυσσει", "Οδυσσεα", "Οδυσσευ"]
                    .into_iter()
                    .any(|u| r.contains(u))
                    .then_some(n)
            })
        })
        .sum()
}

fn rotate(input: &str, n: usize) -> String {
    let out = input
        .chars()
        .map(|mut c| match c {
            'Α'..='Ω' => "ΑΒΓΔΕΖΗΘΙΚΛΜΝΞΟΠΡΣΤΥΦΧΨΩ"
                .chars()
                .nth(
                    ("ΑΒΓΔΕΖΗΘΙΚΛΜΝΞΟΠΡΣΤΥΦΧΨΩ"
                        .chars()
                        .position(|c2| c2 == c)
                        .unwrap()
                        + n)
                        % 24,
                )
                .unwrap(),
            'α'..='ω' => {
                if c == 'ς' {
                    c = 'σ';
                }
                "αβγδεζηθικλμνξοπρστυφχψω"
                    .chars()
                    .nth(
                        ("αβγδεζηθικλμνξοπρστυφχψω"
                            .chars()
                            .position(|c2| c2 == c)
                            .unwrap()
                            + n)
                            % 24,
                    )
                    .unwrap()
            }
            _ => c,
        })
        .collect::<String>();
    out.chars()
        .tuple_windows()
        .map(|(c1, c2)| {
            if c1 == 'σ' && !c2.is_alphabetic() {
                'ς'
            } else {
                c1
            }
        })
        .chain(out.chars().last())
        .collect()
}
