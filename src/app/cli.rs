use super::core::{
    add_comment,
    delete_comment_v2,
    settigs_file
};
use structopt::StructOpt;

#[derive(Debug,StructOpt)]
#[structopt(name = "Comment Adder",about = "This programm adds the ability to automatically add comments to source files. By Uzdenov Karim  2020")]
enum Opt {
    #[structopt(about = "Added a comment that is specified in settings.json")]
    Add {
        file_path: String
    },

    #[structopt(about = "Deleted a comment that is specified in settings.json")]
    Del {
        file_path: String
    },
    
    #[structopt(about = "Open settings.json")]
    Settings

}


pub fn run() ->std::io::Result<()> {
    let opt = Opt::from_args();
    match opt  {
        Opt::Add {file_path} => add_comment(&file_path.as_str())?,
        Opt::Del{file_path} => delete_comment_v2(&file_path.as_str())?,
        Opt::Settings => settigs_file()?

    }
    Ok(())

}



