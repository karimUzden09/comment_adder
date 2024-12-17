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
// pub fn remove_text(path: &Path, comment: &str, patterns_match: Option<usize>) -> Result<()> {
//     unimplemented!()
//     // let patterns_match = patterns_match.unwrap_or(0);
//     // let readed_file = fs::read_to_string(path)?;
//     // let res = readed_file.replacen(comment, "", patterns_match);
//     // todo!()
//     //Ok(())
// }

// #[test]
// fn test_add_to_file_comment() -> Result<()> {
//     let source = fs::read_to_string("futex.h")?;

//     let comment = "//Autor by karim\n".to_string();
//     let res = comment.clone() + &source;
//     let r = res.replacen(&comment, "", 1);
//     print!("{r}");
//     Ok(())
// }
#[test]
fn test_writer() -> Result<()> {
    let file = File::open("futex.h")?;
    let reader = BufReader::new(file);
    dbg!(reader.bytes());

    Ok(())
}

#[test]
fn test_add_text() -> Result<()> {
    add_comment("futex.h", "//Lol\n".to_owned())?;

    Ok(())
}
