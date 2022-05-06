use chrono::{DateTime, Utc};
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

fn hash(file: &str) -> String {
    let computed_hash = sha256::digest_file(std::path::Path::new(&file)).unwrap();
    computed_hash
}

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_dir = "./test_files/input";
    let output_dir = "./test_files/output";
    //let input_dir = "Z:/data/tmp/input";
    //let output_dir = "Z:/data/tmp/output";
    fs::create_dir_all(output_dir)?;

    for entry in WalkDir::new(input_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.metadata().unwrap().is_file() {
            let path = entry.path();
            println!("{}", path.display());

            if let Ok(time) = entry.metadata().unwrap().created() {
                let date = DateTime::<Utc>::from(time).date();
                let year = date.format("%Y").to_string();
                let month = date.format("%m").to_string();
                let day = date.format("%d").to_string();
                let hash = hash(path.to_str().unwrap());
                let extension =
                    get_extension_from_filename(path.to_str().unwrap()).unwrap_or("missing");

                //println!("{year}-{month}-{day}");
                //println!("{hash}");

                fs::create_dir_all(std::format!("{output_dir}/{year}/{month}/{day}"))?;

                fs::rename(
                    path,
                    std::format!(
                        "{output_dir}/{year}/{month}/{day}/{year}-{month}-{day}_{hash}.{extension}"
                    ),
                )?;
            } else {
                println!("Not supported on this platform or filesystem");
            }
        }
    }

    Ok(())
}
