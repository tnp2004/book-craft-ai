use std::env::args;

pub fn get_prompt() -> String {
    let args: Vec<String> = args().collect();
    let args_len = args.len();

    if args_len > 2 {
        panic!("Expected 1 got {} arguments", args_len - 1);
    }else if args_len < 2 {
        panic!("No prompt provided");
    }

    args[1].clone()
}