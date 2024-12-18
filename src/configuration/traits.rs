use crate::errors::Result;
use jwalk::WalkDir;
use std::{path::Path, sync::atomic::AtomicUsize};

pub trait BaseConfig: Send + Sync {
    type Output;
    fn build_config() -> Self::Output;
    fn build_walker_dir(&self) -> WalkDir;
}

pub trait FileProccesing: BaseConfig {
    type FileProcessingOutput;
    fn process(
        &self,
        path: &Path,
        counter: Option<&AtomicUsize>,
    ) -> Result<Self::FileProcessingOutput>;
}
