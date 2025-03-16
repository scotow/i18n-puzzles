i18n_puzzles_2025::main!();

fn run(input: &str) -> usize {
    let (hashes, tries) = input.split_once("\n\n").unwrap();
    let hashes = hashes
        .lines()
        .map(|l| l.split_once(' ').unwrap())
        .collect::<HashMap<_, _>>();
    let mut valids = HashMap::new();
    tries
        .lines()
        .filter(|l| {
            let (user, pass) = l.split_once(' ').unwrap();
            let flatten = Decompositions::new_canonical(pass.chars()).collect::<String>();
            if let Some(prev) = valids.get(user) {
                return &flatten == prev;
            }
            if is_valid(pass, hashes[user]) {
                valids.insert(user, flatten);
                true
            } else {
                false
            }
        })
        .count()
}

fn is_valid(input: &str, hash: &str) -> bool {
    let mut combis = vec![Vec::new()];
    let mut i = 0;
    while i < input.chars().count() {
        let mut news = HashSet::new();
        if i < input.chars().count() - 1 {
            if let Some(merged) = compose(
                input.chars().nth(i).unwrap(),
                input.chars().nth(i + 1).unwrap(),
            ) {
                news.insert(vec![merged]);
                news.insert(vec![
                    input.chars().nth(i).unwrap(),
                    input.chars().nth(i + 1).unwrap(),
                ]);
                i += 2;
            } else {
                news.insert(vec![input.chars().nth(i).unwrap()]);
                news.insert(
                    Decompositions::new_canonical(input.chars().nth(i).into_iter()).collect(),
                );
                i += 1;
            }
        } else {
            news.insert(vec![input.chars().nth(i).unwrap()]);
            news.insert(Decompositions::new_canonical(input.chars().nth(i).into_iter()).collect());
            i += 1;
        }
        let backup = combis.clone();
        combis = Vec::new();
        for n in &news {
            for b in &backup {
                combis.push([b.clone(), n.clone()].concat());
            }
        }
    }
    combis
        .iter()
        .any(|c| bcrypt::verify(&c.iter().collect::<String>(), hash).unwrap())
}
