use std::io::Write;
use std::fs;
use anyhow::Result;

use base64::{Engine, engine::general_purpose};

pub struct File;

impl File {
    pub fn create_file(base64: &str, file_path: &str) -> Result<()> {
        let image_data = general_purpose::STANDARD.decode(base64)?;
        let mut file = fs::File::create(file_path)?;
        file.write_all(&image_data)?;

        Ok(())
    }

    pub fn create_html(html: &[u8], file_path: &str) -> Result<()> {
        let mut file = fs::File::create(file_path)?;
        file.write_all(html)?;

        Ok(())
    }

    pub fn create_directory(images_dir: &str) -> Result<()> {
        fs::create_dir_all(images_dir)?;

        Ok(())
    }   

    pub fn read_instruction(path: &str) -> Result<String> {
        let instruction_txt = fs::read_to_string(path)?;

        Ok(instruction_txt)
    }
}
