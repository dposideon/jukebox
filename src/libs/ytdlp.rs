use futures_util::StreamExt;
use std::path::PathBuf;
use std::str::FromStr;
use tokio::fs;
use tokio::io::AsyncWriteExt;

pub async fn download_ytdlp(
    link: &str,
    lib_dir: &str,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let output_dir = PathBuf::from_str(lib_dir)?;
    let dest = output_dir.join("yt-dlp");

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

    Ok(dest)
}

pub fn delete_ytdlp(lib_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let target_dir = PathBuf::from_str(lib_dir)?;
    let target_file = target_dir.join("yt-dlp");
    std::fs::remove_file(target_file)?;

    Ok(())
}
