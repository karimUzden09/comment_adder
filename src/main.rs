use comment_adder::configuration::{config::CommentAdderConfig, traits::BaseConfig};
use comment_adder::errors::Result;
use comment_adder::files_walker::files_rw::run;
fn main() -> Result<()> {
    let config: CommentAdderConfig = <CommentAdderConfig as BaseConfig>::build_config()?;
    run(config)?;
    Ok(())
}
