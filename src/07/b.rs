use std::fs;

#[derive(Debug, Clone)]
pub struct FolderOrFile {
    name: String,
    size: i64,
    path: String,
    files: Vec<FolderOrFile>,
}

fn get_used_space(folders: &Vec<FolderOrFile>, path: &str) -> i64 {
    folders
        .iter()
        .filter(|f| f.path.starts_with(path))
        .map(|f| {
            let mut size = 0;

            size += f.files.iter().map(|f| f.size).sum::<i64>();
            size += f.size;

            println!("{} {}", f.path, size);

            size
        })
        .sum()
}

fn get_folder_sizes(folders: &Vec<FolderOrFile>, path: &str) -> Vec<i64> {
    folders
        .iter()
        .filter(|f| f.path != path && f.path.starts_with(path))
        .map(|f| {
            let mut size = 0;

            // let subfolders = get_folder_sizes(&folders, &f.path);
            // size += subfolders.iter().map(|s| s).sum::<i64>();

            size += f.files.iter().map(|f| f.size).sum::<i64>();
            size += f.size;

            println!("{} {}", f.path, size);

            size
        })
        .collect::<Vec<i64>>()
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

    let folders = input
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

                println!("cd .. {}", cwd);

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

                println!("cd {}", folder_name);

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
                    size: size.parse::<i64>().unwrap(),
                    path: cwd.to_string(),
                    files: vec![],
                };

                println!("create file {:?}", file.name);

                folders.iter_mut().for_each(|folder| {
                    if folder.path == cwd {
                        folder.files.push(file.clone());
                    }
                });

                return folders;
            }

            return folders;
        });

    let mut folder_sizes = get_folder_sizes(&folders, "/");
    folder_sizes.sort();

    let max: i64 = 70000000;

    let used_space = get_used_space(&folders, "/");
    let remaining = max.saturating_sub(used_space);

    println!("used space: {}", used_space);
    println!("remaining space: {}", remaining);

    println!("folder sizes: {:?}", folder_sizes);
    let result = folder_sizes
        .iter()
        .find(|f| remaining + *f > 30000000)
        .unwrap();

    println!("result: {:?}", result);
}
