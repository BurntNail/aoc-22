use crate::FileIntermediary;
use std::collections::HashMap;
use utilities::vec_utils::VecUtils;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Item {
    File { name: String, size: u64 },
    Folder { name: String, contents: Vec<Item> },
}

impl Item {
    pub fn from(mut files: Vec<FileIntermediary>, name: Option<String>) -> Self {
        let mut cd_items = files.take_and_return(|f| f.path.is_empty()).map(|fi| {
            let FileIntermediary {
                name,
                path: _,
                size,
            } = fi;
            Self::File { name, size }
        });

        let mut subdirectories: HashMap<_, Vec<_>> = HashMap::new();

        for mut file in files {
            let base_dir_in_me = file.path.remove(0);
            let sd = subdirectories.entry(base_dir_in_me.clone()).or_default();
            sd.push(file);
        }

        subdirectories
            .into_iter()
            .for_each(|(subd_name, cntnts)| cd_items.push(Self::from(cntnts, Some(subd_name))));

        Self::Folder {
            name: name.unwrap_or_default(),
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

    pub fn get_folders_lte_size(&self, less_than_oe: u64) -> Vec<Self> {
        let mut v = vec![];

        if self.full_contents() <= less_than_oe && self.is_folder() {
            v.push(self.clone());
        }

        if let Self::Folder { name: _, contents } = self {
            if !contents.is_empty() {
                for f in contents.iter().filter(|f| f.is_folder()) {
                    v.append(&mut f.get_folders_lte_size(less_than_oe));
                }
            }
        }

        v
    }

    pub fn get_folders_gte_size(&self, greater_than_oe: u64) -> Vec<Self> {
        let mut v = vec![];

        if self.full_contents() >= greater_than_oe && self.is_folder() {
            v.push(self.clone());
        }

        if let Self::Folder { name: _, contents } = self {
            if !contents.is_empty() {
                for f in contents.iter().filter(|f| f.is_folder()) {
                    v.append(&mut f.get_folders_gte_size(greater_than_oe));
                }
            }
        }

        v
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Folder { name, .. } | Self::File { name, .. } => name,
        }
    }
}
