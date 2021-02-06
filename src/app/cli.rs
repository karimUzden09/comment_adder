use super::core::{
    add_comment_progres_paralel, delate_comment_parallel, scun_wraper, settigs_file,
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Comment Adder",
    about = "This programm adds the ability to automatically add comments to source files. By Uzdenov Karim  2020"
)]
enum Opt {
    #[structopt(about = "Added a comment that is specified in settings.json")]
    Add { file_path: String },

    #[structopt(about = "Deleted a comment that is specified in settings.json")]
    Del { file_path: String },
    // #[structopt(about = "Added a comment that is specified in settings.json and progress bar")]
    // Addprog {
    //     file_path: String
    // },
    #[structopt(about = "Open settings.json")]
    Settings,

    #[structopt(
        about = "Recursive scan directory and finde files with extension witch defined in settings.json"
    )]
    Scun { file_path: String },
}

pub fn run() -> std::io::Result<()> {
    let opt = Opt::from_args();
    match opt {
        Opt::Add { file_path } => add_comment_progres_paralel(&file_path.as_str()),
        Opt::Del { file_path } => delate_comment_parallel(&file_path.as_str()),
        Opt::Settings => settigs_file()?,
        //Opt::Addprog{file_path} => add_comment_progres(&file_path.as_str())?,
        Opt::Scun { file_path } => scun_wraper(&file_path.as_str())?,
    }
    Ok(())
}
