use std::io::{Error, Write};
use std::{fs, io};

use base64::{Engine, engine::general_purpose};

pub struct File;

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

    pub fn create_html(html: &str, file_path: &str) -> Result<(), io::Error> {
        let mut file = fs::File::create(file_path).expect("Create file failed");
        file.write_all(html.as_bytes()).expect("Write html file failed");

        Ok(())
    }

    pub fn create_directory(images_dir: &str) -> Result<(), Error> {
        fs::create_dir_all(images_dir)?;

        Ok(())
    }   

    pub fn read_instruction(path: &str) -> String {
        let instruction_txt = fs::read_to_string(path).expect("Read instruction failed");

        return instruction_txt;
    }
}
