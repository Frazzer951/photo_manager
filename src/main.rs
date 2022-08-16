use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use fs_err as fs;
use itertools::Itertools;
use walkdir::WalkDir;

const DRY_RUN: bool = false;

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}

fn get_filename(filename: String, extension: &str, path: &str) -> PathBuf {
    let mut loops = 0;
    loop {
        let mut filepath = PathBuf::from(path);
        let filename = if loops != 0 {
            format!("{}_{}.{}", filename, loops, extension)
        } else {
            format!("{}.{}", filename, extension)
        };
        filepath.push(filename);

        // if filepath doesnt exist
        if !filepath.exists() {
            println!("VP: {}", filepath.display());

            return filepath;
        }
        loops += 1;
        println!("PT: {}", filepath.display());
    }
}

fn main() {
    //let input_dir = "Z:\\NextCloud\\frazzer\\files\\Photos";
    //let output_photo_dir = "Z:\\NextCloud\\frazzer\\files\\Picture_Processing\\pictures";
    //let output_video_dir = "Z:\\NextCloud\\frazzer\\files\\Picture_Processing\\videos";
    //let output_other_dir = "Z:\\NextCloud\\frazzer\\files\\Picture_Processing\\other";

    let input_dir = "C:\\Users\\luke3\\Desktop\\tmp\\input";
    let output_photo_dir = "C:\\Users\\luke3\\Desktop\\tmp\\pictures";
    let output_video_dir = "C:\\Users\\luke3\\Desktop\\tmp\\videos";
    let output_other_dir = "C:\\Users\\luke3\\Desktop\\tmp\\other";

    match fs::create_dir_all(output_photo_dir) {
        Ok(_) => {},
        Err(e) => {
            println!("Failed to create output_photo_dir {} - {}", output_photo_dir, e);
            return;
        },
    }
    match fs::create_dir_all(output_video_dir) {
        Ok(_) => {},
        Err(e) => {
            println!("Failed to create output_photo_dir {} - {}", output_video_dir, e);
            return;
        },
    }
    match fs::create_dir_all(output_other_dir) {
        Ok(_) => {},
        Err(e) => {
            println!("Failed to create output_photo_dir {} - {}", output_other_dir, e);
            return;
        },
    }

    let photo_extensions = vec!["png", "jpg", "heic"];
    let video_extensions = vec!["mp4", "mov"];
    let other_extensions = vec!["gif"];
    let skip_extensions = vec!["md"];
    let mut unknown_extensions = vec![];

    for entry in WalkDir::new(input_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.metadata().unwrap().is_file() {
            let path = entry.path();
            //println!("{}", path.display());

            let extension = get_extension_from_filename(path.to_str().unwrap()).unwrap_or("missing");

            let filename = path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
                .replace(&format!(".{}", extension), "");
            //println!("FN: {}", filename);
            //println!("EX: .{}", extension);

            let file_move = |from: &Path, to: &Path| {
                if DRY_RUN {
                    println!("MS: {} -> {}", from.display(), to.display());
                } else {
                    match fs::rename(from, to) {
                        Ok(_) => {
                            println!("MS: {} -> {}", from.display(), to.display());
                        },
                        Err(e) => {
                            println!("MF: {} ", e);
                        },
                    }
                }
            };

            if photo_extensions.contains(&extension.to_lowercase().as_str()) {
                println!("P:  {}", path.display());
                let new_path = get_filename(filename, extension, output_photo_dir);
                println!("MT: {}", new_path.display());
                file_move(path, &new_path);
            } else if video_extensions.contains(&extension.to_lowercase().as_str()) {
                println!("V:  {}", path.display());
                let new_path = get_filename(filename, extension, output_video_dir);
                println!("MT: {}", new_path.display());
                file_move(path, &new_path);
            } else if other_extensions.contains(&extension.to_lowercase().as_str()) {
                println!("O:  {}", path.display());
                let new_path = get_filename(filename, extension, output_other_dir);
                println!("MT: {}", new_path.display());
                file_move(path, &new_path);
            } else if skip_extensions.contains(&extension.to_lowercase().as_str()) {
                println!("S:  {}", path.display());
            } else {
                println!("U:  {}", path.display());
                unknown_extensions.push(extension.to_string());
            }
        }
    }

    let unknown_extensions: Vec<_> = unknown_extensions.into_iter().unique().collect();
    println!("Unknown Extension: {:#?}", unknown_extensions);
}
