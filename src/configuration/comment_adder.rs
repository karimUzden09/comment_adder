use serde::Deserialize;
use std::{collections::HashSet, path::PathBuf};

pub(super) const CONFIG_NAME: &str = "CommentAdder";

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

impl CommentAdderConfig {
    pub fn chceck_and_process_text(&mut self) {
        if self.text.ends_with("\n") {
            return;
        }
        self.text.push('\n');
    }
    pub fn check_end_line(&self) -> bool {
        self.text.ends_with("\n")
    }
}
