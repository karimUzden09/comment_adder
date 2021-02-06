extern crate colored;
use crate::info_message;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
pub use std::io::prelude::*;
use std::time::Instant;
use std::{fs, fs::File, io::BufReader, io::BufWriter, io::Read, io::Write, path::Path};
use walkdir::WalkDir;

use std::sync::{Arc, Mutex};

extern crate rayon;
use rayon::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct CommentStruct {
    pub file_name_extension: Vec<String>,
    pub comments: Vec<String>,
}

pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn pirnt_json_settings(json_settings: &CommentStruct) {
    for name in json_settings.file_name_extension.iter() {
        info_message!(
            "{} {}",
            "Settings file name extensions: ".bold().blue(),
            name.bold()
        );
    }

    for comment in json_settings.comments.iter() {
        info_message!(
            "{} {}",
            "Comments from the settings file:".bold().blue(),
            comment.bold()
        );
    }
}

pub fn settigs_file() -> std::io::Result<()> {
    let mut settings_fille = File::open("settings.json").expect("Unable to open settings.json");
    let mut data = String::new();
    settings_fille.read_to_string(&mut data)?;

    let json_settings: CommentStruct =
        serde_json::from_str(&data).expect("Json was not well format");

    pirnt_json_settings(&json_settings);
    Ok(())
}

pub fn delate_comment_parallel(path: &&str) {
    let total = Instant::now();

    let mut settings_fille = File::open("settings.json").expect("Unable to open");
    let mut data = String::new();
    settings_fille
        .read_to_string(&mut data)
        .expect("erorre of reading settings.json");

    let json_settings: CommentStruct =
        serde_json::from_str(&data).expect("Json was not well format");
    paralel_scan_files(path).unwrap();

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(120);
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&[
                "▹▹▹▹▹",
                "▸▹▹▹▹",
                "▹▸▹▹▹",
                "▹▹▸▹▹",
                "▹▹▹▸▹",
                "▹▹▹▹▸",
                "▪▪▪▪▪",
            ])
            .template("{spinner:.green} {msg}"),
    );
    pb.set_message("Deleting comments...");

    pirnt_json_settings(&json_settings);

    let extensions_count = json_settings.file_name_extension.len();

    let _par_vec: Vec<_> = (0..extensions_count)
        .into_par_iter()
        .map(|ext| {
            for entry in WalkDir::new(path).into_iter() {
                let entry = entry.unwrap();
                let file_name = entry.path().file_name().unwrap().to_str().unwrap();
                if file_name.ends_with(&json_settings.file_name_extension[ext]) {
                    let mut strings_buffer = lines_from_file(entry.path());

                    for comm in json_settings.comments.iter() {
                        strings_buffer.retain(|x| x != comm);
                        let new_file = File::create(entry.path()).expect("Unable create file");
                        for comment in &strings_buffer {
                            let mut writer = BufWriter::new(&new_file);
                            writeln!(&mut writer, "{}", comment).unwrap();
                        }
                    }
                }
            }
        })
        .collect();

    pb.finish();
    println!("Total time: {}", total.elapsed().as_secs_f32());
    println!(
        "{}",
        "-------------------------DONE-------------------------"
            .bold()
            .green()
    );
}

pub fn scun_wraper(path: &&str) -> std::io::Result<()> {
    paralel_scan_files(path).expect("Errore to scun");
    Ok(())
}

#[inline(always)]
pub fn paralel_scan_files(path: &&str) -> Option<usize> {
    let mut settings_fille = File::open("settings.json").expect("Unable to open");
    let mut data = String::new();
    settings_fille.read_to_string(&mut data).unwrap();

    let json_settings: CommentStruct =
        serde_json::from_str(&data).expect("Json was not well format");
    let extensions_count = json_settings.file_name_extension.len();

    let summ_vector = Arc::new(Mutex::new(Vec::new()));

    let _par_vec: Vec<_> = (0..extensions_count)
        .into_par_iter()
        .map(|ext| {
            let mut files_count: usize = 0;

            for entry in WalkDir::new(path).into_iter() {
                let entry = entry.unwrap();
                let file_name = entry.path().file_name().unwrap().to_str().unwrap();

                if file_name.ends_with(&json_settings.file_name_extension[ext]) {
                    files_count += 1;

                    //println!("Debug: {}",files_count);
                }
            }
            summ_vector.lock().unwrap().push(files_count);
        })
        .collect();

    let files_count = summ_vector.lock().unwrap().iter().sum();
    info_message!("files count {}", files_count);
    Some(files_count)
}

#[inline(always)]
pub fn add_comment_progres_paralel(path: &&str) {
    let total = Instant::now();
    let mut settings_fille = File::open("settings.json").expect("Unable to open");
    let mut data = String::new();
    settings_fille.read_to_string(&mut data).unwrap();

    paralel_scan_files(path).unwrap();

    let mut json_settings: CommentStruct =
        serde_json::from_str(&data).expect("Json was not well format");
    pirnt_json_settings(&json_settings);

    json_settings.comments.reverse();

    let extensions_count = json_settings.file_name_extension.len();

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(120);
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&[
                "▹▹▹▹▹",
                "▸▹▹▹▹",
                "▹▸▹▹▹",
                "▹▹▸▹▹",
                "▹▹▹▸▹",
                "▹▹▹▹▸",
                "▪▪▪▪▪",
            ])
            .template("{spinner:.green} {msg}"),
    );
    pb.set_message("Addition comments...");
    let _par_vec: Vec<_> = (0..extensions_count)
        .into_par_iter()
        .map(|ext| {
            for entry in WalkDir::new(path).into_iter() {
                let entry = entry.unwrap();
                let file_name = entry.path().file_name().unwrap().to_str().unwrap();

                if file_name.ends_with(&json_settings.file_name_extension[ext]) {
                    let mut file = fs::OpenOptions::new()
                        .write(true)
                        .append(true)
                        .read(true)
                        .open(entry.path())
                        .unwrap();
                    let mut temp_data = String::new();
                    file.read_to_string(&mut temp_data).unwrap();
                    let mut string_buffer: VecDeque<String> = VecDeque::new();

                    string_buffer.push_back(temp_data);

                    for comments in json_settings.comments.iter() {
                        string_buffer.push_front(comments.clone());
                    }
                    let new_file = File::create(entry.path()).expect("Unable create file");
                    for comment in string_buffer {
                        let mut writer = BufWriter::new(&new_file);
                        writeln!(&mut writer, "{}", comment).unwrap();
                    }
                }
            }
        })
        .collect();
    pb.finish();
    println!("Total time: {}", total.elapsed().as_secs_f32());
    info_message!(
        "{}",
        "-------------------------DONE-------------------------"
            .bold()
            .green()
    );
}
