i18n_puzzles_2025::main!();

fn run(input: &str) -> usize {
    let (encoded, grid) = input.split_once("\n\n").unwrap();
    let mut res = 0;
    'outer: for (i, line) in encoded.lines().enumerate() {
        let bytes = hex::decode(line).unwrap();
        for enc in [
            encoding_rs::UTF_8,
            encoding_rs::UTF_16BE,
            encoding_rs::UTF_16LE,
            encoding_rs::WINDOWS_1252,
        ] {
            let (decoded, _, errors) = enc.decode(&bytes);
            if errors
                || decoded
                    .chars()
                    .any(|c| !c.is_alphabetic() || !c.is_lowercase())
            {
                continue;
            }
            for g in grid.lines().map(str::trim) {
                let placed = g.chars().position(|c| c != '.').unwrap();
                if g.chars().count() == decoded.chars().count()
                    && g.chars().nth(placed).unwrap() == decoded.chars().nth(placed).unwrap()
                {
                    res += i + 1;
                    continue 'outer;
                }
            }
        }
    }
    res
}
