use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PageNumber(u32);

pub type Update<'a> = Vec<Page<'a>>;
type OrderingRules = HashMap<PageNumber, HashMap<PageNumber, Ordering>>;

#[derive(Debug, Clone, Copy, Eq)]
pub struct Page<'a> {
    page_number: PageNumber,
    ordering: &'a OrderingRules 
}

impl<'a> Page<'a> {
    #[must_use]
    pub fn new(page_number: PageNumber, ordering: &'a OrderingRules) -> Self {
        Page {
            page_number,
            ordering
        }
    }

    pub fn number(&self) -> u32 {
        self.page_number.0
    }
}

impl<'a> PartialEq for Page<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.ordering.get(&self.page_number)
            .and_then(|inner| inner.get(&other.page_number)).is_none()
    }
}

impl<'a> PartialOrd for Page<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.ordering.get(&self.page_number)
            .and_then(|inner| inner.get(&other.page_number).copied())
    }
}

impl<'a> Ord for Page<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ordering.get(&self.page_number)
            .and_then(|inner| inner.get(&other.page_number).copied())
            .unwrap_or(Ordering::Equal)
    }
}

#[cfg(test)]
mod test {
    use std::cmp::Ordering;

    use test_case::test_case;

    use super::{parser, Page, PageNumber};

    #[test_case(vec![75,47,61,53,29], true)]
    #[test_case(vec![97,61,53,29,13], true)]
    #[test_case(vec![75,29,13], true)]
    #[test_case(vec![75,97,47,61,53], false)]
    #[test_case(vec![61,13,29], false)]
    #[test_case(vec![97,13,75,29,47], false)]
    fn test_is_sorted(raw_pages: Vec<u32>, expected: bool) {
        let sample_rules = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13
"#;

        let (_, rules) = parser::ordering_rules(sample_rules).unwrap();
        let pages = raw_pages.into_iter()
            .map(|pg| Page::new(PageNumber(pg), &rules));

        assert_eq!(pages.is_sorted(), expected);
    }

    #[test_case(1, 2, "3|4\n", true)]
    #[test_case(3, 4, "3|4\n", false)]
    fn test_page_partial_eq(a: u32, b: u32, rule: &str, expected: bool) {
        let (_, rules) = parser::ordering_rules(rule).unwrap();
        let page_a = Page::new(PageNumber(a), &rules);
        let page_b = Page::new(PageNumber(b), &rules);

        assert_eq!(page_a == page_b, expected);
    }

    #[test_case(1, 2, "3|4\n", Ordering::Equal)]
    #[test_case(3, 4, "3|4\n", Ordering::Less)]
    #[test_case(4, 3, "3|4\n", Ordering::Greater)]
    fn test_page_ord(a: u32, b: u32, rule: &str, expected: Ordering) {
        let (_, rules) = parser::ordering_rules(rule).unwrap();
        let page_a = Page::new(PageNumber(a), &rules);
        let page_b = Page::new(PageNumber(b), &rules);

        assert_eq!(page_a.cmp(&page_b), expected);
    }
 }

pub mod parser {
    use std::{cmp::Ordering, collections::HashMap};
    use nom::{bytes::complete::tag, character::complete::{newline, u32}, combinator::map, multi::{fold_many1, separated_list1}, sequence::{separated_pair, terminated}, IResult};

    use super::{PageNumber, OrderingRules};

    type UpdatedPageNumbers = Vec<Vec<PageNumber>>;

    pub fn input_sections(input: &str) -> IResult<&str, (OrderingRules, UpdatedPageNumbers)> {
        separated_pair(ordering_rules, newline, updates)(input)
    }

    fn updates(input: &str) -> IResult<&str, UpdatedPageNumbers> {
        separated_list1(newline, update)(input)
    }

    fn update(input: &str) -> IResult<&str, Vec<PageNumber>> {
        separated_list1(tag(","), page_number)(input)
    }

    pub fn ordering_rules(input: &str) -> IResult<&str, OrderingRules> {
        fold_many1(terminated(ordering, newline), || HashMap::new(), |mut map, (a, b)| {
             let inner_map = map.entry(a).or_insert(HashMap::new());
             inner_map.insert(b, Ordering::Less);

             let inner_map = map.entry(b).or_insert(HashMap::new());
             inner_map.insert(a, Ordering::Greater);

            map
        })(input)
    }

    fn ordering(input: &str) -> IResult<&str, (PageNumber, PageNumber)> {
        map(
            separated_pair(page_number, tag("|"), page_number),
            |(a, b)| (a, b)
        )(input)
    }

    fn page_number(input: &str) -> IResult<&str, PageNumber> {
        map(u32, |v| PageNumber(v))(input)
    }

    #[cfg(test)]
    mod test {
        use std::{cmp::Ordering, collections::HashMap};
        use test_case::test_case;
        use crate::day5::{parser::{ordering,update,UpdatedPageNumbers}, OrderingRules, PageNumber,};

        use super::{input_sections, ordering_rules, updates};

        #[test]
        fn test_parse_sample_input() {
            let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#;
            let expected_updates: UpdatedPageNumbers = vec![
                vec![75,47,61,53,29],
                vec![97,61,53,29,13],
                vec![75,29,13],
                vec![75,97,47,61,53],
                vec![61,13,29],
                vec![97,13,75,29,47],
            ].into_iter()
            .map(|each| each.into_iter().map(PageNumber).collect())
            .collect();

            let expected_ordering: OrderingRules = HashMap::from([
                (PageNumber(13), HashMap::from([
                    (PageNumber(29), Ordering::Greater),
                    (PageNumber(47), Ordering::Greater),
                    (PageNumber(53), Ordering::Greater),
                    (PageNumber(61), Ordering::Greater),
                    (PageNumber(75), Ordering::Greater),
                    (PageNumber(97), Ordering::Greater),
                ])),
                (PageNumber(29), HashMap::from([
                    (PageNumber(13), Ordering::Less),
                    (PageNumber(47), Ordering::Greater),
                    (PageNumber(53), Ordering::Greater),
                    (PageNumber(61), Ordering::Greater),
                    (PageNumber(75), Ordering::Greater),
                    (PageNumber(97), Ordering::Greater),
                ])),
                (PageNumber(47), HashMap::from([
                    (PageNumber(13), Ordering::Less),
                    (PageNumber(29), Ordering::Less),
                    (PageNumber(53), Ordering::Less),
                    (PageNumber(61), Ordering::Less),
                    (PageNumber(75), Ordering::Greater),
                    (PageNumber(97), Ordering::Greater),
                ])),
                (PageNumber(53), HashMap::from([
                    (PageNumber(13), Ordering::Less),
                    (PageNumber(29), Ordering::Less),
                    (PageNumber(47), Ordering::Greater),
                    (PageNumber(61), Ordering::Greater),
                    (PageNumber(75), Ordering::Greater),
                    (PageNumber(97), Ordering::Greater),
                ])),
                (PageNumber(61), HashMap::from([
                    (PageNumber(13), Ordering::Less),
                    (PageNumber(29), Ordering::Less),
                    (PageNumber(47), Ordering::Greater),
                    (PageNumber(53), Ordering::Less),
                    (PageNumber(75), Ordering::Greater),
                    (PageNumber(97), Ordering::Greater),
                ])),
                (PageNumber(75), HashMap::from([
                    (PageNumber(13), Ordering::Less),
                    (PageNumber(29), Ordering::Less),
                    (PageNumber(47), Ordering::Less),
                    (PageNumber(53), Ordering::Less),
                    (PageNumber(61), Ordering::Less),
                    (PageNumber(97), Ordering::Greater),
                ])),
                (PageNumber(97), HashMap::from([
                    (PageNumber(13), Ordering::Less),
                    (PageNumber(29), Ordering::Less),
                    (PageNumber(47), Ordering::Less),
                    (PageNumber(53), Ordering::Less),
                    (PageNumber(61), Ordering::Less),
                    (PageNumber(75), Ordering::Less),
                ])),
            ]);

            let (_, (actual_orderings, actual_updates)) = input_sections(input).unwrap();
            assert_eq!(actual_orderings, expected_ordering);
            assert_eq!(actual_updates, expected_updates);

        }

        #[test]
        fn test_parse_ordering_rules() {
            let input = r#"47|53
97|13
97|61
97|47
"#;

            let expected: OrderingRules = HashMap::from([
                (
                    PageNumber(13),
                    HashMap::from([
                        (PageNumber(97), Ordering::Greater),
                    ]),
                ),
                (
                    PageNumber(47),
                    HashMap::from([
                        (PageNumber(53), Ordering::Less),
                        (PageNumber(97), Ordering::Greater),
                    ]),
                ),
                (
                    PageNumber(53),
                    HashMap::from([
                        (PageNumber(47), Ordering::Greater),
                    ]),
                ),
                (
                    PageNumber(61),
                    HashMap::from([
                        (PageNumber(97), Ordering::Greater),
                    ]),
                ),
                (
                    PageNumber(97),
                    HashMap::from([
                        (PageNumber(13), Ordering::Less),
                        (PageNumber(47), Ordering::Less),
                        (PageNumber(61), Ordering::Less),
                    ]),
                ),
            ]);

            let (_, actual) = ordering_rules(input).unwrap();
            assert_eq!(actual, expected);

        }

        #[test_case("47|53", (PageNumber(47), PageNumber(53)))]
        fn test_parse_ordering(input: &str, expected: (PageNumber, PageNumber)) {
            let (_, actual) = ordering(input).unwrap();
            assert_eq!(actual, expected);
        }

        #[test]
        fn test_parse_updates() {
            let input = r#"75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#;
            let expected: Vec<Vec<_>> = vec![
                vec![75,47,61,53,29],
                vec![97,61,53,29,13],
                vec![75,29,13],
                vec![75,97,47,61,53],
                vec![61,13,29],
                vec![97,13,75,29,47],
            ].into_iter()
            .map(|each| each.into_iter().map(PageNumber).collect())
            .collect();

            let (_, actual) = updates(input).unwrap();
            assert_eq!(actual, expected);
        }

        #[test_case(
            "75,47,61,53,29",
            vec![
                PageNumber(75),
                PageNumber(47),
                PageNumber(61),
                PageNumber(53),
                PageNumber(29),
            ]
        )]
        fn test_parse_update(input: &str, expected: Vec<PageNumber>) {
            let (_, actual) = update(input).unwrap();
            assert_eq!(actual, expected);
        }
    }
}