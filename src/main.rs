use std::ffi::OsStr;
use std::path::Path;

use fs_err as fs;
use itertools::Itertools;
use walkdir::WalkDir;

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_dir = "Z:\\NextCloud\\frazzer\\files\\Photos";
    //let output_dir = "Z:\\NextCloud\\frazzer\\files\\Picture_Processing";
    let output_photo_dir = "Z:\\NextCloud\\frazzer\\files\\Picture_Processing\\pictures";
    let output_video_dir = "Z:\\NextCloud\\frazzer\\files\\Picture_Processing\\videos";
    let output_other_dir = "Z:\\NextCloud\\frazzer\\files\\Picture_Processing\\other";

    fs::create_dir_all(output_photo_dir)?;
    fs::create_dir_all(output_video_dir)?;
    fs::create_dir_all(output_other_dir)?;

    let photo_extensions = vec!["png", "jpg", "heic"];
    let video_extensions = vec!["mp4", "mov"];
    let other_extensions = vec!["gif"];
    let skip_extensions = vec!["md"];
    let mut unknown_extensions = vec![];

    for entry in WalkDir::new(input_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.metadata().unwrap().is_file() {
            let path = entry.path();
            //println!("{}", path.display());

            let extension = get_extension_from_filename(path.to_str().unwrap())
                .unwrap_or("missing")
                .to_lowercase();

            if photo_extensions.contains(&extension.as_str()) {
                println!("Photo:   {}", path.display());
            } else if video_extensions.contains(&extension.as_str()) {
                println!("Video:   {}", path.display());
            } else if other_extensions.contains(&extension.as_str()) {
                println!("Other:   {}", path.display());
            } else if skip_extensions.contains(&extension.as_str()) {
                println!("Skip:    {}", path.display());
            } else {
                println!("Unknown: {}", path.display());
                unknown_extensions.push(extension.to_string());
            }

            //    fs::rename(
            //        path,
            //        std::format!(
            //            "{output_dir}/{year}/{month}/{day}/{year}-{month}-{day}_{hash}.{extension}"
            //        ),
            //    )?;
        }
    }

    let unknown_extensions: Vec<_> = unknown_extensions.into_iter().unique().collect();
    println!("Unknown Extension: {:#?}", unknown_extensions);

    Ok(())
}
