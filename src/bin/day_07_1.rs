extern crate core;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!(
        "{}",
        sum_of_small_directory_sizes(all_directory_sizes(&read_file_system(
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

struct DirectorySizes<'a> {
    // current node, current index, current size
    queue: Vec<(&'a DirectoryNode, usize, usize)>,
}

impl<'a> Iterator for DirectorySizes<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((node, index, size)) = self.queue.last() {
            if *index >= node.directories.len() {
                let files_size: usize = node.files.iter().map(|f| f.size).sum();
                let total_node_size = *size + files_size;

                self.queue.pop();

                if let Some(parent) = self.queue.last_mut() {
                    parent.1 += 1;
                    parent.2 += total_node_size;
                }

                return Some(total_node_size);
            } else {
                self.queue.push((&node.directories[*index], 0, 0))
            }
        }

        None
    }
}

fn all_directory_sizes(file_system: &DirectoryNode) -> DirectorySizes {
    DirectorySizes {
        queue: vec![(file_system, 0, 0)],
    }
}

fn sum_of_small_directory_sizes(all_directory_sizes: impl Iterator<Item = usize>) -> usize {
    all_directory_sizes.filter(|&x| x < 100_000).sum()
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
    fn all_directory_sizes_works1() {
        assert_eq!(
            all_directory_sizes(&DirectoryNode {
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
    fn all_directory_sizes_works2() {
        assert_eq!(
            all_directory_sizes(&create_test_file_system()).collect::<Vec<usize>>(),
            vec![584, 94853, 24933642, 48381165]
        );
    }

    #[test]
    fn sum_of_small_directory_sizes_works() {
        assert_eq!(
            sum_of_small_directory_sizes(all_directory_sizes(&create_test_file_system())),
            95437
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
