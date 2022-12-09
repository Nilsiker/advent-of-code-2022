use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

fn main() {
    let root_dir = Dir::new("/");
    let root = Rc::new(RefCell::new(root_dir)); // keep reference to root alive
    
    let mut device = Device {
        wd: root.clone(),
    };
    device.touch(".config", 500);

    device.mkdir("a");
    device.cd("a");
    device.touch("file.txt", 100);
    device.cd("..");
    device.wd.borrow().print_tree();
}

struct Device {
    wd: Rc<RefCell<Dir>>,
}
impl Device {
    fn ls(&self) {
        self.wd.borrow().ls();
    }
    fn touch(&self, name: &str, size: usize) {
        self.wd.borrow_mut().add_file(name, size);
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
            let new_wd = self.wd.borrow_mut().dirs.iter().find(|dir| dir.borrow().name == name).unwrap().clone();
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

    fn add_file(&mut self, name: &str, size: usize) {
        self.files.push(File::new(name, size));
    }

    fn add_dir(&mut self, dir: Rc<RefCell<Dir>>) {
        self.dirs.push(dir);
    }

    fn get_parent(&self) -> Option<Rc<RefCell<Dir>>> {
        self.parent.upgrade()
    }

    fn ls(&self) {
        for dir in self.dirs.iter() {
            println!("dir {}", dir.borrow().name);
        }
        for file in &self.files {
            println!("{} {}", file.size, file.name);
        }
    }

    fn get_size(&self) -> usize {
        let file_sizes = self.files.iter().map(|file| file.size).sum::<usize>();
        let mut dir_sizes = 0;
        for dir in self.dirs.iter() {
            dir_sizes += dir.borrow().get_size();
        }

        file_sizes + dir_sizes
    }

    fn print_tree(&self) {
        println!("- {} (dir)", self.name);

        for file in &self.files {
            println!("{} (file, size={})", file.name, file.size);
        }
        for dir in &self.dirs {
            dir.borrow().print_tree();
        }
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

impl Drop for Dir {
    fn drop(&mut self) {
        println!("dropped {}", self.name);
    }
}

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}
impl File {
    fn new(name: &str, size: usize) -> Self {
        Self {
            name: name.to_string(),
            size: size,
        }
    }
}
