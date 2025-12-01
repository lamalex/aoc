use nom::{IResult, character::{complete::newline, is_newline}, multi::{many0, separated_list0}, bytes::complete::take_till, combinator::map_parser};

fn main() {
    let input = include_bytes!("input.txt");
    let (_, schematic) = parse_schematic(input).unwrap();
}

fn parse_schematic(i: &[u8]) -> IResult<&[u8], Vec<Vec<GridEntry>>> {
    separated_list0(newline, parse_line)(i)
}

fn parse_line(i: &[u8]) -> IResult<&[u8], Vec<GridEntry>> {
    
    map_parser(take_till(is_newline), many0(parse_member))(i)
}

fn parse_member(i: &[u8]) -> IResult<&[u8], GridEntry> {
    if i.len() == 0 {
        Err(nom::Err::Error(nom::error::Error::new(i, nom::error::ErrorKind::LengthValue)))
    }
    else {
        Ok((&i[1..], match i[0] {
            b'.' => GridEntry::Empty,
            c if (c as char).is_digit(10) => GridEntry::Number(c as char),
            _ => GridEntry::Symbol 
        }))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum GridEntry {
    Empty,
    Symbol,
    Number(char),
    
}

#[cfg(test)]
mod test {
    use super::*;
    // use proptest::prelude::*;
    use test_case::test_case;

    #[test_case(".".as_bytes(), GridEntry::Empty)]
    #[test_case("*".as_bytes(), GridEntry::Symbol)]
    #[test_case("6".as_bytes(), GridEntry::Number('6'))]
    fn test_parse_entry(i: &[u8], expected: GridEntry) {
        let (rem, actual) = parse_member(i).unwrap();

        let expected_rem: &[u8] = &[];
        assert_eq!(expected_rem, rem);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_line() {
        use super::GridEntry::*;

        let input = "467..114..".as_bytes();
        let expected = vec![Number('4'),Number('6'),Number('7'),Empty,Empty,Number('1'),Number('1'),Number('4'),Empty,Empty,];

        let (rem, actual) = parse_line(input).unwrap();
        let expected_rem: &[u8] = &[];
        assert_eq!(expected_rem, rem);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_schematic_input() {
        use super::GridEntry::*;

        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

        let expected = vec![
            vec![Number('4'),Number('6'),Number('7'),Empty,Empty,Number('1'),Number('1'),Number('4'),Empty,Empty,],
            vec![Empty,Empty,Empty,Symbol,Empty,Empty,Empty,Empty,Empty,Empty,],
            vec![Empty,Empty,Number('3'),Number('5'),Empty,Empty,Number('6'),Number('3'),Number('3'),Empty],
            vec![Empty,Empty,Empty,Empty,Empty,Empty,Symbol,Empty,Empty,Empty,],
            vec![Number('6'),Number('1'),Number('7'),Symbol,Empty,Empty,Empty,Empty,Empty,Empty,],
            vec![Empty,Empty,Empty,Empty,Empty,Symbol,Empty,Number('5'), Number('8'), Empty,],
            vec![Empty,Empty,Number('5'),Number('9'),Number('2'),Empty,Empty,Empty,Empty,Empty,],
            vec![Empty,Empty,Empty,Empty,Empty,Empty,Number('7'),Number('5'),Number('5'),Empty],
            vec![Empty,Empty,Empty,Symbol,Empty,Symbol,Empty,Empty,Empty,Empty],
            vec![Empty,Number('6'),Number('6'),Number('4'),Empty,Number('5'),Number('9'),Number('8'),Empty,Empty]
        ];

        let (_, actual) = parse_schematic(input.as_bytes()).unwrap();
        assert_eq!(expected, actual);
    }
}