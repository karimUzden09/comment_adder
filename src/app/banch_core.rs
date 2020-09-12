extern crate colored;
use colored::*;
//use serde::{Deserialize, Serialize};
use walkdir::{WalkDir};
use std::{fs,io::BufWriter, /*io::BufReader,*/ io::Write,io::Read, fs::File,/* path::Path*/};
use std::collections::VecDeque;
use indicatif::ProgressBar;
//use std::time::{Instant};
pub use std::io::prelude::*;

extern crate rayon;
use rayon::prelude::*;
use std::sync::{Arc,Mutex};

use super::core::*;
use crate::info_message;

// Bench versions functions add_comment_bench

pub fn add_comment_bench(path : &&str,settings_path : &str)->std::io::Result<()> {
    let mut settings_fille = File::open(settings_path).expect("Unable to open");
    let mut data = String::new();
    settings_fille.read_to_string(&mut data)?;
    
    
    let mut  json_settings: CommentStruct = serde_json::from_str(&data).expect("Json was not well format");

   
    pirnt_json_settings(&json_settings);
    
    
    
    json_settings.comments.reverse();
    for extensions in json_settings.file_name_extension.iter() {
        let walker = WalkDir::new(path).into_iter();
        for entry in walker {
            let entry = entry.unwrap();
            let file_name = entry.path().file_name().unwrap().to_str().unwrap();
            if file_name.ends_with(extensions) {
                //info_message!("{}",file_name);
               
    
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
    println!("{}","-------------------------DONE-------------------------".bold().green());
    Ok(())


}
#[inline(always)]
pub fn scunn_filles_bench(path : &&str,settings_path : &str) -> Option<u64> {
    let mut settings_fille = File::open(settings_path).expect("Unable to open");
    let mut data = String::new();
    settings_fille.read_to_string(&mut data).unwrap();
    let mut files_count : u64 = 0;
    
    let json_settings:CommentStruct = serde_json::from_str(&data).expect("Json was not well format");

    for extensions in json_settings.file_name_extension.iter() {
        let walker = WalkDir::new(path).into_iter();
        
        
        for entry in walker {
            let entry = entry.unwrap();
            let file_name = entry.path().file_name().unwrap().to_str().unwrap();
            if file_name.ends_with(extensions) {
                files_count += 1;

            }
        }
    }

    info_message!("files count {}",files_count);
    Some(files_count)
    
}

#[inline(always)]
pub fn paralel_scan_files_bench (path : &&str, settings_path : &str) -> Option<usize> {
    let mut settings_fille = File::open(settings_path).expect("Unable to open");
    let mut data = String::new();
    settings_fille.read_to_string(&mut data).unwrap();
    
    let json_settings:CommentStruct = serde_json::from_str(&data).expect("Json was not well format");
     
    let extensions_count = json_settings.file_name_extension.len();

    let summ_vector = Arc::new(Mutex::new(Vec::new()));
    
     let par_vec: Vec<_> = (0..extensions_count).into_par_iter().map(|ext| {
        let mut files_count : usize = 0;

        for entry in WalkDir::new(path).into_iter() {
            let entry = entry.unwrap();
            let file_name = entry.path().file_name().unwrap().to_str().unwrap();

            if file_name.ends_with(&json_settings.file_name_extension[ext]) {
                files_count += 1;

                //println!("{}",files_count);
            }
        }
        summ_vector.lock().unwrap().push(files_count);

     }).collect();


     let  files_count = summ_vector.lock().unwrap().iter().sum();
     info_message!("files count {}",files_count);
    Some(files_count)
}


pub fn add_comment_v2_bench(path : &&str,settings_path : &str)->std::io::Result<()> {
    let mut settings_fille = File::open(settings_path).expect("Unable to open");
    let mut data = String::new();
    settings_fille.read_to_string(&mut data)?;
    
    let files_count = scunn_filles_bench(path, settings_path).unwrap();
    let bar = ProgressBar::new(files_count);
    
    let mut  json_settings:CommentStruct = serde_json::from_str(&data).expect("Json was not well format");
    
    json_settings.comments.reverse();
    for extensions in json_settings.file_name_extension.iter() {
        let walker = WalkDir::new(path).into_iter();
        for entry in walker {
            let entry = entry.unwrap();
            let file_name = entry.path().file_name().unwrap().to_str().unwrap();
            if file_name.ends_with(extensions) {
                //info_message!("{}",file_name);
    
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
                
                bar.inc(1);
    
            }
    
            
        }
    }
    bar.finish();
    info_message!("{}","-------------------------DONE-------------------------".bold().green());
    
    
    Ok(())
}
