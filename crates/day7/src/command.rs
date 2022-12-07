use itertools::Itertools;

pub enum Command {
    ChangeDirectory(DirectoryChange), //If None, go back a directory
    ListFiles,
}
pub enum DirectoryChange {
    Up,
    Root,
    InCurrent(String),
}
impl TryFrom<String> for Command {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut chars: Vec<_> = value.chars().collect();

        if chars.remove(0) != '$' {
            return Err(value);
        }
        chars.remove(0); //Remove space

        match [chars.remove(0), chars.remove(0)] {
            ['l', 's'] => return Ok(Self::ListFiles),
            ['c', 'd'] => {}
            _ => return Err(value),
        }
        chars.remove(0); // remove space

        if chars.is_empty() {
            return Err(value);
        }

        if chars[0] == '/' {
            Ok(Self::ChangeDirectory(DirectoryChange::Root))
        } else if chars[0..2] == ['.', '.'] {
            Ok(Self::ChangeDirectory(DirectoryChange::Up))
        } else {
            Ok(Self::ChangeDirectory(DirectoryChange::InCurrent(
                chars.into_iter().map(|c| c.to_string()).join(""),
            )))
        }
    }
}
