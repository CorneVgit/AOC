use id_tree::{
    InsertBehavior::{AsRoot, UnderNode},
    Node, NodeId, Tree,
};
use unwrap_infallible::UnwrapInfallible;

use crate::util::read_all;

fn values() -> Vec<String> {
    read_all::<String>("input_7")
        .into_iter()
        .map(|line| line.unwrap_infallible())
        .collect()
}

struct Entry {
    name: String,
    size: usize,
}

pub fn d7() -> (usize, usize) {
    let terminal_output = values();

    let mut file_system: Tree<Entry> = Tree::new();
    let mut current_node: Option<NodeId> = None;

    for line in terminal_output {
        let mut line_split = line.split_ascii_whitespace();

        match line_split.next() {
            Some(shell) => parse_shell(shell, line_split, &mut file_system, &mut current_node),
            None => panic!("Empty line"),
        }
    }

    calculate_directory_sizes(&mut file_system);

    let space_to_free = calculate_space_to_free(&file_system);

    let mut r1 = 0;
    let mut r2 = usize::MAX;

    for n in file_system
        .traverse_pre_order(file_system.root_node_id().unwrap())
        .unwrap()
    {
        let size = n.data().size;

        if !n.children().is_empty() {
            if size <= 100_000 {
                r1 += size;
            }

            if r2 > size && size > space_to_free {
                r2 = size
            }
        }
    }

    (r1, r2)
}

fn parse_shell(
    shell: &str,
    mut line_split: std::str::SplitAsciiWhitespace,
    file_system: &mut Tree<Entry>,
    current_node: &mut Option<NodeId>,
) {
    match shell {
        "$" => parse_command(&mut line_split, file_system, current_node),
        "dir" => parse_directory(&mut line_split, file_system, current_node),
        _ => parse_file(shell, line_split, file_system, current_node),
    }
}

fn parse_command(
    line_split: &mut std::str::SplitAsciiWhitespace,
    file_system: &mut Tree<Entry>,
    current_node: &mut Option<NodeId>,
) {
    match line_split.next() {
        Some(command) => match command {
            "ls" => (),
            "cd" => parse_command_cd(line_split, file_system, current_node),
            _ => panic!("Invalid command"),
        },
        None => panic!("Empty command"),
    }
}

fn parse_command_cd(
    line_split: &mut std::str::SplitAsciiWhitespace,
    file_system: &mut Tree<Entry>,
    current_node: &mut Option<NodeId>,
) {
    match line_split.next() {
        Some(directory_name) => match directory_name {
            ".." => parse_command_cd_up(file_system, current_node),
            "/" => parse_command_cd_root(file_system, current_node),
            _ => parse_command_cd_dir(file_system, current_node, directory_name),
        },
        None => panic!("Empty cd command"),
    }
}

fn parse_command_cd_up(file_system: &mut Tree<Entry>, current_node: &mut Option<NodeId>) {
    match file_system
        .get(current_node.as_ref().unwrap())
        .unwrap()
        .parent()
    {
        Some(parent_node) => *current_node = Some(parent_node.clone()),
        None => panic!("No parent directory"),
    }
}

fn parse_command_cd_root(file_system: &mut Tree<Entry>, current_node: &mut Option<NodeId>) {
    match file_system.root_node_id() {
        Some(root_node) => *current_node = Some(root_node.clone()),
        None => {
            *current_node = Some(
                file_system
                    .insert(
                        Node::new(Entry {
                            name: "/".to_string(),
                            size: 0,
                        }),
                        AsRoot,
                    )
                    .unwrap(),
            )
        }
    }
}

fn parse_command_cd_dir(
    file_system: &mut Tree<Entry>,
    current_node: &mut Option<NodeId>,
    directory_name: &str,
) {
    for id in file_system
        .children_ids(current_node.as_ref().unwrap())
        .unwrap()
    {
        let entry = file_system.get(id).unwrap();
        if entry.data().name == directory_name {
            *current_node = Some(id.clone());
            return;
        }
    }
    panic!("Dir does not exist");
}

fn parse_directory(
    line_split: &mut std::str::SplitAsciiWhitespace,
    file_system: &mut Tree<Entry>,
    current_node: &mut Option<NodeId>,
) {
    match line_split.next() {
        Some(d) => {
            file_system
                .insert(
                    Node::new(Entry {
                        name: d.to_string(),
                        size: 0,
                    }),
                    UnderNode(current_node.as_ref().unwrap()),
                )
                .unwrap();
        }
        None => panic!("No directory name specified"),
    }
}

fn parse_file(
    shell: &str,
    mut line_split: std::str::SplitAsciiWhitespace,
    file_system: &mut Tree<Entry>,
    current_node: &mut Option<NodeId>,
) {
    match shell.parse::<usize>() {
        Ok(size) => match line_split.next() {
            Some(name) => {
                file_system
                    .insert(
                        Node::new(Entry {
                            name: name.to_string(),
                            size,
                        }),
                        UnderNode(current_node.as_ref().unwrap()),
                    )
                    .unwrap();
            }
            None => panic!("No file name specified"),
        },
        Err(_) => panic!("Invalid line"),
    }
}

fn calculate_directory_sizes(file_system: &mut Tree<Entry>) {
    for node_id in file_system
        .traverse_post_order_ids(file_system.root_node_id().unwrap())
        .unwrap()
    {
        let node = file_system.get(&node_id).unwrap();
        let parent_id = match node.parent() {
            Some(parent_id) => parent_id.to_owned(),
            None => continue,
        };
        let size = node.data().size;
        let parent = file_system.get_mut(&parent_id).unwrap();

        parent.data_mut().size += size;
    }
}

fn calculate_space_to_free(file_system: &Tree<Entry>) -> usize {
    let used_space = file_system
        .get(&file_system.root_node_id().unwrap().clone())
        .unwrap()
        .data()
        .size
        - (70_000_000 - 30_000_000);

    used_space
}
