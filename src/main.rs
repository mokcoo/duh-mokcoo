use std::{fs, io, io::Write, path::PathBuf, process};
// use jwalk::WalkDir;
use std::os::unix::fs::MetadataExt;
use clap::Parser;


mod options;
mod crossdev;
mod traverse;
fn cwd_dirlist() -> Result<Vec<PathBuf>, io::Error> {
    let mut v: Vec<_> = fs::read_dir(".")?
        .filter_map(|e: Result<fs::DirEntry, io::Error>| {
            e.ok()
                .and_then(|e| e.path().strip_prefix(".").ok().map(ToOwned::to_owned))
        })
        .filter(|p| {
            if let Ok(meta) = p.symlink_metadata() {
                if meta.file_type().is_symlink() {
                    return false;
                }
            };
            true
        })
        .collect();
    v.sort();
    Ok(v)
}


fn extract_paths_maybe_set_cwd(
    mut paths: Vec<PathBuf>,
    cross_filesystems: bool,
) -> Result<Vec<PathBuf>, io::Error> {
    if paths.len() == 1 && paths[0].is_dir() {
        std::env::set_current_dir(&paths[0])?;
        paths.clear();
    }
    let device_id = std::env::current_dir()
        .ok()
        .and_then(|cwd| crossdev::init(&cwd).ok());

    if paths.is_empty() {
        cwd_dirlist().map(|paths| match device_id {
            Some(device_id) if !cross_filesystems => paths
                .into_iter()
                .filter(|p| match p.metadata() {
                    Ok(meta) => crossdev::is_same_device(device_id, &meta),
                    Err(_) => true,
                })
                .collect(),
            _ => paths,
        })
    } else {
        Ok(paths)
    }
}

type WalkDir = jwalk::WalkDirGeneric<((), Option<Result<std::fs::Metadata, jwalk::Error>>)>;

fn agg() {
    let paths = vec![];
    let ignore_dir = vec![PathBuf::from("target".to_string())];
    let mut total = 0u128;
    for f in  extract_paths_maybe_set_cwd(paths, false).unwrap().iter(){
        // println!("{}", f.display());
        // if f.is_dir() {
        //     println!("{} is dir", f.display());
        // }
        // if ignore_dir.contains(&f) {
        //     continue;
        // }
        for path in WalkDir::new(f).follow_links(false).sort(true).process_read_dir(|_,_,_,dir_entry_result| {
            dir_entry_result.iter_mut().for_each(|dir_entry_result| {
                if let Ok(dir_entry) = dir_entry_result{
                    // println!("{}", dir_entry.path().display());
                    let metadata = dir_entry.metadata();
                    dir_entry.client_state = Some(metadata);
                    // dir_entry.
                }
            });
        }) {
            match path {
                Ok(entry) => {
                    // let entry = entry.path();
                    // if entry.is_file() {
                    //     println!("{}", entry.display());
                    // }
                    // if entry.display().
                    let path_name = entry.path().display().to_string();
                    let device_id = entry.metadata().map(|m| m.dev()).unwrap();
                    let file_len = match entry.client_state {
                        Some(Ok(meta)) => meta.len(),
                        _ => 0,
                    };
                    total += file_len as u128;
                    // println!("{} device_id: {}, file len: {}", path_name, device_id, file_len);

                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    process::exit(1);
                }
            }
        }

    };
    println!("Total: {}", total);
    cwd_dirlist().map(|f| {
        for p in f {
            println!("{}", p.display());;
        }
    }).unwrap();
    println!("Hello, world!");
}

fn int(){

}
fn main() {
    let opt = options::Arg::parse_from(wild::args_os());
    let num_cpu = num_cpus::get();
    // agg();

}

#[cfg(test)]
mod tests {
    use super::*;
    use wild::args_os;
    use options::Arg;

    #[test]
    fn test_wild_parser_with_args() {
        let args = vec!["duam", "x"];
        let opt = options::Arg::parse_from(args.iter().map(|s| s));
        
        assert_eq!(opt.stay_on_filesystem, true);
    }
}