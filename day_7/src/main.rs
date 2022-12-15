use std::env;
use std::collections::{HashMap, HashSet};
use std::cell::RefCell;
use std::fmt::Debug;

use log::debug;

use utils::{read_input, set_logging_level};

#[derive(Eq, Hash, PartialEq)]
struct File {
    name: String,
    size: usize
}

impl Debug for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} {}", self.size, self.name))
    }
}

#[derive(Debug)]
struct FileSystem {
    location: String,
    nodes: HashMap<String, RefCell<HashSet<File>>>
}

impl FileSystem {
    pub fn init_root() -> FileSystem{
        let mut fs = FileSystem {
            location: "/".to_string(),
            nodes: HashMap::new()
        };
        fs.mkdir(&"/");
        return fs;
    }

    fn ch(&mut self, dir: &str) {
        match dir {
            "/" => self.location = "/".to_string(),
            ".." => {
                let split: Vec<&str> = self.location[1..].split("/").collect();
                let mut new_location = "/".to_string();
                new_location.push_str(&split[..split.len() - 2].join("/"));
                if new_location != "/" {
                    new_location.push('/');
                }
                self.location = new_location;
            },
            &_ => {
                let mut new_location = self.location.to_owned();
                new_location.push_str(&format!("{dir}/"));
                self.mkdir(&new_location);
                self.location = new_location;
            }
        }
    }

    fn mkdir(&mut self, dir: &str) {
        self.nodes.entry(dir.to_string()).or_insert(RefCell::new(HashSet::new()));
    }

    fn new_file(&mut self, name: &str, size: usize) {
        self.nodes[&self.location].borrow_mut().insert(File {name: name.to_string(), size: size});
    }

    pub fn process_line(&mut self, line: &str) {
        if line.starts_with("$ cd") {
            self.ch(line.split(" ").last().unwrap())
        } else if line.starts_with("$ ls") {
            return;
        } else if line.starts_with("dir") {
            let dir: &str = line.split(" ").last().unwrap();
            let mut new_location = self.location.to_owned();
            new_location.push_str(&format!("{dir}/"));
            self.mkdir(&new_location)
        } else {
            let split: Vec<&str> = line.split(" ").collect();
            let name: &str = split[1];
            let size: usize = split[0].parse().unwrap();
            self.new_file(name, size)
        }
    }

    fn immediate_size(&self, dir: &str) -> usize {
        let mut size: usize = 0;

        for file in self.nodes[dir].borrow().iter() {
            size += file.size;
        }
        return size;
    }

    pub fn folder_sizes(&self) -> HashMap<String, usize> {
        let mut size_map: HashMap<String, usize> = HashMap::new();
        for dir in self.nodes.keys() {
            let mut size: usize = 0;
            for child in self.nodes.keys() {
                if child.starts_with(dir) {
                    size += self.immediate_size(child);
                }
            }
            size_map.insert(dir.to_string(), size);
        }
        return size_map;
    }
}

fn size_of_small_folder(size_map: &HashMap<String, usize>, max: &usize) -> usize {
    let mut total_size: usize = 0;
    for size in size_map.values() {
        if !(size > max) {
            total_size += size;
        }
    }
    return total_size;
}

fn smallest_possible_folder(size_map: &HashMap<String, usize>, min: &usize) -> (String, usize) {
    let mut smallest_dir: Option<&str> = None;
    let mut smallest_size: &usize = &usize::MAX;

    for (dir, size) in size_map.iter() {
        if !(size < min) & (size < &smallest_size) {
            smallest_dir = Some(dir);
            smallest_size = size;
        }
    }
    return (smallest_dir.expect("No directories over minimum size").to_string(), *smallest_size);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    set_logging_level(&args);
    let input = read_input(&args);
    let mut filesystem = FileSystem::init_root();

    for line in input.lines() {
        filesystem.process_line(line);
    }

    debug!("{:?}", filesystem);
    debug!("{:?}", filesystem.nodes.keys());
    let size_map = filesystem.folder_sizes();
    debug!("{:?}", size_map);
    let size_of_small = size_of_small_folder(&size_map, &100000);
    println!("Total size of small directories: {size_of_small}");

    let total_disk_space: usize = 70000000;
    let required_disk_space: usize = 30000000;
    let free_disk_space: usize = total_disk_space - size_map["/"];
    let min_deletion: usize = required_disk_space - free_disk_space;
    debug!("Free disk space: {free_disk_space}");
    debug!("Minimum folder size: {min_deletion}");
    let (smallest_dir, smallest_size) = smallest_possible_folder(&size_map, &min_deletion);
    println!("Smallest applicable directory: {smallest_dir} ({smallest_size})");
}
