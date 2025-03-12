i18n_puzzles_2025::main!();

fn run(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let max_x = grid.iter().map(|l| l.len()).max().unwrap();
    let (mut x, mut y) = (0, 0);
    let mut poops = 0;
    while y < grid.len() {
        if grid[y].get(x) == Some(&'💩') {
            poops += 1;
        }
        x = (x + 2) % max_x;
        y += 1;
    }
    poops
}
