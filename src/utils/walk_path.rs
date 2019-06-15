use super::path::{is_exist};
use std::fs::read_dir;

#[derive(Debug)]
pub struct PathMap {
    name: String,
    list: Vec<String>,
}

impl PathMap {
    pub fn new(name: String) -> PathMap {
        PathMap { name, list: vec![] }
    }
    pub fn add(&mut self, item: String) {
        self.list.push(item);
    }
    pub fn data(self) -> (String, Vec<String>) {
        (self.name, self.list)
    }
}

pub fn run(path_str: &str) -> Vec<PathMap> {
    let mut result: Vec<PathMap> = vec![];
    map_dir(path_str, &mut result);
    return result;
}

fn map_dir(path_str: &str, list: &mut Vec<PathMap>) {
    if !is_exist(path_str) {
        panic!("path not exist");
    }
    let mut path_map = PathMap::new(String::from(path_str));

    for entry in read_dir(path_str).unwrap() {
        let entry_path = entry.unwrap().path();
        let entry_str = entry_path.to_str().unwrap();
        if (&entry_path).is_dir() {
            map_dir(entry_str, list);
        } else if (&entry_path).is_file() {
            let extension = match entry_path.extension() {
                Some(ext) => ext.to_str().unwrap(),
                None => "",
            };
            if extension == "png" {
                path_map.add(entry_str.to_owned());
            }
        }
    }
    if path_map.list.len() > 0 {
        list.push(path_map);
    }
}

#[test]
fn name() {
    let a = run("./test");
    for path in a {
        // println!("Name: {:?}", path)
    }
}
