use std::fs;
use std::process::exit;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use regex::Regex;
use lazy_static::lazy_static;

static INPUT: &str = "input/day7.txt";

#[derive(Debug)]
enum FsEntry {
    File(u64),
    Dir(RefCell<Vec<Rc<FsNode>>>),
}

#[derive(Debug)]
struct FsNode {
    name: String,
    value: FsEntry,
    parent: RefCell<Weak<FsNode>>,
}

impl FsNode {
    fn new_fs() -> Rc<Self> {
        Rc::new(FsNode {
            name: "/".to_string(),
            value: FsEntry::Dir(RefCell::new(Vec::new())),
            parent: RefCell::new(Weak::new()),
        })
    }

    fn mkdir(this: Rc<Self>, name: &str) -> Rc<Self> {
        match &this.value {
            FsEntry::File(..) => panic!("Calling mkdir on a file"),
            FsEntry::Dir(children) => {
                let new_dir = Rc::new(FsNode {
                    name: name.to_string(),
                    value: FsEntry::Dir(RefCell::new(Vec::new())),
                    parent: RefCell::new(Rc::downgrade(&this)),
                });

                children.borrow_mut().push(Rc::clone(&new_dir));
                return Rc::clone(&new_dir);
            }
        }
    }

    fn add_file(this: Rc<Self>, name: &str, size: u64) {
        match &this.value {
            FsEntry::File(..) => panic!("Calling add_file on a file"),
            FsEntry::Dir(children) => {
                let new_file = Rc::new(FsNode {
                    name: name.to_string(),
                    value: FsEntry::File(size),
                    parent: RefCell::new(Rc::downgrade(&this)),
                });
                children.borrow_mut().push(new_file);
            }
        }
    }

    // Return the entry of the directory with name
    fn get(&self, name: &str) -> Option<Rc<Self>> {
        match &self.value {
            FsEntry::File(..) => panic!("Calling get on a file"),
            FsEntry::Dir(children) => {
                children.borrow().iter().find(
                    |e| e.name == name).cloned()
            }
        }
    }

    fn size_rec(&self, mut sizes: &mut Vec<u64>) -> u64 {
        match &self.value {
            FsEntry::File(size) => *size,
            FsEntry::Dir(children) => {
                let size = children.borrow().iter()
                    .map(|e| e.size_rec(&mut sizes))
                    .sum();
                sizes.push(size);
                size
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string(INPUT)
        .unwrap_or_else(|e| {
            eprintln!("Could not read input file: {}", e);
            exit(1);
        });
    let mut lines_it = input.lines();
    if lines_it.next().unwrap_or_default() != "$ cd /" {
        eprintln!("Input doesn't start with 'cd /'");
        exit(2);
    }
    let lines: Vec<&str> = lines_it.collect();
    let my_fs = FsNode::new_fs();
    parse_input(lines, Rc::clone(&my_fs));

    let mut sizes: Vec<u64> = Vec::new();
    let total = my_fs.size_rec(&mut sizes);
    sizes.sort();
    let needed = 30_000_000 - (70_000_000 - total);
    for s in sizes {
        if s >= needed {
            println!("{}", s);
            break;
        }
    }
}

fn parse_input(lines: Vec<&str>, my_fs: Rc<FsNode>) {
    lazy_static! {
        static ref PAT_CD: Regex = Regex::new(r"^\$ cd (.*)$").unwrap();
        static ref PAT_FILE: Regex = Regex::new(r"^(\d+) (.*)$").unwrap();
        static ref PAT_DIR: Regex = Regex::new(r"^dir (.*)$").unwrap();
    }
    let mut cur_dir = my_fs;
    let mut ls_out = false;
    for line in lines {
        if ls_out {
            if line.starts_with('$') {
                ls_out = false;
            } else if let Some(cap) = PAT_DIR.captures(line) {
                let name = cap.get(1).unwrap().as_str();
                FsNode::mkdir(Rc::clone(&cur_dir), name);
            } else if let Some(cap) = PAT_FILE.captures(line) {
                let size: u64 = cap.get(1).unwrap().as_str()
                    .parse().unwrap();
                let name = cap.get(2).unwrap().as_str();
                FsNode::add_file(Rc::clone(&cur_dir), name, size);
            }
        }
        if !ls_out {
            if line == "$ ls" {
                ls_out = true;
            } else if let Some(cap) = PAT_CD.captures(line) {
                let name = cap.get(1).unwrap().as_str();
                cur_dir = if name == ".." {
                    cur_dir.parent.borrow().upgrade().unwrap()
                } else {
                    cur_dir.get(name).unwrap()
                };
            }
        }
    }
}
