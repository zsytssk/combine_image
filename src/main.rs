use lib::run;
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    println!("{:?}", args);
    if args.len() < 5 || args.len() > 6 {
        args_err_tip();
    }
    let src = &args[0];
    let dist = &args[1];
    let json_suffix = &args[2];
    let space_width = args[3].parse::<i32>().unwrap();
    let space_height = args[4].parse::<i32>().unwrap();
    let mut prefix = "";
    if args.len() == 6 {
        prefix = &args[5];
    }
    run(src, dist, json_suffix, space_width, space_height, prefix);
}

fn args_err_tip() {
    panic!("combine src dist json_suffix space_width space_height prefix");
}
