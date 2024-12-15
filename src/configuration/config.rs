use std::path::PathBuf;

use super::traits::BaseConfig;
use config::Config;
use serde::Deserialize;
//for erly dev errors
use crate::errors::Result;

const CONFIG_NAME: &str = "CommentAdder";
#[derive(Debug, Deserialize)]
pub struct CommentAdderConfig {
    pub text: String,
    pub path: PathBuf,
    pub file_formats: Vec<String>,
    // if max deph = 0 then
    pub max_deph: Option<usize>,
}

impl BaseConfig for CommentAdderConfig {
    type Output = Result<Self>;
    fn build_config() -> Self::Output {
        let config = Config::builder()
            .add_source(config::File::with_name(CONFIG_NAME))
            .build()?;
        let res = config.try_deserialize()?;
        Ok(res)
    }
}
