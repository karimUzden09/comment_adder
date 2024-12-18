use std::{sync::atomic::AtomicUsize, time::Instant};

use jwalk::{
    rayon::iter::{ParallelBridge, ParallelIterator},
    Parallelism,
};

use crate::{configuration::traits::FileProccesing, errors::Result};

pub fn execute_parallel<P: FileProccesing>(file_proccesor: P) -> Result<()> {
    let start = Instant::now();
    let modified_files_count = AtomicUsize::new(0);
    let walk_dir = file_proccesor.build_walker_dir();
    walk_dir
        .parallelism(Parallelism::RayonNewPool(0))
        .into_iter()
        .par_bridge()
        .filter_map(|e| e.ok())
        .for_each(|entry| {
            file_proccesor
                .process(&entry.path(), Some(&modified_files_count))
                .expect("File processing error");
        });

    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
    println!("Files modifiered {}", modified_files_count.into_inner());
    Ok(())
}
