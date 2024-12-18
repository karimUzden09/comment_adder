use crate::errors::Result;
use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::Path,
};

pub fn read_file<P: AsRef<Path>>(path: P, comment: String) -> Result<String> {
    // Reading file
    let f = File::open(&path)?;
    let mut buffer = BufReader::new(f);
    let mut string_buffer = comment;
    buffer.read_to_string(&mut string_buffer)?;
    Ok(string_buffer)
}
pub fn add_comment<P: AsRef<Path>>(path: P, comment: String) -> Result<()> {
    let buffer = read_file(&path, comment)?;
    write_text(&path, buffer.as_bytes())?;
    Ok(())
}
fn write_text<P: AsRef<Path>>(path: P, data: &[u8]) -> Result<()> {
    let mut writer = BufWriter::new(
        File::options()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)?,
    );
    writer.write_all(data)?;
    Ok(())
}
