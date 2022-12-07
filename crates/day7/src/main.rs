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
    pub path: String,
    pub size: usize, 
}

impl FileIntermediary {
    pub fn new(name: String, path: String, size: usize) -> Self { Self { name, path, size } }
}



fn main() -> color_eyre::Result<()> { //TODO: Bounds checks to remove panics
    color_eyre::install()?;

    let input: Vec<_> = include_str!("input.txt")
        .lines()
        .map(|l| l.trim().to_string())
        .collect();

    let mut intermediaries: Vec<FileIntermediary> = vec![];
    let mut directories = vec![];
    let mut current_dir = String::new();

    for line in input {
        if let Ok(command) = Command::try_from(line.clone()) {
            match command {
                Command::ChangeDirectory(cd) => match cd {
                    DirectoryChange::Up => {
                        let split = current_dir.split("/").collect::<Vec<_>>();
                        if split.is_empty() {
                            current_dir = "/".into();
                        } else {
                            current_dir = String::from("/") + &split[1..split.len() - 2].join("/") + "/"; //Due to starting and ending slashes, we don't want the outer-most elements
                        }
                    }
                    DirectoryChange::Root => {
                        current_dir = "/".into();
                    }
                    DirectoryChange::InCurrent(new) => {
                        if directories.contains(&(current_dir.clone() + &new)) {
                            current_dir += &(new + "/");
                        } else {
                            bail!("Directory {new} in {current_dir} not found in {directories:?}");
                        }
                    }
                },
                Command::ListFiles => {}
            }
        } else {
            match FolderContents::try_from(line)? {
                FolderContents::Directory(dir) => directories.push(current_dir.clone() + &dir),
                FolderContents::File { name, size } => intermediaries.push(FileIntermediary::new(name, current_dir.clone(), size)),
            }
        }
    }

    let root = Item::from(intermediaries, directories);

    Ok(())
}
