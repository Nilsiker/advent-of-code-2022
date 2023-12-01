use std::{
    cell::{Ref, RefCell},
    rc::{Rc, Weak},
};

use advent_of_code::read_input_lines;

fn main() {
    let mut device = Device::new();
    let mut lines = read_input_lines(2022, 7);
    lines.remove(0);
    for line in lines.iter() {
        let parts = line.split(' ').collect::<Vec<&str>>();
        if line.starts_with('$') {
            if line.starts_with("$ ls") {
                continue;
            }
            device.cd(parts[2]);
        } else if line.starts_with("dir") {
            device.mkdir(parts[1]);
        } else {
            device.touch(parts[0].parse::<usize>().unwrap())
        }
    }

    let mut v = vec![];
    recursively_collect_dir_sizes_into(device.root.borrow(), &mut v);
    let filtered: Vec<(String, usize)> = v.iter().filter(|d| d.1 <= 100000).cloned().collect();
    println!(
        "Sum of all sizes no greater than 100000 is {:#?}",
        &filtered.iter().map(|d| d.1).sum::<usize>()
    );

    let total = 70000000;
    let needed = 30000000;
    let used = device.root.borrow().get_size();
    let remaining = total - used;
    let min_to_delete = needed - remaining;

    let mut filtered_delete: Vec<(String, usize)> =
        v.iter().filter(|d| d.1 >= min_to_delete).cloned().collect();
    filtered_delete.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    println!(
        "Minimum directory to delete has size {}",
        filtered_delete[0].1
    );
}

fn recursively_collect_dir_sizes_into(dir: Ref<Dir>, v: &mut Vec<(String, usize)>) {
    for dir in &dir.dirs {
        v.push((dir.borrow().name.clone(), dir.borrow().get_size()));
        recursively_collect_dir_sizes_into(dir.borrow(), v);
    }
}

struct Device {
    root: Rc<RefCell<Dir>>,
    wd: Rc<RefCell<Dir>>,
}
impl Device {
    fn new() -> Self {
        let root_dir = Dir::new("/");
        let root = Rc::new(RefCell::new(root_dir)); // keep reference to root alive

        Self {
            root: root.clone(),
            wd: root,
        }
    }

    fn touch(&self, size: usize) {
        self.wd.borrow_mut().add_file(size);
    }

    fn mkdir(&mut self, name: &str) {
        let dir = Dir::new(name);
        self.wd.add_dir(dir);
    }

    fn cd(&mut self, name: &str) {
        if name == ".." {
            let new_wd = self.wd.borrow_mut().parent.upgrade().unwrap();
            self.wd = new_wd;
        } else {
            let new_wd = self
                .wd
                .borrow_mut()
                .dirs
                .iter()
                .find(|dir| dir.borrow().name == name)
                .unwrap()
                .clone();
            self.wd = new_wd;
        }
    }
}

#[derive(Debug)]
struct Dir {
    name: String,
    parent: Weak<RefCell<Dir>>,
    dirs: Vec<Rc<RefCell<Dir>>>,
    files: Vec<File>,
}

impl Dir {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            parent: Weak::new(),
            dirs: vec![],
            files: vec![],
        }
    }

    fn add_child_to_parent(parent: &Rc<RefCell<Dir>>, child: &Rc<RefCell<Dir>>) {
        child.borrow_mut().parent = Rc::downgrade(&parent.clone());
        parent.borrow_mut().add_dir(child.clone());
    }

    fn add_file(&mut self, size: usize) {
        self.files.push(File(size));
    }

    fn add_dir(&mut self, dir: Rc<RefCell<Dir>>) {
        self.dirs.push(dir);
    }

    fn get_size(&self) -> usize {
        let file_sizes = self.files.iter().map(|file| file.0).sum::<usize>();
        let mut dir_sizes = 0;
        for dir in self.dirs.iter() {
            dir_sizes += dir.borrow().get_size();
        }

        file_sizes + dir_sizes
    }
}

trait Convenience {
    fn add_dir(&self, child: Dir);
}
impl Convenience for Rc<RefCell<Dir>> {
    fn add_dir(&self, child: Dir) {
        Dir::add_child_to_parent(self, &Rc::new(RefCell::new(child)));
    }
}

#[derive(Debug)]
struct File(usize);
