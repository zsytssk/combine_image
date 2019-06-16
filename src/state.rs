
use std::sync::{Mutex};

lazy_static! {
    pub static ref STATE: Mutex<State> = Mutex::new(State::new());
}

#[derive(Debug)]
pub struct State {
    pub src: String,
    pub dist: String,
    pub json_suffix: String,
    pub prefix: String,
    pub space_width: i32,
    pub space_height: i32,
    pub n: usize,
}

impl State {
    pub fn new() -> State {
        State {
            src: "".to_owned(),
            dist: "".to_owned(),
            json_suffix: "".to_owned(),
            prefix: "".to_owned(),
            space_width: 0,
            space_height: 0,
            n: 0,
        }
    }
    pub fn init(
        &mut self,
        src: &str,
        dist: &str,
        json_suffix: &str,
        space_width: i32,
        space_height: i32,
        n: usize,
        prefix: &str,
    ) {
        self.src = src.to_owned();
        self.dist = dist.to_owned();
        self.json_suffix = json_suffix.to_owned();
        self.space_width = space_width;
        self.space_height = space_height;
        self.n = n;
        self.prefix = prefix.to_owned();;
    }
}

function initState() {

}