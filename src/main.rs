mod treeview;

use std::{
    fs, path::{Path, PathBuf}, sync::{Arc, Mutex}, thread, time::Duration
};
use treeview::{TreeEntry, TreeModel};
use clap::Parser;


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
    //let dirs = collect_dirs(Path::new(&args.file_path).into());
    //let results = search_names(dirs,args.search_string);

    //println!("search results: {results:?}");
    tree_test();

}

fn tree_test() {
    let tree = Arc::new(Mutex::new(TreeModel::new()));
    let tree_handle = TreeModel::start(Arc::clone(&tree));

    let ex1 = TreeEntry {
        name: "Example 1".to_string(),
        is_dir: true,
        depth: 0,
    };
    let ex2 = TreeEntry {
        name: "Example 2".to_string(),
        is_dir: false,
        depth: 1,
    };

    thread::sleep(Duration::from_secs(1));
    
    tree.lock().unwrap().add_entry(ex1);
    tree.lock().unwrap().add_entry(ex2);

    tree_handle.join().unwrap();

}

/// Given a Path object, traverse all sub directories under that path
/// and return results as a vector of PathBuf objects.
/// 
/// When a new directory is encounter, the recursive call takes place
/// and dirs vector is extended by the return value. Otherwise, 
/// entries (including the dir) are pushed into the dirs vector
/// 
/// TODO: Add the ability to limit path traversal depth
/// 
/// 
fn collect_dirs(path: &Path) -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    //let pool = ThreadPool::new(4);

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
