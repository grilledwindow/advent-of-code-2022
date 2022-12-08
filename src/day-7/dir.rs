#[derive(Debug)]
pub(super) struct Dir<'a> {
    pub name: &'a str,
    pub size: u32,
    pub children: Option<Vec<Dir<'a>>>,
}

impl<'a> Dir<'a> {
    pub(super) fn new(name: &'a str, size: u32, children: Option<Vec<Dir<'a>>>) -> Self {
        Self {
            name,
            size,
            children,
        }
    }

    pub(super) fn calculate_size(&mut self) -> u32 {
        const size: u32 = self._get_size(&self);
        self.size = size;
    }

    fn get_size(&self) -> u32 {
        self.size
    }

    fn get_children(&self) -> &Option<Vec<Dir<'a>>> {
        &self.children
    }

    fn _get_size(&self, dir: &Dir) -> u32 {
        if let Some(children) = dir.get_children() {
            children
                .into_iter()
                .fold(dir.size, |_size, _dir| _size + self._get_size(&_dir))
        } else {
            dir.size
        }
    }
}
