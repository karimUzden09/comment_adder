use crate::configuration::config::CommentAdderConfig;
use crate::configuration::traits::BaseConfig;
use crate::errors::Result;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct ContextInner<C: BaseConfig> {
    config: C,
}

impl<C: BaseConfig> ContextInner<C> {
    pub fn new(config: C) -> Self {
        Self { config }
    }
}

#[derive(Debug)]
pub struct Context<C: BaseConfig> {
    config: Mutex<ContextInner<C>>,
}

pub type ArcContex<C> = Arc<Context<C>>;
