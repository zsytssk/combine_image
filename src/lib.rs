mod img_map;

mod state;
mod utils;

use async_std::fs;
use async_std::task;
use state::State;
use std::time::Instant;
use utils::{img::save, path, walk_path};

pub fn run(
    src: &str,
    dist: &str,
    json_suffix: &str,
    space_width: i32,
    space_height: i32,
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
    State::init(src, dist, json_suffix, space_width, space_height, prefix);

    let mut tasks: Vec<task::JoinHandle<()>> = vec![];
    for path in paths {
        let task_item = task::spawn(async move {
            let state = State::get();
            let src = state.src.to_owned();
            let dist = state.dist.to_owned();
            let json_suffix = state.json_suffix.to_owned();

            let mut map_item = img_map::run(path).await;
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

            fs::write(dist_atlas_path, map_item.to_json())
                .await
                .expect("Unable to write file");
            save(buffer, dist_img_path).await;

            let mut n = State::get().n.lock().unwrap();
            *n += 1;
            println!("combine:> {}% -- {}", (*n * 100) / all, file_path);
        });

        tasks.push(task_item);
    }

    task::block_on(async {
        for task_item in tasks {
            task_item.await;
        }
    });

    println!("completed:> {}", now.elapsed().as_millis());
}
