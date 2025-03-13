i18n_puzzles_2025::main!();

fn run(input: &str) -> usize {
    let mut score = 0;
    let (words, grid) = input.split_once("\n\n").unwrap();
    for (i, mut word) in words.lines().map(|w| w.to_owned()).enumerate() {
        if i % 3 == 2 {
            word = decode(&word);
        }
        if i % 5 == 4 {
            word = decode(&word);
        }
        for line in grid.lines().map(str::trim) {
            if line.chars().count() == word.chars().count() {
                let set = line.chars().position(|c| c != '.').unwrap();
                if line.chars().nth(set).unwrap().to_lowercase().eq(word
                    .chars()
                    .nth(set)
                    .unwrap()
                    .to_lowercase())
                {
                    score += i + 1;
                }
            }
        }
    }
    score
}

fn decode(input: &str) -> String {
    from_utf8(&*encoding_rs::mem::encode_latin1_lossy(input))
        .unwrap()
        .to_owned()
}
