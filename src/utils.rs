use std::{
    env::args,
    time::{SystemTime, UNIX_EPOCH},
};

pub fn get_prompt() -> String {
    let args: Vec<String> = args().collect();
    let args_len = args.len();

    if args_len > 2 {
        panic!("Expected 1 got {} arguments", args_len - 1);
    } else if args_len < 2 {
        panic!("No prompt provided");
    }

    return args[1].clone();
}

pub fn generate_image_name(prefix: &str) -> String {
    let now = SystemTime::now();
    let duration_since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

    let timestamp = duration_since_epoch.as_secs();
    let file_name = format!("{}-{}.png", prefix, timestamp);

    return file_name;
}
