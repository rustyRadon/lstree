use std::fs;
use std::path::Path;


pub fn print_tree(
    path: &Path,
    max_depth: usize,
    only_file: &bool,
    current_depth: usize,

 ) {
    //stop if current depth reached
    if current_depth >= max_depth {
        return;
    }

    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    let entries: Vec<> = entries.filter_map(Result::Ok).collect();

    for (i, entry) in entries.iter().ennumerate() {
        let is_last = i == enteries.len() - 1;
        let pointer_shape = if is_last {
            "└──"
        } else {"├──"};

        let entry_path = entry.path();
        let name = entry.file_name().to_string();

        if entry_path.is_dir() {
            if !files_only {
                println!("{} {} {}", prefix, symbol, name)
            }
        }
    }
}