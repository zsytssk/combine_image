#[macro_use]
extern crate lazy_static;

mod img_map;

mod state;
mod utils;
use std::fs;

use std::thread;
use std::time::Instant;
use utils::{path, walk_path};

pub fn run(
    src: &str,
    dist: &str,
    json_suffix: &str,
    space_width: u32,
    space_height: u32,
    prefix: &str,
) {
    let now = Instant::now();


    let paths = walk_path::run(src);

    println!(
        "file_num = {} :> {}",
        &paths.len(),
        now.elapsed().as_millis()
    );

    let all = paths.len();
    (&state::STATE).lock().unwrap().init(
        src,
        dist,
        json_suffix,
        space_width,
        space_height,
        0,
        prefix,
    );

    let mut handles = vec![];
    for path in paths {
        let handle = thread::spawn(move || {
            let state = (&state::STATE).lock().unwrap();
            let src = (&state.src).to_owned();
            let dist = (&state.dist).to_owned();
            let json_suffix = (&state.json_suffix).to_owned();
            drop(state);

            let mut map_item = img_map::run(path);
            let mut file_path = path::relative(&map_item.name, &src).unwrap();
            if &file_path == "" {
                file_path = path::file_name(&map_item.name).to_owned();
            }
            let buffer = map_item.combine();
            let dir_path = format!("{}/{}", &dist, file_path);
            let (dir_path, file_name) = path::split_path(&dir_path);
            match path::create_dir(&dir_path) {
                Err(_) => panic!("cant create dir {}", &dir_path),
                _ => {}
            };
            let dist_img_path = format!("{}/{}.png", dir_path, file_name);
            let dist_atlas_path = format!("{}/{}.{}", dir_path, file_name, &json_suffix);

            fs::write(dist_atlas_path, map_item.to_json()).expect("Unable to write file");
            buffer.save(dist_img_path).unwrap();

            let mut state = (&state::STATE).lock().unwrap();
            state.n += 1;
            println!("combine:> {}% -- {}", state.n * 100 / all, file_path);
            drop(state);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    println!("completed:> {}", now.elapsed().as_millis());
}
