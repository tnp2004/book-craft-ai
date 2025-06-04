use std::{
    env::args,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::{anyhow, Ok, Result};
use loading::Loading;

pub fn get_prompt() -> Result<String> {
    let args: Vec<String> = args().collect();
    let args_len = args.len();

    if args_len > 2 {
        return Err(anyhow!("Expected 1 got {} arguments", args_len - 1));
    } else if args_len < 2 {
        return Err(anyhow!("No prompt provided"));
    }

    Ok(args[1].clone())
}

pub fn generate_image_name(prefix: &str) -> Result<String> {
    let now = SystemTime::now();
    let duration_since_epoch = now.duration_since(UNIX_EPOCH)?;

    let timestamp = duration_since_epoch.as_millis();
    let file_name = format!("{}-{}.png", prefix, timestamp);

    Ok(file_name)
}

pub fn create_dir_name(dir: String) -> String {
    dir.replace(" ", "-")
}

pub fn create_loader(text: &str) -> Loading {
    let loading = Loading::default();

    loading.text(text);

    loading
}
