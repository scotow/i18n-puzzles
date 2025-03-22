i18n_puzzles_2025::main!();

fn run(input: &str) -> u64 {
    [english, swedish, dutch]
        .into_iter()
        .map(|f| {
            let sorted = input
                .lines()
                .map(|l| {
                    let (name, number) = l.split_once(": ").unwrap();
                    let (last, first) = name.split_once(", ").unwrap();
                    (last, first, number.parse::<u64>().unwrap())
                })
                .sorted_by(|(l1, f1, _), (l2, f2, _)| {
                    f(l1).cmp(&f(l2)).then_with(|| f(f1).cmp(&f(f2)))
                })
                .collect_vec();
            sorted[sorted.len() / 2].2
        })
        .product()
}

fn english(input: &str) -> Vec<u8> {
    deunicode(input)
        .to_lowercase()
        .replace([' ', '\'', '-'], "")
        .into_bytes()
}

fn swedish(input: &str) -> Vec<u8> {
    input
        .chars()
        .flat_map(|c| {
            let up = c.to_uppercase().to_string();
            assert_eq!(up.chars().count(), 1);
            match c.to_uppercase().to_string().as_str() {
                "Å" => vec![26],
                "Ä" | "Æ" => vec![27],
                "Ö" | "Ø" => vec![28],
                " " | "'" | "-" => Vec::new(),
                _ => deunicode(&up).bytes().map(|c| c - b'A').collect(),
            }
        })
        .collect()
}

fn dutch(input: &str) -> Vec<u8> {
    deunicode(input)
        .trim_start_matches("van ")
        .trim_start_matches("de ")
        .trim_start_matches("den ")
        .trim_start_matches("der ")
        .to_string()
        .into_bytes()
}
