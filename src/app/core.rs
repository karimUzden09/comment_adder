extern crate colored;
use colored::*;
use serde::{Deserialize, Serialize};
use walkdir::{WalkDir};
use std::{fs,io::BufWriter, io::BufReader, io::Write,io::Read, fs::File, path::Path};
use std::collections::VecDeque;
pub use std::io::prelude::*;

#[macro_export]
macro_rules! info_message {
    () => {
        let message = "[info]";
        println!("{}", message.bold().green());
    };
    ($($arg:tt)*) => {
        let message ="[info]";
        print!("{} ",message.bold().green());
        println!($($arg)*);
    }
}

#[derive(Serialize, Deserialize)]
pub struct CommentStruct 
{
    file_name_extension: Vec<String>,
    comments: Vec<String>,
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
    .map(|l| l.expect("Could not parse line"))
    .collect()
}

fn pirnt_json_settings(json_settings : &CommentStruct) {
    
    for name in json_settings.file_name_extension.iter(){
        info_message!("{} {}","Settings file name extensions: ".bold().blue(),name.bold());
    }
    
    for comment in json_settings.comments.iter(){
        info_message!("{} {}","Comments from the settings file:".bold().blue(),comment.bold());
    }
}

pub fn settigs_file () -> std::io::Result<()> {
    let mut settings_fille = File::open("settings.json").expect("Unable to open");
    let mut data = String::new();
    settings_fille.read_to_string(&mut data)?;
    
    
    let json_settings:CommentStruct = serde_json::from_str(&data).expect("Json was not well format");
 
    pirnt_json_settings(&json_settings);
    Ok(())


}

pub fn delete_comment_v2(path : &&str)->std::io::Result<()> {
    let mut settings_fille = File::open("settings.json").expect("Unable to open");
    let mut data = String::new();
    settings_fille.read_to_string(&mut data)?;
    
    
    let json_settings:CommentStruct = serde_json::from_str(&data).expect("Json was not well format");

 
    pirnt_json_settings(&json_settings);

    for extensions in json_settings.file_name_extension.iter() {
        let walker = WalkDir::new(path).into_iter();
        for entry in walker {
            let entry = entry.unwrap();
            let file_name = entry.path().file_name().unwrap().to_str().unwrap();
            if file_name.ends_with(extensions) {
                info_message!("{}",file_name);

               
    
              let mut strings_buffer = lines_from_file(entry.path()); 

                
                for comm in json_settings.comments.iter() {
                    strings_buffer.retain(|x| x != comm);
                                   
                 }

                 
                 let  new_file = File::create(entry.path()).expect("Unable create file");
                 for comment in strings_buffer {
                 let mut writer = BufWriter::new(&new_file);
                 writeln!(&mut writer,"{}",comment)?;
                
                 }     
                
    
    
            }
    
            
        }
    }
    println!("{}","-------------------------DONE-------------------------".green());
    Ok(())
}

pub fn add_comment(path : &&str)->std::io::Result<()> {
    let mut settings_fille = File::open("settings.json").expect("Unable to open");
    let mut data = String::new();
    settings_fille.read_to_string(&mut data)?;
    
    
    let mut  json_settings:CommentStruct = serde_json::from_str(&data).expect("Json was not well format");

   
    pirnt_json_settings(&json_settings);
    
    
    json_settings.comments.reverse();
    for extensions in json_settings.file_name_extension.iter() {
        let walker = WalkDir::new(path).into_iter();
        for entry in walker {
            let entry = entry.unwrap();
            let file_name = entry.path().file_name().unwrap().to_str().unwrap();
            if file_name.ends_with(extensions) {
                info_message!("{}",file_name);
               
    
               let mut file = fs::OpenOptions::new()
                    .write(true)
                    .append(true)
                    .read(true)
                    .open(entry.path())
                    .unwrap();
                let mut temp_data = String::new();
                
                file.read_to_string(&mut temp_data)?;
    
                 let mut string_buffer: VecDeque<String> = VecDeque::new();
                 
                 string_buffer.push_back(temp_data);
                 
                 for comments in json_settings.comments.iter() {
                    string_buffer.push_front(comments.clone());
                 }
                let new_file = File::create(entry.path()).expect("Unable create file");
                for comment in string_buffer {
                    let mut writer = BufWriter::new(&new_file);
                    writeln!(&mut writer,"{}",comment)?;
                }     
                
    
    
            }
    
            
        }
    }
    println!("{}","-------------------------DONE-------------------------".green());
    Ok(())


}