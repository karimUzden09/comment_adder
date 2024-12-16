use std::ffi::OsStr;
use std::sync::Mutex;

use file_reader::write_text;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::configuration::config::CommentAdderConfig;
use crate::errors::Result;
pub mod file_reader;

pub fn run(config: CommentAdderConfig) -> Result<()> {
    let walk_dir = config.get_walkdir();
    let context = Mutex::new(config);

    walk_dir
        .into_iter()
        .par_bridge()
        .filter_map(|e| e.ok())
        .for_each(|entry| {
            if entry.file_type().is_file() {
                let path = entry.path();
                if let Some(extension) = path.extension().and_then(OsStr::to_str) {
                    let contex_lock_guard = context.lock().expect("ERROR IN LOCKING MUTEX");
                    if contex_lock_guard.file_formats.contains(extension) {
                        write_text(path, contex_lock_guard.text.clone()).expect("Write error");
                        // or delete text
                    }
                }
            }
        });

    todo!()
}
