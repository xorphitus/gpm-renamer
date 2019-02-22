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

    let track = match Tag::read_from_path(file_path_str) {
        Ok(tag) => tag.track().map(|tr| format!("{:02}", tr)),
        Err(e) => panic!("failed to get mp3 tag: {}, {}", file_path_str, e),
    };

    let track = match track {
        Some(t) => t,
        None => panic!("failed to get track: {}", file_path_str),
    };

    let re = Regex::new(r".+ -").unwrap();
    let new_name = re.replace(file_name, NoExpand(&track)).into_owned();
    let dir = file_path.parent().unwrap();
    let new_path = dir.join(new_name);

    match fs::rename(file_path_str, new_path) {
        Ok(_) => {}
        Err(e) => { eprintln!("{}", e); }
    }
}
