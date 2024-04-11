use std::{
    fs::File,
    io::{Read, Write},
};

use tinyfiledialogs::{open_file_dialog, save_file_dialog};

pub fn save(content: &str, path: &str) -> std::io::Result<()> {
    let mut f = File::create(path)?;
    f.write(content.as_bytes())?;
    Ok(())
}

pub fn load(path: &str) -> std::io::Result<String> {
    let mut f = File::open(path)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content)
}

pub fn select_save_file() -> Option<String> {
    save_file_dialog("Save File", ".")
}

pub fn select_open_file() -> Option<String> {
    open_file_dialog("Open File", ".", None)
}
