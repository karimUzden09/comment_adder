//for erly dev errors

pub trait BaseConfig {
    type Output;
    fn build_config() -> Self::Output;
}
