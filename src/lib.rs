use regex::Regex;
use std::fs::{self, File};
use std::path::Path;

const F_PATH: &str = "/home/aditya/test/files/"; // Delete when done testing
const D_PATH: &str = "/home/aditya/test/dirs/"; // Delete when done testing

#[derive(Debug)]
pub struct FileName {
    prefix: String,
    lower: u32,
    upper: u32,
}

impl FileName {
    pub fn new(prefix: String, lower: u32, upper: u32) -> FileName {
        FileName {
            prefix,
            lower,
            upper,
        }
    }
}

pub fn get_extension<'a>(input: &'a String) -> Option<&'a str> {
    Some(Path::new(input).extension()?.to_str()?)
}

pub fn get_regex<'a>(expr: &str, data: &'a String) -> Vec<regex::Captures<'a>> {
    let re = Regex::new(expr).unwrap();
    re.captures_iter(data).collect::<Vec<regex::Captures>>()
}

pub fn create_file(depth: usize, name: &String, filename: &Vec<FileName>, extension: &str) -> std::io::Result<()> {
    let filepart: &FileName;
    if let Some(f) = filename.get(depth) {
        filepart = f;
    } else {
        return Ok(());
    }

    for i in filepart.lower..filepart.upper + 1 {
        if depth == filename.len() - 1 {
            // println!("{}{}{}", &name, filepart.prefix, i);
            File::create(format!(
                "{}{}{}{}.{}",
                F_PATH, &name, filepart.prefix, i, extension
            ))?;
            // remove F_PATH when done testing
        }
        let name = format!("{}{}{}", name, filepart.prefix, i);
        create_file(depth + 1, &name, filename, extension)?;
    }
    Ok(())
}

pub fn create_folder(depth: usize, name: &String, dirname: &Vec<FileName>) -> std::io::Result<()> {
    let dirpart: &FileName;
    if let Some(d) = dirname.get(depth) {
        dirpart = d;
    } else {
        return Ok(());
    }

    for i in dirpart.lower..dirpart.upper + 1 {
        if depth == dirname.len() - 1 {
            // println!("{}{}{}", &name, filepart.prefix, i);
            fs::create_dir(format!("{}{}{}{}", D_PATH, &name, dirpart.prefix, i))?;
            // remove D_PATH when done testing
        }
        let name = format!("{}{}{}", name, dirpart.prefix, i);
        create_folder(depth + 1, &name, dirname)?;
    }
    Ok(())
}
