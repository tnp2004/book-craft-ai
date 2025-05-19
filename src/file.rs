use std::io::Write;
use std::{fs, io};

use base64::{Engine, engine::general_purpose};

use serde::Deserialize;

pub struct File;

#[derive(Deserialize, Debug)]
pub struct Instruction {
    pub instruction: String,
}

impl File {
    pub fn create_file(base64: &str, file_path: &str) -> Result<(), io::Error> {
        let image_data = general_purpose::STANDARD
            .decode(base64)
            .expect("Decode file failed");
        let mut file = fs::File::create(file_path).expect("Create file failed");
        file.write_all(&image_data)
            .expect("Write file buffer failed");

        Ok(())
    }

    pub fn read_instruction(path: &str) -> Instruction {
        let contents = fs::read_to_string(path).expect("Read instruction failed");
        let instruction: Instruction =
            serde_json::from_str(&contents).expect("Parse instruction failed");

        return instruction;
    }
}
