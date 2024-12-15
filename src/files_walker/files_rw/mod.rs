use std::ffi::OsStr;

use file_reader::write_text;
use walkdir::WalkDir;

use crate::configuration::config::CommentAdderConfig;
use crate::errors::Result;

pub mod file_reader;
pub mod file_writer;

pub fn run(config: CommentAdderConfig) -> Result<()> {
    let walk_dir = if let Some(max_depth) = config.max_deph {
        WalkDir::new(config.path).max_depth(max_depth)
    } else {
        WalkDir::new(config.path)
    };
    for entry in walk_dir.into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path();

            if let Some(extension) = path.extension().and_then(OsStr::to_str) {
                if check_extension(extension, &config.file_formats) {
                    write_text(path, config.text.clone())?;
                }
            }
        }
    }
    todo!()
}

fn check_extension(current_extension: &str, extensions: &[String]) -> bool {
    extensions.iter().any(|e| e == current_extension)
}
