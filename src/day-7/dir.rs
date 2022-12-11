use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

type Dirrr<'a> = Rc<RefCell<Dir<'a>>>;
type Dirs<'a> = HashMap<&'a str, Dirrr<'a>>;
pub type Cwd<'a> = Vec<&'a str>;

#[derive(Debug, Default)]
pub(super) struct Dir<'a> {
    pub(super) root: Option<Dirrr<'a>>,
    pub(super) size: u32,
    pub(super) dirs: Dirs<'a>,
}

impl<'a> Dir<'a> {
    pub(super) fn new(root: Option<Dirrr<'a>>, size: u32, dirs: Dirs<'a>) -> Self {
        Self { root, size, dirs }
    }

    pub(super) fn calculate_size(&'a mut self) {
        // self._get_size(&self).clone();
        // self.size = size;
        // size
    }

    pub(super) fn add_dir(&mut self, dir_name: &'a str, dir: Dir<'a>) {
        // if let Some(dirs) = &mut self.dirs {
            self.dirs.insert(dir_name, Rc::new(RefCell::new(dir)));
            // println!("{0:#?}\n", self.dirs);
        // } else {
        //     let mut dirs = HashMap::new();
        //     dirs.insert(dir_name, Rc::new(RefCell::new(dir)));
        //     self.dirs = Some(dirs);
        // }
    }

    // pub(super) fn set_dirs(&mut self, dirs: Option<Dirs<'a>>) {
    //     self.dirs = dirs;
    // }

    pub(super) fn find_dir(&mut self, target_dir: &'a str) -> Option<Rc<RefCell<Dir<'a>>>> {
        // if let Some(dirs) = &mut self.dirs.borrow() {
            self.dirs.get(target_dir).cloned()
        // } else {
        //     None
        // }
    }

    // pub fn get_dirs_mut(&'a mut self) -> &'a mut Option<Dirs> {
    //     &mut self.dirs
    // }

    // pub fn find_dir(&mut self, target_dir: &'a str) -> Option<&mut Dir> {
    //     if let Some(dirs) = &mut self.dirs {
    //         dirs.get_mut(target_dir)
    //     } else {
    //         None
    //     }
    // }

    // fn _get_size(&'a self, dir: &'a Dir<'a>) -> u32 {
    //     // let dir = dirrr.borrow();
    //     if let Some(dirs) = dir.get_dirs() {
    //         dirs
    //             .into_iter()
    //             .fold(dir.get_size(), |_size, (_, _dirrr)| _size + self.borrow().get_size())
    //     } else {
    //         dir.get_size()
    //     }
    // }
}
