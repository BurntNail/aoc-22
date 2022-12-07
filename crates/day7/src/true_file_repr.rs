use crate::FileIntermediary;

pub enum Item {
    File {
        name: String,
        size: usize
    },
    Folder {
        name: String,
        contents: Vec<Item>
    }
}

impl Item {
    pub fn from (files: Vec<FileIntermediary>, folders: Vec<String>) -> Self {
        let mut folders = folders.into_iter().map(|folder| folder.split("/").skip(1).map(|s| s.to_string()).collect::<Vec<_>>()).collect::<Vec<_>>();
        
        let roots = folders.clone().into_iter().filter_map(|mut f| if f.len() == 1 {Some(f.remove(0))} else {None});
        let non_roots = folders.into_iter().filter_map(|mut f| if f.len() == 1 {None} else {f.remove(0); f}).collect::Vec<_>>();

        for root in roots {
            let contents = non_roots.clone();
        }

        Self::Folder { name: "/".into(), contents: vec![] }
    }
}
