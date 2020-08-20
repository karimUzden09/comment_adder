extern crate comment_adder;
extern crate clap;

use comment_adder::core::*;
use clap::{App,Arg};


fn main() -> std::io::Result<()>  {
    let matches = App::new("Test App")
    .author("Uzdenov Karim")
    .about("This programm adds the ability to automatically add comments to source files.")
    .version("0.1.0")
    .usage("CommnetAdder [--add_comment, --delate_comment] <FILE>")
        .arg(
            Arg::with_name("add")
            .takes_value(true)
            .long("add_comment")
            .short("add")
            .multiple(true)
            .required(true)
            .conflicts_with("del")
            .help("Added a comment that is specified in settings.json")
            .display_order(1)            
        )
        .arg(
            Arg::with_name("del")
            .takes_value(true)
            .long("delete_comment")
            .short("del")
            .multiple(true)
            .required(true)
            .conflicts_with("add")
            .help("Deleted a comment that is specified in settings.json")
            .display_order(2)
        )
        .get_matches();


    if let Some(ref in_file) = matches.value_of("add") {
        add_comment(in_file)?;
    }
    
    if let Some(ref in_file) = matches.value_of("del") {
        delete_comment_v2(in_file)?;

    }


    Ok(())
}
