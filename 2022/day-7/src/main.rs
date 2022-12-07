#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileTree<'a> {
    Dir(&'a str, Vec<FileTree<'a>>),
    File(&'a str, usize),
}

impl<'a> PartialOrd for FileTree<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for FileTree<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        std::cmp::Ordering::Equal
    }
}

impl<'a> FileTree<'a> {
    pub fn is_dir(&self) -> bool {
        match self {
            FileTree::Dir(_, _) => true,
            _ => false,
        }
    }

    pub fn all_dirs(&self) -> Vec<FileTree> {
        let dirs: Vec<FileTree> = match self {
            Self::Dir(_, children) => {
                let mut dirs = vec![];
                for child in children.iter().filter(|t| t.is_dir()) {
                    dirs.push(child.clone());
                    dirs.extend(child.all_dirs().iter().cloned());
                }

                dirs
            }
            Self::File(_, _) => {
                println!("oh the fuckin base case?");
                vec![]
            }
        };

        dirs
    }

    pub fn sum_dirs(&self) -> u64 {
        self.all_dirs()
            .iter()
            .map(|t| t.sum())
            .filter(|s| s <= &100000)
            .sum::<u64>()
    }

    pub fn sum(&self) -> u64 {
        match self {
            Self::Dir(_name, children) => children.iter().fold(0, |acc, t| acc + t.sum()),
            Self::File(_, size) => *size as u64,
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    let (_, tree) = parser::parse_str_to_file_tree(input).unwrap();

    const FS_SPACE: u64 = 70000000;
    let total_used = tree.sum();

    let unused = FS_SPACE - total_used;
    let need_to_free = 30000000 - unused;

    let del = tree
        .all_dirs()
        .into_iter()
        .map(|t| t.sum())
        .filter(|size| *size >= need_to_free)
        .min();

    // .min();
    println!("{total_used}");
    println!("{unused}");
    println!("{del:?}",);
}

mod parser {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{digit1, line_ending, multispace1, not_line_ending, space0, space1},
        combinator::map,
        multi::separated_list0,
        sequence::{preceded, terminated, tuple},
        IResult,
    };

    use crate::FileTree;

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum ShellCommand<'a> {
        Ls(Vec<FileTree<'a>>),
        Cd(&'a str),
    }

    pub fn parse_str_to_file_tree(i: &str) -> IResult<&str, FileTree> {
        let (i, interactions) = parse_to_interaction_list(i)?;
        let mut root = FileTree::Dir("/", vec![]);

        assign_children(&mut root, &interactions[1..], &mut 0);

        Ok((i, root))
    }

    fn assign_children<'a>(
        tree: &mut FileTree<'a>,
        interactions: &[ShellCommand<'a>],
        mut idx: &mut usize,
    ) {
        while *idx < interactions.len() {
            let interaction = &interactions[*idx];
            *idx += 1;

            match interaction {
                ShellCommand::Ls(o) => {
                    if let FileTree::Dir(_name, c) = tree {
                        *c = o.clone();
                    }
                }
                ShellCommand::Cd(name) => {
                    match name {
                        &".." => return,
                        name => {
                            // loop through this tree's children and find the one whose name matches
                            // then recurse
                            if let FileTree::Dir(_, children) = tree {
                                for child in children {
                                    if let FileTree::Dir(c_name, _) = child {
                                        if c_name == name {
                                            assign_children(child, &interactions[..], &mut idx)
                                        }
                                    }
                                }
                            } else {
                                panic!("Called `cd` on a file");
                            }
                        }
                    }
                }
            }
        }
    }

    fn parse_to_interaction_list(i: &str) -> IResult<&str, Vec<ShellCommand>> {
        separated_list0(line_ending, parse_str_to_cmd)(i)
    }

    fn parse_str_to_cmd(i: &str) -> IResult<&str, ShellCommand> {
        let (i, o) = preceded(
            preceded(tag("$"), multispace1),
            alt((
                map(
                    preceded(terminated(tag("ls"), multispace1), parse_ls_output),
                    |o| ShellCommand::Ls(o),
                ),
                map(preceded(tag("cd"), not_line_ending), |s: &str| {
                    ShellCommand::Cd(s.trim())
                }),
            )),
        )(i)?;

        Ok((i, o))
    }

    fn parse_ls_output(i: &str) -> IResult<&str, Vec<FileTree>> {
        separated_list0(line_ending, parse_ls_single_line)(i)
    }

    fn parse_ls_single_line(i: &str) -> IResult<&str, FileTree> {
        preceded(
            space0,
            alt((
                map(
                    tuple((digit1, preceded(space1, not_line_ending))),
                    |(size, name)| FileTree::File(name, size.parse().unwrap()),
                ),
                map(
                    preceded(terminated(tag("dir"), space1), not_line_ending),
                    |name| FileTree::Dir(name, vec![]),
                ),
            )),
        )(i)
    }

    #[cfg(test)]
    mod test {
        use crate::parser::parse_ls_single_line;

        use super::{
            parse_ls_output, parse_str_to_cmd, parse_str_to_file_tree, parse_to_interaction_list,
            FileTree, ShellCommand,
        };
        use test_case::test_case;

        #[test_case("$  ls
", ShellCommand::Ls(vec![]))]
        #[test_case("$ cd   a", ShellCommand::Cd("a"))]
        #[test_case("$ cd     ..", ShellCommand::Cd(".."))]
        fn test_parse_str_to_cmd(i: &str, expected: ShellCommand) {
            let (_, actual) = parse_str_to_cmd(i).unwrap();

            assert_eq!(expected, actual);
        }

        #[test_case("       dir a", FileTree::Dir("a", vec![]))]
        #[test_case("           14848514   b.txt", FileTree::File("b.txt", 14848514))]
        #[test_case("   29116 f", FileTree::File("f", 29116))]
        fn test_parse_ls_single_line(i: &str, expected: FileTree) {
            let (_, actual) = parse_ls_single_line(i).unwrap();

            assert_eq!(expected, actual);
        }
        #[test_case("dir a
            14848514 b.txt
            8504156 c.dat",
            vec![
                FileTree::Dir("a", vec![]),
                FileTree::File("b.txt", 14848514),
                FileTree::File("c.dat", 8504156)
            ])]
        fn test_parse_ls_output(i: &str, expected: Vec<FileTree>) {
            let (_, actual) = parse_ls_output(i).unwrap();

            assert_eq!(expected, actual);
        }

        #[test]
        fn test_parse_sample_input_to_interactions() {
            let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d";

            let (_, actual) = parse_to_interaction_list(input).unwrap();

            let expected = vec![
                ShellCommand::Cd("/"),
                ShellCommand::Ls(vec![
                    FileTree::Dir("a", vec![]),
                    FileTree::File("b.txt", 14848514),
                    FileTree::File("c.dat", 8504156),
                    FileTree::Dir("d", vec![]),
                ]),
            ];
            assert_eq!(expected, actual);
        }

        #[test]
        fn test_parse_sample_input() {
            let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
            let expected = FileTree::Dir(
                "/",
                vec![
                    FileTree::Dir(
                        "a",
                        vec![
                            FileTree::Dir("e", vec![FileTree::File("i", 584)]),
                            FileTree::File("f", 29116),
                            FileTree::File("g", 2557),
                            FileTree::File("h.lst", 62596),
                        ],
                    ),
                    FileTree::File("b.txt", 14848514),
                    FileTree::File("c.dat", 8504156),
                    FileTree::Dir(
                        "d",
                        vec![
                            FileTree::File("j", 4060174),
                            FileTree::File("d.log", 8033020),
                            FileTree::File("d.ext", 5626152),
                            FileTree::File("k", 7214296),
                        ],
                    ),
                ],
            );
            let (_, actual) = parse_str_to_file_tree(input).unwrap();

            assert_eq!(expected, actual);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::FileTree;

    #[test]
    fn compute_dir_sum() {
        let expected = FileTree::Dir(
            "/",
            vec![
                FileTree::Dir(
                    "a",
                    vec![
                        FileTree::Dir("e", vec![FileTree::File("i", 584)]),
                        FileTree::File("f", 29116),
                        FileTree::File("g", 2557),
                        FileTree::File("h.lst", 62596),
                    ],
                ),
                FileTree::File("b.txt", 14848514),
                FileTree::File("c.dat", 8504156),
                FileTree::Dir(
                    "d",
                    vec![
                        FileTree::File("j", 4060174),
                        FileTree::File("d.log", 8033020),
                        FileTree::File("d.ext", 5626152),
                        FileTree::File("k", 7214296),
                    ],
                ),
            ],
        );

        let sum = expected.sum_dirs();

        assert_eq!(95437, sum);
    }
}
