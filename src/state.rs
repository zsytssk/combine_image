use once_cell::sync::OnceCell;
use std::sync::Mutex;

#[derive(Debug)]
pub struct State {
    pub src: String,
    pub dist: String,
    pub json_suffix: String,
    pub prefix: String,
    pub space_width: i32,
    pub space_height: i32,
    pub n: Mutex<usize>,
}

static INSTANCE: OnceCell<State> = OnceCell::new();

impl State {
    pub fn get() -> &'static State {
        INSTANCE.get().expect("State is not initialized")
    }
    pub fn init(
        src: &str,
        dist: &str,
        json_suffix: &str,
        space_width: i32,
        space_height: i32,
        prefix: &str,
    ) {
        let state = State {
            src: src.to_owned(),
            dist: dist.to_owned(),
            json_suffix: json_suffix.to_owned(),
            prefix: prefix.to_owned(),
            space_width: space_width,
            space_height: space_height,
            n: Mutex::new(0),
        };

        INSTANCE.set(state).unwrap();
    }
}
