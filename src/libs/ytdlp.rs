use futures_util::StreamExt;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use tokio::fs;
use tokio::io::AsyncWriteExt;

pub async fn download_ytdlp(
    link: &str,
    dest: &Path,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    println!("Downloading yt-dlp from {}", link);

    let response = reqwest::get(link).await?;

    if !response.status().is_success() {
        return Err(format!("Download failed: HTTP {}", response.status()).into());
    }

    let mut file = fs::File::create(&dest).await?;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        file.write_all(&chunk?).await?;
    }

    file.flush().await?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&dest, std::fs::Permissions::from_mode(0o755)).await?;
    }

    println!("yt-dlp downloaded to {}", dest.display());

    Ok(dest.to_path_buf())
}

pub fn delete_ytdlp(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::remove_file(path)?;

    Ok(())
}
