use std::time::Duration;

use anyhow::{Ok, Result};
use indicatif::ProgressBar;
use walkdir::WalkDir;

pub async fn run(path: String) -> Result<()> {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_message("Scanning files..");

    let mut count = 0;
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            count += 1;
        }
    }

    pb.finish_with_message("Done");
    println!("Found {} files", count);

    Ok(())
}