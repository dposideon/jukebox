pub mod queue;
pub mod sink;

use crate::paths::output_dir;

pub const OUTPUT_DIR: &str = "output";

pub fn clean_old_output() -> Result<(), Box<dyn std::error::Error>> {
    for entry in std::fs::read_dir(output_dir())? {
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
