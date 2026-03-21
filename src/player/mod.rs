pub mod queue;
pub mod sink;

use std::path::PathBuf;

pub const OUTPUT_DIR: &str = "output";

pub fn clean_old_output() -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = PathBuf::from(OUTPUT_DIR);

    for entry in std::fs::read_dir(output_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            std::fs::remove_dir_all(&path)?;
        } else {
            std::fs::remove_file(&path)?;
        }
    }

    Ok(())
}
