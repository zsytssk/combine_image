
use std::fs;
use std::path::Path;
pub fn is_exist(path_str: &str) -> bool {
    let path = Path::new(path_str);
    path.exists()
}
pub fn path_type(path_str: &str) -> &str {
    let path = Path::new(path_str);
    if path.is_dir() {
        return "is_dir";
    } else {
        return "is_file";
    }
}
pub fn file_name(path_str: &str) -> &str {
    let path = Path::new(path_str);
    path.file_name().unwrap().to_str().unwrap()
}

pub fn dir_path(path_str: &str) -> &str {
    let path = Path::new(path_str);
    path.parent().unwrap().to_str().unwrap()
}

pub fn normalize(path_str: &str) -> String {
    path_str.replace("//", "/").replace("\\", "/")
}

/**
 * 将路径的最后一个拆分下来 作为文件名
 */
pub fn split_path(path_str: &str) -> (String, String) {
    let base = Path::new(path_str);
    let parent = base.parent().unwrap().to_str().unwrap().to_owned();
    let name = base.file_name().unwrap().to_str().unwrap().to_owned();
    (parent, name)
}

pub fn relative(file: &str, base: &str) -> Result<String, String> {
    let base = Path::new(base);
    let file = Path::new(file);
    match file.strip_prefix(base) {
        Ok(file_path) => {
            let file_path = match file_path.to_str() {
                Some(path) => Ok(normalize(path)),
                None => Err("path cant to_str".to_owned()),
            };
            return file_path;
        }
        Err(e) => Err("fail to get re".to_owned()),
    }
}

pub fn create_dir(dir_path: &str) -> std::io::Result<()> {
    let dir_path = Path::new(dir_path);
    fs::create_dir_all(dir_path)
}


#[test]
fn name() {
    let a = relative(
        "D:\\zsytssk\\github\\combine_image\\test\\src\\t1\\t2",
        "D:\\zsytssk\\github\\combine_image\\test\\src",
    );
}
