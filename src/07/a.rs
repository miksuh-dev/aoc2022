use std::fs;

#[derive(Debug, Clone)]
pub struct FolderOrFile {
    name: String,
    size: u64,
    children: Vec<FolderOrFile>,
    parent: Option<Box<FolderOrFile>>,
}

pub fn main() {
    let root = FolderOrFile {
        name: String::from("root"),
        size: 0,
        children: vec![],
        parent: None,
    };

    let input = fs::read_to_string("src/07/input.txt").expect("File not found");

    // let mut pointer = &root;

    let mut result = input
        .lines()
        .fold(root, |mut pointer, command| -> FolderOrFile {
            if command.starts_with("ls") || command == ("$ cd /") {
                // Do nothing
                pointer = pointer.clone();

                return pointer;
            }

            if command == "$ cd .." {
                pointer = *pointer.parent.unwrap();

                println!("cd .. new pointer {}", pointer.name);

                return pointer;
            }

            if command.starts_with("dir ") {
                let folder_name = command.split(" ").collect::<Vec<&str>>()[1];

                let folder = FolderOrFile {
                    name: folder_name.to_string(),
                    size: 0,
                    children: vec![],
                    parent: Some(Box::new(pointer.to_owned())),
                };

                println!("dir {:?} {}", folder.name, pointer.name);

                pointer.children.push(folder);

                return pointer;
            }

            if command.starts_with("$ cd ") {
                let folder_name = command.split(" ").collect::<Vec<&str>>()[2];

                println!("cd {}", folder_name);

                println!("{:?}", pointer.children);

                let folder = pointer
                    .children
                    .iter()
                    .find(|x| x.name == folder_name)
                    .unwrap()
                    .to_owned();

                println!("cd {:?}", folder.name);

                pointer = folder;

                return pointer;
            }

            // file
            if command.as_bytes()[0].is_ascii_digit() {
                let (size, file_name) = command.split_once(" ").unwrap();

                let file = FolderOrFile {
                    name: file_name.to_string(),
                    size: size.parse::<u64>().unwrap(),
                    children: vec![],
                    parent: Some(Box::new(pointer.to_owned())),
                };

                println!("create file {:?} {}", file.name, pointer.name);

                pointer.children.push(file);

                return pointer;
            }

            pointer
        });

    println!("{:?}", result.children);
}
