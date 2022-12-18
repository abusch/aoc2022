use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use eyre::Result;

pub fn run() -> Result<()> {
    let data = std::fs::read_to_string("inputs/day07.txt")?;

    let mut shell = Shell::new();
    for line in data.lines() {
        shell.parse_line(line);
    }

    let sizes = shell.compute_dir_sizes();
    let total_size = sizes
        .clone()
        .into_iter()
        .map(|(_, s)| s)
        .filter(|size| *size <= 100000)
        .sum::<usize>();

    println!("Part 1: {total_size}");

    const TOTAL_DISK_SPACE: usize = 70_000_000;
    const FREE_SPACE_NEEDED: usize = 30_000_000;
    let total_used_space = sizes
        .iter()
        .find_map(|(name, size)| if name == "/" { Some(size) } else { None })
        .unwrap();
    let total_unused_space = TOTAL_DISK_SPACE - total_used_space;
    let space_to_free = FREE_SPACE_NEEDED - total_unused_space;
    let size_of_dir_to_delete = sizes
        .iter()
        .filter_map(|(_name, size)| (*size >= space_to_free).then_some(*size))
        .min()
        .unwrap();

    println!("Part 2: {size_of_dir_to_delete}");

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Inode(usize);

impl Inode {
    pub fn inc(&mut self) {
        self.0 += 1;
    }
}

impl Deref for Inode {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Inode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Default)]
pub struct InodeTable {
    next_inode: Inode,
    table: HashMap<Inode, Entry>,
}

impl InodeTable {
    pub fn new() -> Self {
        let root = Entry::Dir {
            name: "/".to_string(),
            parent: Inode(0),
            entries: Vec::new(),
        };
        let mut table = Self::default();
        table.add_entry(root);
        table
    }

    pub fn get(&self, inode: Inode) -> Option<&Entry> {
        self.table.get(&inode)
    }

    pub fn get_mut(&mut self, inode: Inode) -> Option<&mut Entry> {
        self.table.get_mut(&inode)
    }

    pub fn add_entry(&mut self, entry: Entry) -> Inode {
        let inode = self.next_inode;
        self.table.insert(inode, entry);
        self.next_inode.inc();
        inode
    }

    /// Find a directory with the given name under the given parent name and return its inode
    pub fn find_dir(&self, parent: Inode, dir_name: &str) -> Inode {
        if let Some(Entry::Dir { entries, .. }) = self.table.get(&parent) {
            entries
                .iter()
                .find(|i| {
                    matches!(self.table.get(i), Some(Entry::Dir { name, .. }) if name.as_str() == dir_name)})
                .copied()
                .unwrap_or_else(|| {
                    panic!("No directory with name {dir_name} found under parent {parent:?}")
                })
        } else {
            panic!("Inode {parent:?} doesn't exist or is not a directory");
        }
    }

    pub fn compute_dir_sizes(&self) -> Vec<(String, usize)> {
        let mut dirs = Vec::new();

        let size = self.compute_dir_sizes_inner(Inode(0), &mut dirs);
        dirs.push(("/".to_string(), size));
        dirs
    }

    fn compute_dir_sizes_inner(&self, root: Inode, acc: &mut Vec<(String, usize)>) -> usize {
        let mut total_size = 0usize;
        // root has to be a dir
        if let Some(Entry::Dir { entries, .. }) = self.get(root) {
            for e in entries {
                match self.get(*e).unwrap() {
                    Entry::Dir { name, .. } => {
                        let dir_size = self.compute_dir_sizes_inner(*e, acc);
                        acc.push((name.clone(), dir_size));
                        total_size += dir_size;
                    }
                    Entry::File { size, .. } => total_size += size,
                }
            }
        }
        total_size
    }
}

#[derive(Debug)]
struct Shell {
    fs: InodeTable,
    cwd: Inode,
}

impl Shell {
    pub fn new() -> Self {
        let fs = InodeTable::new();
        let cwd = Inode(0);
        Self { fs, cwd }
    }

    pub fn parse_line(&mut self, line: &str) {
        if let Some(new_dir) = line.strip_prefix("$ cd ") {
            self.cd(new_dir);
        } else if line.starts_with("$ ls") {
            // nop
        } else if let Some(dir_name) = line.strip_prefix("dir ") {
            self.add_new_dir(dir_name);
        } else {
            // must be a file entry
            let (size_str, name) = line.split_once(' ').unwrap();
            let size = size_str
                .parse::<usize>()
                .unwrap_or_else(|_| panic!("Invalid number: {size_str}"));
            self.add_new_file(size, name);
        }
    }

    pub fn cd(&mut self, new_dir: &str) {
        // println!("cd'ing into {new_dir}");
        if new_dir == "/" {
            self.cwd = Inode(0);
        } else if new_dir == ".." {
            if let Some(Entry::Dir { parent, .. }) = self.fs.get(self.cwd) {
                self.cwd = *parent;
            }
        } else {
            self.cwd = self.fs.find_dir(self.cwd, new_dir);
        }
    }

    pub fn add_new_dir(&mut self, dir_name: &str) {
        // println!("Adding new directory: {dir_name}");
        let entry = Entry::Dir {
            name: dir_name.to_string(),
            parent: self.cwd,
            entries: Vec::new(),
        };
        let inode = self.fs.add_entry(entry);

        if let Entry::Dir { entries, .. } = self.fs.get_mut(self.cwd).unwrap() {
            entries.push(inode);
        }
    }

    pub fn add_new_file(&mut self, size: usize, name: &str) {
        // println!("Adding new file: {name}");
        let file_entry = Entry::File {
            size,
            name: name.to_string(),
        };
        let inode = self.fs.add_entry(file_entry);

        if let Entry::Dir { entries, .. } = self.fs.get_mut(self.cwd).unwrap() {
            entries.push(inode);
        }
    }

    pub fn compute_dir_sizes(&self) -> Vec<(String, usize)> {
        self.fs.compute_dir_sizes()
    }
}

// #[derive(Debug)]
// struct Fs {
//     root: Rc<Entry>,
// }

// impl Fs {
//     pub fn new() -> Self {
//         Self {
//             root: Rc::new(Entry::Dir {
//                 name: "/".to_string(),
//                 entries: Vec::new(),
//             }),
//         }
//     }
// }

#[derive(Debug)]
pub enum Entry {
    Dir {
        name: String,
        parent: Inode,
        entries: Vec<Inode>,
    },
    File {
        size: usize,
        name: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let data = r"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        let mut shell = Shell::new();
        for line in data.lines() {
            shell.parse_line(line);
        }
    }
}
