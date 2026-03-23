use super::*;

use std::io::Cursor;
use std::path::{Path, PathBuf};

//pub fn ffmpeg_on_path() -> bool {
//    std::process::Command::new("ffmpeg")
//        .arg("-version")
//        .output()
//        .is_ok()
//}

pub async fn download_ffmpeg(
    lib_dir: &Path,
    link: &str,
    os: &Os,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    println!("Downloading ffmpeg from {}", link);

    let response = reqwest::get(link).await?;

    if !response.status().is_success() {
        return Err(format!("Download failed: HTTP {}", response.status()).into());
    }

    let bytes = response.bytes().await?;

    println!("Downloaded compressed ffmpeg from {}", link);

    let extracted = match os {
        Os::Mac => extract_ffmpeg_zip(&bytes, lib_dir)?,
        Os::Linux => extract_ffmpeg_tar_xz(&bytes, lib_dir)?,
    };

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&extracted, std::fs::Permissions::from_mode(0o755))?;
    }

    println!("Extracted ffmpeg to {}", extracted.display());

    Ok(extracted)
}

fn extract_ffmpeg_zip(
    bytes: &[u8],
    output_dir: &Path,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let cursor = Cursor::new(bytes);
    let mut archive = zip::ZipArchive::new(cursor)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let name = file.name().to_string();

        if name == "ffmpeg" || name.ends_with("/ffmpeg") {
            let dest = output_dir.join("ffmpeg");
            let mut out = std::fs::File::create(&dest)?;

            std::io::copy(&mut file, &mut out)?;
            return Ok(dest);
        }
    }

    Err("ffmpeg binary not found in archive".into())
}

fn extract_ffmpeg_tar_xz(
    bytes: &[u8],
    output_dir: &Path,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let cursor = Cursor::new(bytes);
    let decompressor = xz2::read::XzDecoder::new(cursor);
    let mut archive = tar::Archive::new(decompressor);

    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?.to_path_buf();

        if path.file_name().map(|f| f == "ffmpeg").unwrap_or(false)
            && path.parent().map(|p| p.ends_with("bin")).unwrap_or(false)
        {
            let dest = output_dir.join("ffmpeg");
            let mut out = std::fs::File::create(&dest)?;

            std::io::copy(&mut entry, &mut out)?;

            return Ok(dest);
        }
    }

    Err("ffmpeg binary not found in archive".into())
}

pub fn delete_ffmpeg(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::remove_file(path)?;

    Ok(())
}
