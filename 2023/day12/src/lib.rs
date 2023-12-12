use common::winnow::stream::AsChar;
use std::collections::BTreeMap;
use std::io::Write;

/// Make a .dot file to be render with graphviz, a bit hacky
pub fn create_visual_graph(map: BTreeMap<(usize, usize, usize), usize>, puzzle: String, key: Vec<usize>) {
    let mut dot_file = std::fs::File::create("day12.dot").unwrap();
    let mut file_contents = String::new();

    file_contents += "strict digraph {\n";
    file_contents += r#"rankdir = "LR";"#;
    for ((puzzle_index, key_index, current_count), solution) in &map {
        if *solution == 0 {
            continue;
        }
        let char = puzzle.as_bytes().get(*puzzle_index).unwrap_or(&b'X').as_char();

        file_contents += format!(
            r#"_{}_{}_{} [shape="record", label= "{}|solutions:{}"];{}"#,
            puzzle_index, key_index, current_count, char, solution, '\n'
        )
        .as_str();
        if let '#' | '?' = char {
            if let Some(s2) = map.get(&(puzzle_index + 1, *key_index, current_count + 1)) {
                file_contents += format!(
                    r##"_{}_{}_{} -> _{}_{}_{} [label = "#"];{}"##,
                    puzzle_index,
                    key_index,
                    current_count,
                    puzzle_index + 1,
                    key_index,
                    current_count + 1,
                    '\n'
                )
                .as_str();
                if *s2 == 0 {
                    file_contents += format!(
                        r#"_{}_{}_{} [shape="record", color="red", label= "{}|solutions:{}"];{}"#,
                        puzzle_index + 1,
                        key_index,
                        current_count + 1,
                        char,
                        s2,
                        '\n'
                    )
                    .as_str()
                }
            }
        }
        if let '.' | '?' = char {
            if *current_count != 0 {
                if let Some(a) = key.get(*key_index) {
                    if *current_count == *a {
                        if let Some(s2) = map.get(&(puzzle_index + 1, key_index + 1, 0)) {
                            file_contents += format!(
                                r##"_{}_{}_{} -> _{}_{}_{} [label = "."];{}"##,
                                puzzle_index,
                                key_index,
                                current_count,
                                puzzle_index + 1,
                                key_index + 1,
                                0,
                                '\n'
                            )
                            .as_str();
                            if *s2 == 0 {
                                file_contents += format!(
                                    r#"_{}_{}_{} [shape="record", color="red",label= "{}|solutions:{}"];{}"#,
                                    puzzle_index + 1,
                                    key_index + 1,
                                    0,
                                    char,
                                    s2,
                                    '\n'
                                )
                                .as_str()
                            }
                        }
                    }
                }
            } else if let Some(s2) = map.get(&(puzzle_index + 1, *key_index, *current_count)) {
                file_contents += format!(
                    r##"_{}_{}_{} -> _{}_{}_{} [label = "."];{}"##,
                    puzzle_index,
                    key_index,
                    current_count,
                    puzzle_index + 1,
                    key_index,
                    current_count,
                    '\n'
                )
                .as_str();
                if *s2 == 0 {
                    file_contents += format!(
                        r#"_{}_{}_{} [shape="record", color="red",label= "{}|solutions:{}"];{}"#,
                        puzzle_index + 1,
                        key_index,
                        current_count,
                        char,
                        s2,
                        '\n'
                    )
                    .as_str()
                }
            }
        }
    }
    file_contents += "}\n";
    dot_file.write_all(file_contents.as_bytes()).unwrap();

}
