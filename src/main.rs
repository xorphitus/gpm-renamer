use id3::Tag;
use regex::{Regex, NoExpand};
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} have to specify a mp3 file name", &args[0]);
        return;
    }

    let file_path_str = &args[1];
    let file_path = Path::new(file_path_str);

    let file_name = file_path.file_name().unwrap().to_str().unwrap();

    let tag = Tag::read_from_path(file_path_str).unwrap();
    let track = format!("{:02}", tag.track().unwrap());

    let re = Regex::new(r".+ -").unwrap();
    let new_name = re.replace(file_name, NoExpand(&track)).into_owned();
    let dir = file_path.parent().unwrap();
    let new_path = dir.join(new_name);

    match fs::rename(file_path_str, new_path) {
        Ok(_) => {}
        Err(e) => { eprintln!("{}", e); }
    }
}
