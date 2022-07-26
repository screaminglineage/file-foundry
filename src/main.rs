use file_foundry as ff;
use std::process;

const DEPTH: usize = 0;
const REGEX: &str = r"(\S*?)\{(\d+)\.\.(\d+)\}";

fn main() {
    let input = "file=={0..10}=={5..6///{5..100}..txt".to_string(); // Will be taken from user
    let extension;
    if let Some(ext) = ff::get_extension(&input) {
        extension = ext;
    } else {
        extension = "";
    }

    let captures = ff::get_regex(REGEX, &input);
    let names = get_filename_parts(captures);
    // println!("{:?}", names);    // testing code

    if let Err(e) = ff::create_file(DEPTH, &"".to_string(), &names, extension) {
        handle_io_error(e);
    }
    if let Err(e) = ff::create_folder(DEPTH, &"".to_string(), &names) {
        handle_io_error(e);
    }
}

// Extracts the digits matched in the regular expression
fn num_from_re_capture(capture: &regex::Captures, index: usize) -> Result<u32, String> {
    capture.get(index).map_or(
        Err("Failed to get lower/upper limit".to_string()),
        |m| match m.as_str().parse::<u32>() {
            Ok(n) => Ok(n),
            Err(s) => Err(s.to_string()),
        },
    )
}

// Shows error message and exits
fn handle_io_error(error: std::io::Error) {
    eprintln!(
        "{}\nMake sure none of the characters you included in the pattern are forbidden in file/folder names", 
        error
    );
    process::exit(1);
}

// Gets the filename parts from a vector of regex matches
fn get_filename_parts(captures: Vec<regex::Captures>) -> Vec<ff::FileName> {
    let mut names = Vec::new();
    
    for capture in captures {
        let prefix = capture.get(1).map_or("", |m| m.as_str()).to_string();
        let lower;
        let upper;

        if let Ok(l) = num_from_re_capture(&capture, 2) {
            lower = l;
        } else {
            eprintln!("Error in capturing lower limit from input");
            process::exit(1);
        }

        if let Ok(u) = num_from_re_capture(&capture, 3) {
            upper = u;
        } else {
            eprintln!("Error in capturing upper limit from input");
            process::exit(1);
        }

        // println!("PREFIX: {prefix}, LOWER: {lower}, UPPER: {upper}");    // testing code
        let filename = ff::FileName::new(prefix, lower, upper);
        names.push(filename);
    }
    names
}
