use crate::FileIntermediary;
use itertools::Itertools;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Item {
    File { name: String, size: u64 },
    Folder { name: String, contents: Vec<Item> },
}

impl Item {
    pub fn from(
        files: &Vec<FileIntermediary>,
        folders: &Vec<Vec<String>>,
        start_path: Option<Vec<String>>,
    ) -> Self {
        let mut start_path = start_path.unwrap_or_default();

        let mut cd_items = files
            .clone()
            .into_iter()
            .filter(|f| f.path == start_path)
            .map(|item| {
                let FileIntermediary {
                    name,
                    path: _,
                    size,
                } = item;
                Self::File { name, size }
            })
            .collect_vec();

        let cd_folders = folders.clone().into_iter().filter(|f| {
            start_path.iter().zip(f).all(|(a, b)| a == b) && f.len() == start_path.len() + 1
        });

        for folder in cd_folders {
            let dir_down_folders = folders
                .clone()
                .into_iter()
                .filter(|f| {
                    folder.iter().zip(f).all(|(a, b)| a == b) && f.len() == folder.len() + 1
                })
                .collect_vec();
            let dir_down_files = files
                .clone()
                .into_iter()
                .filter(|f| folder.iter().zip(&f.path).all(|(a, b)| a == b))
                .collect_vec();

            let folder_items = Self::from(&dir_down_files, &dir_down_folders, Some(folder));
            cd_items.push(folder_items);
        }

        Self::Folder {
            name: start_path.pop().unwrap_or_else(|| "/".into()),
            contents: cd_items,
        }
    }

    #[allow(dead_code)]
    pub fn raw_contents(&self) -> u64 {
        self.contents_inner(true, false)
    }
    pub fn full_contents(&self) -> u64 {
        self.contents_inner(true, true)
    }
    pub const fn is_folder(&self) -> bool {
        matches!(self, Self::Folder { .. })
    }

    fn contents_inner(&self, index_folders: bool, is_fully_recursive: bool) -> u64 {
        let mut tot = 0;
        match self {
            Self::File { name: _, size } => tot += size,
            Self::Folder { name: _, contents } => {
                if !contents.is_empty() && index_folders {
                    for c in contents {
                        tot += c.contents_inner(is_fully_recursive, is_fully_recursive);
                    }
                }
            }
        }
        tot
    }

    pub fn get_folders_with_size(&self, less_than_oe: u64) -> Vec<Self> {
        let mut v = vec![];

        if self.full_contents() <= less_than_oe && self.is_folder() {
            v.push(self.clone());
        }

        if let Self::Folder { name: _, contents } = self {
            if !contents.is_empty() {
                for f in contents.iter().filter(|f| f.is_folder()) {
                    v.append(&mut f.get_folders_with_size(less_than_oe));
                }
            }
        }

        v
    }
}
