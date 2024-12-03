const XMAS: &[u8] = "XMAS".as_bytes();
const SAMX: &[u8] = "SAMX".as_bytes();
const MAS: &[u8] = "MAS".as_bytes();
const SAM: &[u8] = "SAM".as_bytes();

const XMAS_LEN: usize = XMAS.len();
const MAS_LEN: usize = MAS.len();

pub fn count_x_mas(matrix: &[&[u8]]) -> u32 {
    let mut count = 0;

    for y in 0..matrix.len() {
        if y > matrix.len() - MAS_LEN {
            break;
        }

        for x in 0..matrix[y].len() {
            if x > matrix[y].len() - MAS_LEN {
                break;
            }

            let candidate = matrix[y..y+3].iter().map(|row| &row[x..x+3]).collect::<Vec<_>>();

            if check_3_by_3(&candidate) {
                count += 1;
            }
        }
    }

    count
}

fn check_3_by_3(matrix: &[&[u8]]) -> bool {
    let dr_candidate = matrix[0..3].iter().enumerate().map(|(i, &c)| c[i]).collect::<Vec<_>>();
    // dbg!(String::from_utf8(dr_candidate.clone()));
    let dl_candidate = matrix[0..3].iter().enumerate().map(|(i, &c)| c[2 - i]).collect::<Vec<_>>();

    (dr_candidate == MAS || dr_candidate == SAM) && (dl_candidate == MAS || dl_candidate == SAM)
}

pub fn count_xmas(matrix: &[&[u8]]) -> u32 {
    let mut count = 0;

    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            if check_right(matrix, (x, y)) {
                count += 1;
            }

            if check_down(matrix, (x, y)) {
                count += 1;
            }

            if check_diag_down_right(matrix, (x, y)) {
                count += 1;
            }

            if check_left(matrix, (x, y)) {
                count += 1;
            }

            if check_up(matrix, (x, y)) {
                count += 1;
            }

            if check_diag_up_left(matrix, (x, y)) {
                count += 1;
            }

            if check_diag_up_right(matrix, (x, y)) {
                count += 1;
            }

            if check_diag_down_left(matrix, (x, y)) {
                count += 1;
            }
        }
    };

    count
}

fn check_right(matrix: &[&[u8]], (x, y): (usize, usize)) -> bool {
    // matrix is 10
    // xmas len is 4
    // x = 6 is ok
    // x = 7 is oob
    if x > matrix[y].len() - XMAS_LEN {
        return false;
    }

    &matrix[y][x..x+XMAS_LEN] == XMAS
}

fn check_left(matrix: &[&[u8]], (x, y): (usize, usize)) -> bool {
    // xmas len - 1 is 3
    // x is 3, 3 - 3 ok
    // x is 2, 2 - 3 is underflow
    if x < XMAS_LEN - 1 {
        return false;
    }

    &matrix[y][x-3..x+1] == SAMX
}

fn check_up(matrix: &[&[u8]], (x, y): (usize, usize)) -> bool {
    // xmas len - 1 is 3
    // y is 3, 3 - 3 ok
    // y is 2, 2 - 3 is underflow
    if y < XMAS_LEN - 1 {
        return false;
    }

    let candidate = &matrix[y-3..y+1];
    let x = candidate.iter().map(|&c| c[x]).collect::<Vec<_>>();

    &x == SAMX
}

fn check_down(matrix: &[&[u8]], (x, y): (usize, usize)) -> bool {
    // matrix len is 10
    // xmas len is 4
    // y = 6 ok
    // y = 7 oob
    if y > matrix.len() - XMAS_LEN {
        return false;
    }

    let candidate = matrix[y..y+XMAS_LEN].iter().map(|&c| c[x]).collect::<Vec<_>>();

    &candidate == XMAS
}

fn check_diag_down_right(matrix: &[&[u8]], (x, y): (usize, usize)) -> bool {
    // CHECK DOWN
    if y > matrix.len() - XMAS_LEN {
        return false;
    }

    // CHECK RIGHT
    // matrix row is 10
    // xmas is 4
    // x is 6
    // 6 + 3 = 9, 9 < 10 ok
    if x > matrix[y + 3].len() - XMAS_LEN {
        return false;
    }

    let candidate = matrix[y..y+XMAS_LEN].iter().enumerate().map(|(i, &c)| c[x + i]).collect::<Vec<_>>();
    &candidate == XMAS
}

fn check_diag_down_left(matrix: &[&[u8]], (x, y): (usize, usize)) -> bool {
    // CHECK DOWN
    if y > matrix.len() - XMAS_LEN {
        return false;
    }

    // CHECK LEFT
    if x < XMAS_LEN - 1{
        return false;
    }

    let candidate = matrix[y..y+4].iter().enumerate().map(|(i, &c)| c[x - i]).collect::<Vec<_>>();
    &candidate == XMAS
}

fn check_diag_up_right(matrix: &[&[u8]], (x, y): (usize, usize)) -> bool {
    // CHECK UP
    if y < XMAS_LEN - 1 {
        return false;
    }

    // CHECK RIGHT
    if x > matrix[y - 3].len() - XMAS_LEN {
        return false;
    }

    let candidate = matrix[y-3..y+1].iter().enumerate().map(|(i, &c)| c[x + XMAS_LEN - 1 - i]).collect::<Vec<_>>();
    &candidate == SAMX
}

fn check_diag_up_left(matrix: &[&[u8]], (x, y): (usize, usize)) -> bool {
    // CHECK UP
    if y < XMAS_LEN - 1 {
        return false;
    }

    // CHECK LEFT
    if x < XMAS_LEN - 1 {
        return false;
    }

    let candidate = matrix[y-3..y+1].iter().enumerate().map(|(i, &c)| c[x - (XMAS_LEN - 1) + i]).collect::<Vec<_>>();
    &candidate == SAMX
}

#[cfg(test)]
mod test {
    use std::sync::LazyLock;

    use test_case::test_case;
    use crate::day4::{check_3_by_3, check_diag_down_left, check_diag_down_right, check_diag_up_left, check_diag_up_right, check_down, check_left, check_right, check_up, count_x_mas};

    use super::count_xmas;

    pub static SAMPLE: LazyLock<Vec<&[u8]>> = LazyLock::new(|| vec![
            "MMMSXXMASM".as_bytes(),
            "MSAMXMSMSA".as_bytes(),
            "AMXSXMAAMM".as_bytes(),
            "MSAMASMSMX".as_bytes(),
            "XMASAMXAMM".as_bytes(),
            "XXAMMXXAMA".as_bytes(),
            "SMSMSASXSS".as_bytes(),
            "SAXAMASAAA".as_bytes(),
            "MAMMMXMMMM".as_bytes(),
            "MXMXAXMASX".as_bytes(),
        ]
    );

    #[test]
    fn test_count_xmas() {
        let actual = count_xmas(&SAMPLE);
        assert_eq!(actual, 18);
    }

    #[test]
    fn test_count_x_mas() {
        let actual = count_x_mas(&SAMPLE);
        assert_eq!(actual, 9);
    }

    #[test_case(vec![
        "M.M".as_bytes(),
        ".A.".as_bytes(),
        "S.S".as_bytes(),
    ], true)]
    #[test_case(vec![
        "S.M".as_bytes(),
        ".A.".as_bytes(),
        "S.M".as_bytes(),
    ], true)]
    #[test_case(vec![
        "S.S".as_bytes(),
        ".A.".as_bytes(),
        "M.M".as_bytes(),
    ], true)]
    #[test_case(vec![
        "M.S".as_bytes(),
        ".A.".as_bytes(),
        "M.S".as_bytes(),
    ], true)]
    fn test_check_3_by_3(matrix: Vec<&[u8]>, expected: bool) {
        assert_eq!(check_3_by_3(&matrix), expected);
    }

    #[test_case(vec!["XMAS".as_bytes()], (0,0), true)]
    #[test_case(vec!["XMAS".as_bytes()], (1,0), false)]
    #[test_case(vec!["XMAS".as_bytes()], (2,0), false)]
    #[test_case(vec!["XMAS".as_bytes()], (3,0), false)]
    fn test_check_right(matrix: Vec<&[u8]>, idx: (usize, usize), expected: bool) {
        let actual = check_right(&matrix, idx);
        assert_eq!(actual, expected);
    }
    
    #[test_case(vec!["SAMX".as_bytes()], (0,0), false)]
    #[test_case(vec!["SAMX".as_bytes()], (1,0), false)]
    #[test_case(vec!["SAMX".as_bytes()], (2,0), false)]
    #[test_case(vec!["SAMX".as_bytes()], (3,0), true)]
    fn test_check_left(matrix: Vec<&[u8]>, idx: (usize, usize), expected: bool) {
        let actual = check_left(&matrix, idx);
        assert_eq!(actual, expected);
    }

    #[test_case(vec![
        "SXXX".as_bytes(),
        "AXXX".as_bytes(),
        "MXXX".as_bytes(),
        "XXXX".as_bytes(),
    ], (0,0), false)]
    #[test_case(vec![
        "SXXX".as_bytes(),
        "AXXX".as_bytes(),
        "MXXX".as_bytes(),
        "XXXX".as_bytes(),
    ], (0,1), false)]
    #[test_case(vec![
        "SXXX".as_bytes(),
        "AXXX".as_bytes(),
        "MXXX".as_bytes(),
        "XXXX".as_bytes(),
    ], (0,2), false)]
    #[test_case(vec![
        "SXXX".as_bytes(),
        "AXXX".as_bytes(),
        "MXXX".as_bytes(),
        "XXXX".as_bytes(),
    ], (0,3), true)]
    fn test_check_up(matrix: Vec<&[u8]>, idx: (usize, usize), expected: bool) {
        let actual = check_up(&matrix, idx);
        assert_eq!(actual, expected);
    }

    #[test_case(vec![
        "XXXX".as_bytes(),
        "MXXX".as_bytes(),
        "AXXX".as_bytes(),
        "SXXX".as_bytes(),
    ], (0,0), true)]
    #[test_case(vec![
        "XXXX".as_bytes(),
        "MXXX".as_bytes(),
        "AXXX".as_bytes(),
        "SXXX".as_bytes(),
    ], (0,1), false)]
    #[test_case(vec![
        "XXXX".as_bytes(),
        "MXXX".as_bytes(),
        "AXXX".as_bytes(),
        "SXXX".as_bytes(),
    ], (0,2), false)]
    #[test_case(vec![
        "XXXX".as_bytes(),
        "MXXX".as_bytes(),
        "AXXX".as_bytes(),
        "SXXX".as_bytes(),
    ], (0,3), false)]
    fn test_check_down(matrix: Vec<&[u8]>, idx: (usize, usize), expected: bool) {
        let actual = check_down(&matrix, idx);
        assert_eq!(actual, expected);
    }

    #[test_case(vec![
        "XXXX".as_bytes(),
        "XMXX".as_bytes(),
        "XXAX".as_bytes(),
        "XXXS".as_bytes(),
    ], (0,0), true)]
    #[test_case(vec![
        "XXXX".as_bytes(),
        "XMXX".as_bytes(),
        "XXAX".as_bytes(),
        "XXXS".as_bytes(),
    ], (1,0), false)]
    #[test_case(vec![
        "XXXX".as_bytes(),
        "XMXX".as_bytes(),
        "XXAX".as_bytes(),
        "XXXS".as_bytes(),
    ], (0,1), false)]
    #[test_case(vec![
        "XXXX".as_bytes(),
        "XMXX".as_bytes(),
        "XXAX".as_bytes(),
        "XXXS".as_bytes(),
    ], (1,1), false)]
    fn test_check_diag_down_right(matrix: Vec<&[u8]>, idx: (usize, usize), expected: bool) {
        let actual = check_diag_down_right(&matrix, idx);
        assert_eq!(actual, expected);
    }

    #[test_case(vec![
        "XXXX".as_bytes(),
        "XXMX".as_bytes(),
        "XAXX".as_bytes(),
        "SXXX".as_bytes(),
    ], (3,0), true)]
    #[test_case(vec![
        "XXXX".as_bytes(),
        "XXMX".as_bytes(),
        "XAXX".as_bytes(),
        "SXXX".as_bytes(),
    ], (2,2), false)]
    fn test_check_diag_down_left(matrix: Vec<&[u8]>, idx: (usize, usize), expected: bool) {
        let actual = check_diag_down_left(&matrix, idx);
        assert_eq!(actual, expected);
    }

    #[test_case(vec![
        "XXXS".as_bytes(),
        "XXAX".as_bytes(),
        "XMXX".as_bytes(),
        "XXXX".as_bytes(),
    ], (0,3), true)]
    #[test_case(vec![
        "XXXS".as_bytes(),
        "XXAX".as_bytes(),
        "XMXX".as_bytes(),
        "XXXX".as_bytes(),
    ], (1,2), false)]
    fn test_check_diag_up_right(matrix: Vec<&[u8]>, idx: (usize, usize), expected: bool) {
        let actual = check_diag_up_right(&matrix, idx);
        assert_eq!(actual, expected);
    }

    #[test_case(vec![
        "SXXX".as_bytes(),
        "XAXX".as_bytes(),
        "XXMX".as_bytes(),
        "XXXX".as_bytes(),
    ], (3,3), true)]
    fn test_check_diag_up_left(matrix: Vec<&[u8]>, idx: (usize, usize), expected: bool) {
        let actual = check_diag_up_left(&matrix, idx);
        assert_eq!(actual, expected);
    }
}
pub mod parser {

    pub fn parse_to_matrix(input: &str) -> Vec<&[u8]> {
        input.trim().lines().map(|line| line.as_bytes()).collect()
    }

    #[cfg(test)]
    mod test {

        use super::parse_to_matrix;
        use crate::day4::test::SAMPLE;

        #[test]
        fn test_parse_to_matrix() {
            let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#;

            let actual = parse_to_matrix(input);
            assert_eq!(actual, *SAMPLE);
        }
    }
}
