use regex::RegexSet;
use relanotes_rs::nodes_representation::NodesRepresentation;
use relanotes_rs::*;
use std::cmp::max;
use std::io;
use std::io::{stdin, stdout, BufRead, BufReader, Write};
use std::iter::repeat;

fn standard_view(
    nodes: &NodesRepresentation,
    child_ids: &Vec<i32>,
    parent_path: &str,
    group_name: &str,
) {
    let header = ("ID", "Name", "Description", "Children");
    let mut max_lengths = (
        header.0.len(),
        header.1.len(),
        header.2.len(),
        header.3.len(),
    );
    let content: Vec<(String, &str, &str, String)> = child_ids
        .iter()
        .map(|id| {
            let node = nodes.get_node(id).unwrap();
            let res = (
                id.to_string(),
                &node.name[..],
                node.description.as_ref().map(|e| &**e).unwrap_or("--"),
                nodes
                    .get_node_loaded_children_count(id)
                    .unwrap()
                    .to_string(),
            );
            max_lengths.0 = max(max_lengths.0, res.0.len());
            max_lengths.1 = max(max_lengths.1, res.1.len());
            max_lengths.2 = max(max_lengths.2, res.2.len());
            max_lengths.3 = max(max_lengths.3, res.3.len());
            res
        })
        .collect();

    let symbols_per_break = 3;
    let breaks_count = 3;

    // Adding spaces
    let overall_length =
        max_lengths.0 + max_lengths.1 + max_lengths.2 + symbols_per_break * breaks_count;
    println!("{}", repeat('-').take(overall_length).collect::<String>());
    println!("Group \"{}\" : {}", group_name, parent_path);

    println!(
        "{:<id_width$} | {:<name_width$} | {:<description_width$} | {:<children_count_width$}",
        header.0,
        header.1,
        header.2,
        header.3,
        id_width = max_lengths.0,
        name_width = max_lengths.1,
        description_width = max_lengths.2,
        children_count_width = max_lengths.3,
    );

    for row in content {
        println!(
            "{:<id_width$} | {:<name_width$} | {:<description_width$} | {:<children_count_width$}",
            row.0,
            row.1,
            row.2,
            row.3,
            id_width = max_lengths.0,
            name_width = max_lengths.1,
            description_width = max_lengths.2,
            children_count_width = max_lengths.3,
        );
    }
    println!("{}", repeat('-').take(overall_length).collect::<String>());
}

enum Command {
    OpenWithId(i32),
    OpenWithName(String),
    AddOrEdit { name: String, description: String }, // Allow access with id
    BackToParent,
    Invalid,
}

fn input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    stdout().lock().flush()?;
    BufReader::new(stdin())
        .lines()
        .next()
        .ok_or(io::Error::new(io::ErrorKind::Other, "Cannot read stdin"))
        .and_then(|inner| inner)
}

fn get_command() -> Command {
    let inp = input("Write your command: ").unwrap();
    let checkers_set = RegexSet::new(&[
        r"^\d+$",
        r"^$",
        r"^(?P<name>^[^-]*(?:-[^-]+)*)\s*--\s*(?P<description>(?:[^-]+-?|$)+)$", // Default requests format from Pharmony
        r"^.+$",
    ])
    .unwrap();

    let first_match = checkers_set.matches(&inp).into_iter().nth(0);

    first_match
        .map(|match_index| match match_index {
            0 => Command::OpenWithId(inp.parse().unwrap()),
            1 => Command::BackToParent,
            2 => {
                let spl = inp.split("--").collect::<Vec<&str>>();
                Command::AddOrEdit {
                    name: String::from(spl.get(0).unwrap().trim()),
                    description: String::from(spl.get(1).unwrap().trim()),
                }
            }
            3 => Command::OpenWithName(inp),
            _ => unimplemented!("Unhandled case!"),
        })
        .unwrap_or(Command::Invalid)
}

fn main() {
    let connection = establish_connection();
    match get_group_by_name(&connection, "My first group") {
        Some(group) => {
            let mut nodes = load_group(&connection, &group);
            let mut parent_id = None;

            loop {
                let children = parent_id
                    .map(|p_id| nodes.get_node_loaded_children(&p_id).unwrap())
                    .unwrap_or(nodes.get_roots());
                let parent_path = parent_id
                    .map(|p_id| nodes.get_node_loaded_path_names(&p_id).unwrap().join(""))
                    .unwrap_or(String::from(""));
                standard_view(&nodes, &children, &parent_path, &group.name);
                let command = get_command();
                match command {
                    Command::BackToParent => {
                        if let Some(p) = parent_id {
                            parent_id = nodes.get_node_loaded_parent(&p);
                        } else {
                            // Nothing happens, because is viewing the roots
                        }
                    }
                    Command::OpenWithId(id) => {
                        if children.iter().find(|e| **e == id).is_some() {
                            parent_id = Some(id);
                        } else {
                            println!("Invalid id");
                        }
                    }
                    Command::OpenWithName(name) => {
                        if let Some(child_id) = children
                            .iter()
                            .find(|e| nodes.get_node(*e).unwrap().name == name)
                        {
                            parent_id = Some(*child_id);
                        } else {
                            println!("Invalid name");
                        }
                    }
                    Command::AddOrEdit { name, description } => {
                        let found = children
                            .iter()
                            .find(|child| nodes.get_node(*child).unwrap().name == name);
                        let resp = match found {
                            Some(child_id) => {
                                // Updating the node
                                nodes
                                    .set_node_description(&connection, child_id, Some(&description))
                                    .and(Ok(()))
                            }
                            None => {
                                // Creating new child
                                nodes
                                    .create_node(
                                        &connection,
                                        &name,
                                        Some(&description),
                                        parent_id,
                                        group.id,
                                        get_node_type(&connection, "regular").unwrap().id,
                                    )
                                    .and(Ok(()))
                            }
                        };
                        match resp {
                            Ok(_) => println!("Done."),
                            Err(e) => {
                                println!("{:?}", e);
                            }
                        }
                    }
                    Command::Invalid => {
                        unimplemented!("Not implemented!");
                    }
                }
            }
        }
        None => {}
    }
}
