use std::{collections::HashSet, path::PathBuf};

use super::traits::BaseConfig;
use config::Config;
use jwalk::WalkDir;
use serde::Deserialize;
//for erly dev errors
use crate::errors::Result;

const CONFIG_NAME: &str = "CommentAdder";
#[derive(Debug, Deserialize)]
pub struct CommentAdderConfig {
    pub text: String,
    pub path: PathBuf,
    pub file_formats: HashSet<String>,
    // if max deph = 0 then
    pub max_deph: Option<usize>,
    pub work_mode: Mode,
    pub patterns_match: Option<usize>,
}
#[derive(Debug, Deserialize, Default)]
pub enum Mode {
    #[default]
    AddText,
    RemoveText,
}

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
}

impl CommentAdderConfig {
    pub fn get_walkdir(&self) -> WalkDir {
        if let Some(max_depth) = self.max_deph {
            return WalkDir::new(&self.path).max_depth(max_depth);
        }
        WalkDir::new(&self.path)
    }
    pub fn chceck_and_process_text(&mut self) {
        if self.text.ends_with("\n") {
            return;
        }
        self.text.push_str("\n");
    }
    pub fn check_end_line(&self) -> bool {
        self.text.ends_with("\n")
    }
}

#[test]
fn test_check_end_line() -> Result<()> {
    let str: &str = "Hello";
    dbg!(str.ends_with("\n"));
    let str_2: &str = "Hello\n";
    print!("{}", str_2);
    dbg!(str_2.ends_with("\n"));

    Ok(())
}
