extern crate comment_adder;
use comment_adder::app;

fn main() -> std::io::Result<()> {
    app::cli::run()?;
    Ok(())
}
