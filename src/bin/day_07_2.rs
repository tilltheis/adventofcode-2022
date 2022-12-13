extern crate core;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!(
        "{}",
        deletable_directory_size(DirectorySizes::new(&read_file_system(
            "src/bin/day_07_input.txt"
        )))
    );
}

#[derive(Debug, Eq, PartialEq)]
struct FileNode {
    name: String,
    size: usize,
}

#[derive(Debug, Eq, PartialEq)]
struct DirectoryNode {
    name: String,
    files: Vec<FileNode>,
    directories: Vec<DirectoryNode>,
}

#[derive(Debug)]
struct FileSystemZipper {
    parent_with_index: Option<(Box<FileSystemZipper>, usize)>,
    node: DirectoryNode,
}

impl FileSystemZipper {
    pub fn new() -> FileSystemZipper {
        FileSystemZipper {
            parent_with_index: None,
            node: DirectoryNode {
                name: "/".to_string(),
                directories: vec![],
                files: vec![],
            },
        }
    }

    pub fn up(self) -> FileSystemZipper {
        let node = self.node;
        let (mut parent, index_in_parent) = self.parent_with_index.unwrap();
        parent.node.directories.push(node);
        let len = parent.node.directories.len();
        parent.node.directories.swap(index_in_parent, len - 1);
        *parent
    }

    pub fn down(mut self, dirname: &str) -> FileSystemZipper {
        let index = self
            .node
            .directories
            .iter()
            .position(|d| d.name == dirname)
            .unwrap();
        let node = self.node.directories.swap_remove(index);
        FileSystemZipper {
            parent_with_index: Some((Box::new(self), index)),
            node,
        }
    }

    pub fn root(self) -> FileSystemZipper {
        let mut cwd = self;
        while cwd.parent_with_index.is_some() {
            cwd = cwd.up();
        }
        cwd
    }

    pub fn add_file(&mut self, name: String, size: usize) -> () {
        self.node.files.push(FileNode { name, size });
    }

    pub fn add_directory(&mut self, name: String) -> () {
        self.node.directories.push(DirectoryNode {
            name,
            directories: vec![],
            files: vec![],
        });
    }
}

fn read_file_system(path: &str) -> DirectoryNode {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|x| x.unwrap()).peekable();

    let mut cwd = FileSystemZipper::new();

    while let Some(line) = lines.next() {
        if let Some(dirname) = line.strip_prefix("$ cd ") {
            match dirname {
                "/" => cwd = cwd.root(),
                ".." => cwd = cwd.up(),
                _ => cwd = cwd.down(dirname),
            }
        } else if line == "$ ls" {
            while !lines.peek().iter().all(|l| l.starts_with('$')) {
                if let Some(line) = lines.next() {
                    if let Some(dirname) = line.strip_prefix("dir ") {
                        cwd.add_directory(dirname.to_string());
                    } else if let Some((size, filename)) = line.split_once(' ') {
                        cwd.add_file(filename.to_string(), size.parse().unwrap());
                    } else {
                        panic!("Cannot parse ls output line {}", line);
                    }
                }
            }
        }
    }

    cwd.root().node
}

#[derive(Clone)]
struct DirectorySizesState<'a> {
    node: &'a DirectoryNode,
    index: usize,
    size: usize,
}

impl<'a> DirectorySizesState<'a> {
    fn new(node: &DirectoryNode) -> DirectorySizesState {
        DirectorySizesState {
            node,
            index: 0,
            size: 0,
        }
    }
}

#[derive(Clone)]
struct DirectorySizes<'a> {
    queue: Vec<DirectorySizesState<'a>>,
}

impl<'a> DirectorySizes<'a> {
    fn new(node: &DirectoryNode) -> DirectorySizes {
        DirectorySizes {
            queue: vec![DirectorySizesState::new(node)],
        }
    }
}

impl<'a> Iterator for DirectorySizes<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(DirectorySizesState { node, index, size }) = self.queue.last() {
            if *index >= node.directories.len() {
                let files_size: usize = node.files.iter().map(|f| f.size).sum();
                let total_node_size = *size + files_size;

                self.queue.pop();

                if let Some(parent) = self.queue.last_mut() {
                    parent.index += 1;
                    parent.size += total_node_size;
                }

                return Some(total_node_size);
            } else {
                self.queue
                    .push(DirectorySizesState::new(&node.directories[*index]));
            }
        }

        None
    }
}

fn deletable_directory_size(all_directory_sizes: DirectorySizes) -> usize {
    const TOTAL_DISK_SPACE: usize = 70_000_000;
    const REQUIRED_DISK_SPACE: usize = 30_000_000;

    let used_disk_space = all_directory_sizes.clone().last().unwrap();
    let available_disk_space = TOTAL_DISK_SPACE - used_disk_space;
    let missing_disk_space = REQUIRED_DISK_SPACE - available_disk_space;

    all_directory_sizes
        .filter(|&x| x >= missing_disk_space)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn read_file_system_works() {
        assert_eq!(
            read_file_system("src/bin/day_07_test_input.txt"),
            create_test_file_system()
        );
    }

    #[test]
    fn directory_sizes_works1() {
        assert_eq!(
            DirectorySizes::new(&DirectoryNode {
                name: "/".to_string(),
                files: vec![
                    FileNode {
                        name: "b.txt".to_string(),
                        size: 14848514,
                    },
                    FileNode {
                        name: "c.dat".to_string(),
                        size: 8504156,
                    },
                ],
                directories: vec![],
            })
            .collect::<Vec<usize>>(),
            vec![23_352_670]
        );
    }

    #[test]
    fn directory_sizes_works2() {
        assert_eq!(
            DirectorySizes::new(&create_test_file_system()).collect::<Vec<usize>>(),
            vec![584, 94853, 24933642, 48381165]
        );
    }

    #[test]
    fn deletable_directory_size_works() {
        assert_eq!(
            deletable_directory_size(DirectorySizes::new(&create_test_file_system())),
            24933642
        );
    }

    fn create_test_file_system() -> DirectoryNode {
        DirectoryNode {
            name: "/".to_string(),
            files: vec![
                FileNode {
                    name: "b.txt".to_string(),
                    size: 14848514,
                },
                FileNode {
                    name: "c.dat".to_string(),
                    size: 8504156,
                },
            ],
            directories: vec![
                DirectoryNode {
                    name: "a".to_string(),
                    files: vec![
                        FileNode {
                            name: "f".to_string(),
                            size: 29116,
                        },
                        FileNode {
                            name: "g".to_string(),
                            size: 2557,
                        },
                        FileNode {
                            name: "h.lst".to_string(),
                            size: 62596,
                        },
                    ],
                    directories: vec![DirectoryNode {
                        name: "e".to_string(),
                        files: vec![FileNode {
                            name: "i".to_string(),
                            size: 584,
                        }],
                        directories: vec![],
                    }],
                },
                DirectoryNode {
                    name: "d".to_string(),
                    files: vec![
                        FileNode {
                            name: "j".to_string(),
                            size: 4060174,
                        },
                        FileNode {
                            name: "d.log".to_string(),
                            size: 8033020,
                        },
                        FileNode {
                            name: "d.ext".to_string(),
                            size: 5626152,
                        },
                        FileNode {
                            name: "k".to_string(),
                            size: 7214296,
                        },
                    ],
                    directories: vec![],
                },
            ],
        }
    }
}
