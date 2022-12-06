use itertools::Itertools;

fn main() {
    const WINDOW_WIDTH: usize = 4;

    let data = include_bytes!("input.txt");

    let data_signal_start_pos = data
        .windows(WINDOW_WIDTH)
        .enumerate()
        .find(|w| w.1.iter().all_unique())
        .map(|(idx, _)| idx + WINDOW_WIDTH)
        .unwrap();

    println!("{data_signal_start_pos}");
}
