use std::fs;

#[derive(Debug, Clone)]
pub struct FolderOrFile {
    name: String,
    size: u64,
    path: String,
    files: Vec<FolderOrFile>,
}

fn get_folder_sizes(folders: &Vec<FolderOrFile>, path: &str) -> Vec<u64> {
    folders
        .iter()
        .filter(|f| f.path != path && f.path.starts_with(path))
        .map(|f| {
            let mut size = 0;

            let subfolders = get_folder_sizes(&folders, &f.path);
            size += subfolders.iter().map(|s| s).sum::<u64>();
            size += f.files.iter().map(|f| f.size).sum::<u64>();
            size += f.size;

            size
        })
        .collect::<Vec<u64>>()
}

pub fn main() {
    let root = FolderOrFile {
        name: String::from("root"),
        size: 0,
        path: "/".to_string(),
        files: vec![],
    };

    let input = fs::read_to_string("src/07/input.txt").expect("File not found");

    let mut cwd = root.path.clone();

    let result = input
        .lines()
        .fold(vec![root], |mut folders, command| -> Vec<FolderOrFile> {
            if command.starts_with("ls") || command == ("$ cd /") {
                // folders

                return folders;
            }

            if command == "$ cd .." {
                let path = cwd.split("/").collect::<Vec<&str>>();

                cwd = path
                    .iter()
                    .enumerate()
                    .take_while(|(index, _)| index != &(&path.len() - 1))
                    .map(|(_, item)| *item)
                    .collect::<Vec<&str>>()
                    .join("/");

                if cwd == "" {
                    cwd = "/".to_string();
                }

                return folders;
            }

            if command.starts_with("dir ") {
                let folder_name = command.split(" ").collect::<Vec<&str>>()[1];

                let new_path = if cwd == "/" {
                    format!("/{}", folder_name.to_owned())
                } else {
                    format!("{}/{}", cwd.to_owned(), folder_name.to_owned())
                };

                let folder = FolderOrFile {
                    name: folder_name.to_string(),
                    size: 0,
                    path: new_path,
                    files: vec![],
                };

                folders.push(folder);

                return folders;
            }

            if command.starts_with("$ cd ") {
                let folder_name = command.split(" ").collect::<Vec<&str>>()[2];

                let new_path = if cwd == "/" {
                    format!("/{}", folder_name.to_owned())
                } else {
                    format!("{}/{}", cwd.to_owned(), folder_name.to_owned())
                };

                cwd = new_path;

                return folders;
            }

            // file
            if command.as_bytes()[0].is_ascii_digit() {
                let (size, file_name) = command.split_once(" ").unwrap();

                let file = FolderOrFile {
                    name: file_name.to_string(),
                    size: size.parse::<u64>().unwrap(),
                    path: cwd.to_string(),
                    files: vec![],
                };

                folders.iter_mut().for_each(|folder| {
                    if folder.path == cwd {
                        folder.files.push(file.clone());
                    }
                });

                return folders;
            }

            return folders;
        });

    let result = get_folder_sizes(&result, "/")
        .iter()
        .filter(|f| **f < 100000)
        .sum::<u64>();

    println!("Result a: {}", result);
}
