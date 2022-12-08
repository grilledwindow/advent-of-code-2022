#[derive(Debug)]
pub(super) struct Dir<'a> {
    name: &'a str,
    size: u32,
    dirs: Option<Vec<Dir<'a>>>,
}

impl<'a> Dir<'a> {
    pub(super) fn new(name: &'a str, size: u32, dirs: Option<Vec<Dir<'a>>>) -> Self {
        Self { name, size, dirs }
    }

    pub(super) fn calculate_size(&mut self) -> u32 {
        let size = self._get_size(&self);
        self.size = size;
        size
    }

    pub(super) fn add_dir(&mut self, dir: Dir<'a>) {
        if let Some(dirs) = &mut self.dirs {
            dirs.push(dir);
        } else {
            self.dirs = Some(vec![dir]);
        }
    }

    pub(super) fn set_dirs(&mut self, dirs: Option<Vec<Dir<'a>>>) {
        self.dirs = dirs;
    }

    fn get_size(&self) -> u32 {
        self.size
    }

    fn get_dirs(&self) -> &Option<Vec<Dir<'a>>> {
        &self.dirs
    }

    fn _get_size(&self, dir: &Dir) -> u32 {
        if let Some(dirs) = dir.get_dirs() {
            dirs
                .into_iter()
                .fold(dir.size, |_size, _dir| _size + self._get_size(&_dir))
        } else {
            dir.size
        }
    }
}
