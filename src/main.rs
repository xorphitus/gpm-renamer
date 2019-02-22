use id3::{Tag};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} have to specify a mp3 file name", &args[0]);
        return;
    }
    let filename = &args[1];
    let tag = Tag::read_from_path(filename).unwrap();
    println!("{}", tag.track().unwrap());
}
