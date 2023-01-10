#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

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
    let mut current_dir = Some(vec![]);

    for line in input {
        if let Ok(command) = Command::try_from(line.clone()) {
            match command {
                Command::ChangeDirectory(cd) => match cd {
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

                        current_dir = Some(new_cd);
                    }
                },
                Command::ListFiles => {}
            }
        } else {
            match FolderContents::try_from(line)? {
                FolderContents::Directory(_dir) => {}
                FolderContents::File { name, size } => intermediaries.push(FileIntermediary::new(
                    name,
                    current_dir.clone().unwrap(),
                    size,
                )),
            }
        }
    }

    let root = Item::from(intermediaries, None);
    part1(&root);
    println!("\n");
    part2(&root);

    Ok(())
}

fn part1(root: &Item) {
    let max = 100_000;
    let tot: u64 = root
        .get_folders_lte_size(max)
        .into_iter()
        .map(|x| x.full_contents())
        .sum();

    println!("Sum of <= {max} = {tot:?}");
}

fn part2(root: &Item) {
    let space_needed_to_be_free = {
        let target_size = 70_000_000_u64 - 30_000_000_u64;
        let current_size = root.full_contents();
        current_size - target_size
    };

    let mut workers = root.get_folders_gte_size(space_needed_to_be_free);
    workers.sort_by_key(Item::full_contents);
    let w = workers.remove(0);
    println!("{space_needed_to_be_free} - all needs to be free");
    println!("{} {} - Should be deleted", w.full_contents(), w.name());
}
