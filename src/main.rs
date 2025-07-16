use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    search_string: String,

    #[arg(short, long, default_value = "/")]
    file_path: String,
}

fn main() {
    let args = Args::parse();
    let dirs = collect_dirs(Path::new(&args.file_path).into());
    let results = search_names(dirs,args.search_string);

    println!("search results: {results:?}");

}

fn collect_dirs(path: &Path) -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    let dir_listings = match fs::read_dir(path) {
        Ok(listing) => listing,
        Err(e) => {
            eprintln!("Error: {}", e);
            return dirs;
        }
    };

    dir_listings
        .filter_map(|x| {
            x.ok().and_then(|entry| Some(entry.path()))
        })
        .for_each(|dir| {
            if dir.is_dir() && !dir.is_symlink() {
                dirs.extend(collect_dirs(&dir));
            }
            dirs.push(dir);
        });
    
    dirs
}

fn search_names(names: Vec<PathBuf>, search_string: String) -> Vec<PathBuf> {
    names
        .into_iter()
        .filter(|name| name
            .file_name()
            .unwrap()
            .to_string_lossy()
            .contains(&search_string)
        )
        .collect()
}
