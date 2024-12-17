use std::ffi::OsStr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use file_reader::{add_text, remove_text};
use jwalk::Parallelism;
//use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::configuration::config::{CommentAdderConfig, Mode};
use crate::configuration::traits::BaseConfig;
use crate::errors::Result;
use jwalk::rayon::iter::{ParallelBridge, ParallelIterator};
use std::time::{Duration, Instant};
pub mod file_reader;

pub fn run(config: CommentAdderConfig) -> Result<()> {
    let start = Instant::now();
    let walk_dir = config.get_walkdir();
    let context = config;
    let modified_files_count = AtomicUsize::new(0);
    walk_dir
        .parallelism(Parallelism::RayonNewPool(0))
        .into_iter()
        .par_bridge()
        .filter_map(|e| e.ok())
        .for_each(|entry| {
            if entry.file_type().is_file() {
                let path = entry.path();
                if let Some(extension) = path.extension().and_then(OsStr::to_str) {
                    let contex_lock_guard = &context;
                    if contex_lock_guard.file_formats.contains(extension) {
                        match contex_lock_guard.work_mode {
                            Mode::AddText => {
                                add_text(&path, contex_lock_guard.text.clone())
                                    .expect("Write error");
                            }
                            Mode::RemoveText => remove_text(
                                &path,
                                &contex_lock_guard.text,
                                contex_lock_guard.patterns_match,
                            )
                            .expect("Remove comment error"),
                        }
                        modified_files_count.fetch_add(1, Ordering::SeqCst);
                    }
                }
            }
        });

    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
    println!("Files modifiered {}", modified_files_count.into_inner());
    Ok(())
}

#[test]
fn test_run() -> Result<()> {
    let config: CommentAdderConfig = <CommentAdderConfig as BaseConfig>::build_config()?;
    dbg!(&config);
    run(config)?;
    Ok(())
}
