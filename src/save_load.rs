use std::{fs::File, io::Write};

pub fn save(content: &str) -> std::io::Result<()> {
    let path = "test.txt";
    let mut f = File::create(path)?;
    f.write(content.as_bytes())?;
    Ok(())
}
