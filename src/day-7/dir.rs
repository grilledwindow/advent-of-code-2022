use std::collections::HashMap;

type Dirs<'a> = HashMap<&'a str, Dir<'a>>;

#[derive(Debug)]
pub(super) struct Dir<'a> {
    size: u32,
    dirs: Option<Dirs<'a>>,
}

impl<'a> Dir<'a> {
    pub(super) fn new(size: u32, dirs: Option<Dirs<'a>>) -> Self {
        Self { size, dirs }
    }

    pub(super) fn calculate_size(&mut self) -> u32 {
        let size = self._get_size(&self);
        self.size = size;
        size
    }

    pub(super) fn add_dir(&mut self, dir_name: &'a str, dir: Dir<'a>) {
        if let Some(dirs) = &mut self.dirs {
            dirs.insert(dir_name, dir);
        } else {
            let mut dirs = HashMap::new();
            dirs.insert(dir_name, dir);
            self.dirs = Some(dirs);
        }
    }

    pub(super) fn set_dirs(&mut self, dirs: Option<Dirs<'a>>) {
        self.dirs = dirs;
    }

    fn get_size(&self) -> u32 {
        self.size
    }

    fn get_dirs(&self) -> &Option<Dirs> {
        &self.dirs
    }

    fn _get_size(&self, dir: &Dir) -> u32 {
        if let Some(dirs) = dir.get_dirs() {
            dirs
                .into_iter()
                .fold(dir.size, |_size, (_, _dir)| _size + self._get_size(&_dir))
        } else {
            dir.size
        }
    }
}
