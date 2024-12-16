//for erly dev errors

pub trait BaseConfig {
    type Output;
    fn build_config() -> Self::Output;
}

pub trait ComentAdderConfigTrait: BaseConfig {
    type Walker;
    fn build_walker(&self) -> Self::Walker;
}
