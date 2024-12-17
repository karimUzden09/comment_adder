use mktemp::Temp;

use crate::errors::Result;
use std::{
    fs::{self, File},
    io::{self, BufWriter, Write},
    path::Path,
};

pub fn add_text(path: &Path, comment: String) -> Result<()> {
    //let readed_file = fs::read_to_string(path)?;
    let mut writer = BufWriter::new(File::create(path)?);
    write!(&mut writer, "{}", comment)?;
    // check comment for new line (/n) in the end of the comment;
    // if have /n continume
    // else havent /n add to comment and write to the file
    //prepend_file(res.as_bytes(), path)?;
    Ok(())
}
pub fn remove_text(path: &Path, comment: &str, patterns_match: Option<usize>) -> Result<()> {
    let patterns_match = patterns_match.unwrap_or(0);
    let readed_file = fs::read_to_string(path)?;
    let res = readed_file.replacen(comment, "", patterns_match);
    prepend_file(res.as_bytes(), path)?;
    Ok(())
}
fn prepend_file(data: &[u8], file_path: &Path) -> Result<()> {
    // Create a temporary file
    let tmp_path = Temp::new_file()?;
    // Stop the temp file being automatically deleted when the variable
    // is dropped, by releasing it.
    let tmp_path = tmp_path.release();
    // Open temp file for writing
    let mut tmp = File::create(&tmp_path)?;
    // Open source file for reading
    let mut src = File::open(&file_path)?;
    // Write the data to prepend
    tmp.write_all(&data)?;
    // Copy the rest of the source file
    io::copy(&mut src, &mut tmp)?;
    fs::remove_file(&file_path)?;
    fs::rename(&tmp_path, &file_path)?;
    Ok(())
}
#[test]
fn test_add_to_file_comment() -> Result<()> {
    let mut source = fs::read_to_string("futex.h")?;

    let mut comment = "//Autor by karim\n".to_string();
    let mut res = comment.clone() + &source;
    let r = res.replacen(&comment, "", 1);
    print!("{r}");
    Ok(())
}
