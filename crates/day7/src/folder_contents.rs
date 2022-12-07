use itertools::Itertools;

pub enum FolderContents {
    Directory(String),
    File {
        name: String,
        size: usize
    }
}
impl TryFrom<String> for FolderContents { //TODO: Rempve clones, iters to speed up, but only once I've got a criterion build
    type Error = color_eyre::Report;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut chars: Vec<_> = value.chars().collect();

        if [chars[0], chars[1], chars[2]] == ['d', 'i', 'r'] {
            //we are in a directory, remove the first 4 letters "dir "
            chars.remove(0);
            chars.remove(0);
            chars.remove(0);
            chars.remove(0);

            return Ok(FolderContents::Directory(chars.into_iter().map(|c| c.to_string()).join("")));
        } else {
            let mut size = String::default();
            loop {
                let c = chars.remove(0);
                if c == ' ' {
                    break;
                }
                size.push(c);
            }
            let size = size.parse()?;
            return Ok(FolderContents::File { name: chars.into_iter().map(|c| c.to_string()).join(""), size })
        }
    }
}
