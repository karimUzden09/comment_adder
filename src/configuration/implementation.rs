use std::{
    ffi::OsStr,
    path::Path,
    sync::atomic::{AtomicUsize, Ordering},
};

use config::Config;
use jwalk::WalkDir;

use super::{
    comment_adder::{CommentAdderConfig, CONFIG_NAME},
    traits::{BaseConfig, FileProccesing},
    utils::add_comment,
};
use crate::errors::Result;

impl BaseConfig for CommentAdderConfig {
    type Output = Result<Self>;
    fn build_config() -> Self::Output {
        let config = Config::builder()
            .add_source(config::File::with_name(CONFIG_NAME))
            .build()?;
        let mut config = config.try_deserialize::<CommentAdderConfig>()?;
        config.chceck_and_process_text();
        Ok(config)
    }
    fn build_walker_dir(&self) -> WalkDir {
        if let Some(max_depth) = self.max_deph {
            return WalkDir::new(&self.path).max_depth(max_depth);
        }
        WalkDir::new(&self.path)
    }
}

impl FileProccesing for CommentAdderConfig {
    type FileProcessingOutput = ();
    fn process(
        &self,
        path: &Path,
        counter: Option<&AtomicUsize>,
    ) -> Result<Self::FileProcessingOutput> {
        if let Some(extension) = path.extension().and_then(OsStr::to_str) {
            if self.file_formats.contains(extension) {
                match self.work_mode {
                    super::comment_adder::Mode::AddText => add_comment(path, self.text.clone())?,
                    super::comment_adder::Mode::RemoveText => unimplemented!(),
                }
                counter.map(|c| c.fetch_add(1, Ordering::SeqCst));
            }
        }
        Ok(())
    }
}
