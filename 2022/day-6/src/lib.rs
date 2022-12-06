use itertools::Itertools;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::path::PathBuf;

#[test]
fn methods_yield_the_same() {
    let meth_1 = with_string_baked_in();
    let meth_2 = with_io().unwrap();

    assert_eq!(meth_1, meth_2);
}

pub fn with_string_baked_in() -> usize {
    const WINDOW_WIDTH: usize = 14;

    let data = include_bytes!("input.txt");

    let data_signal_start_pos = data
        .windows(WINDOW_WIDTH)
        .enumerate()
        .find(|w| w.1.iter().all_unique())
        .map(|(idx, _)| idx + WINDOW_WIDTH)
        .unwrap();

    data_signal_start_pos
}

pub fn with_io() -> io::Result<usize> {
    const WINDOW_WIDTH: u64 = 14;

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/input.txt");

    let mut f = File::open(path)?;
    let mut buf = [0; 14];

    let mut pre_seek_pos = f.seek(SeekFrom::Start(0))?;

    while !buf.iter().all_unique() {
        f.read(&mut buf[..])?;
        pre_seek_pos = f.seek(SeekFrom::Start(pre_seek_pos + 1))?;
    }

    Ok((WINDOW_WIDTH + pre_seek_pos - 1) as usize)
}
