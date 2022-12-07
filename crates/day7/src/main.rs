#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

use color_eyre::eyre::bail;
use command::{Command, DirectoryChange};
use folder_contents::FolderContents;
use true_file_repr::Item;

mod command;
mod folder_contents;
mod true_file_repr;

#[derive(Debug, Clone)]
pub struct FileIntermediary {
    pub name: String,
    pub path: Vec<String>,
    pub size: u64,
}

impl FileIntermediary {
    #[must_use]
    pub fn new(name: String, path: Vec<String>, size: u64) -> Self {
        Self { name, path, size }
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let input: Vec<_> = include_str!("input.txt")
        .lines()
        .map(|l| l.trim().to_string())
        .collect();

    let mut intermediaries: Vec<FileIntermediary> = vec![];
    let mut directories = vec![];
    let mut current_dir = Some(vec![]);

    for line in input {
        if let Ok(command) = Command::try_from(line.clone()) {
            match command {
                Command::ChangeDirectory(cd) => {
                    match cd {
                        DirectoryChange::Up => {
                            current_dir = current_dir.map(|mut cd| {
                                cd.pop();
                                cd
                            });
                        }
                        DirectoryChange::Root => {
                            current_dir = current_dir.map(|mut cd| {
                                cd.clear();
                                cd
                            });
                        }
                        DirectoryChange::InCurrent(new) => {
                            let mut new_cd = std::mem::take(&mut current_dir).unwrap();
                            new_cd.push(new);

                            if directories.contains(&new_cd) {
                                //better way to check
                                current_dir = Some(new_cd);
                            } else {
                                //SAFETY: if we bail, we leave current_dir uninit, but we leave the function so it gets dropped as normal.
                                bail!("Directory {new_cd:?} in {new_cd:?} not found in {directories:?}");
                            }
                        }
                    }
                }
                Command::ListFiles => {}
            }
        } else {
            match FolderContents::try_from(line)? {
                FolderContents::Directory(dir) => directories.push({
                    let mut current_dir = current_dir.clone().unwrap();
                    current_dir.push(dir);
                    current_dir
                }),
                FolderContents::File { name, size } => intermediaries.push(FileIntermediary::new(
                    name,
                    current_dir.clone().unwrap(),
                    size,
                )),
            }
        }
    }

    let root = Item::from(&intermediaries, &directories, None);
    part1(root);

    Ok(())
}

fn part1(root: Item) {
    // println!("{root:#?}");

    let max = 100_000;
    // let max = u64::MAX;
    let lt = root.get_folders_with_size(max);

    let mut tot = 0;
    for f in lt {
        println!("{f:#?}\n\n");
        tot += f.full_contents();
    }

    println!("Sum of <= {max} = {tot:?}");
}
