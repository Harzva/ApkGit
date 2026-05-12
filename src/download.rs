use std::path::Path;

pub fn download_file(url: &str, dest_path: &Path) -> Result<(), String> {
    if url.is_empty() {
        return Err("Download URL is empty.".to_string());
    }

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(300))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let mut response = client
        .get(url)
        .send()
        .map_err(|e| format!("Download request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Download failed: HTTP {}", response.status()));
    }

    if let Some(parent) = dest_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    let mut file =
        std::fs::File::create(dest_path).map_err(|e| format!("Failed to create file: {}", e))?;

    std::io::copy(&mut response, &mut file).map_err(|e| format!("Failed to write file: {}", e))?;

    Ok(())
}

pub fn get_download_dir() -> std::path::PathBuf {
    let home = dirs::download_dir()
        .or_else(dirs::home_dir)
        .unwrap_or_else(|| std::path::PathBuf::from("."));
    home.join("ReleaseMarket_Downloads")
}

pub fn calc_sha256(path: &Path) -> Result<String, String> {
    use sha2::{Digest, Sha256};
    use std::io::Read;

    let mut file = std::fs::File::open(path).map_err(|e| format!("Failed to open file: {}", e))?;

    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];

    loop {
        let n = file
            .read(&mut buffer)
            .map_err(|e| format!("Failed to read file: {}", e))?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    Ok(hex::encode(hasher.finalize()))
}

pub fn filename_from_url(url: &str) -> String {
    url.split('/')
        .next_back()
        .unwrap_or("download.apk")
        .split('?')
        .next()
        .unwrap_or("download.apk")
        .to_string()
}

pub fn open_download_dir() {
    let dir = get_download_dir();
    #[cfg(target_os = "windows")]
    std::process::Command::new("explorer")
        .arg(&dir)
        .spawn()
        .ok();
    #[cfg(target_os = "macos")]
    std::process::Command::new("open").arg(&dir).spawn().ok();
    #[cfg(target_os = "linux")]
    std::process::Command::new("xdg-open")
        .arg(&dir)
        .spawn()
        .ok();
}

pub fn install_apk_via_adb(apk_path: &Path) -> Result<String, String> {
    let output = std::process::Command::new("adb")
        .args(["install", "-r", apk_path.to_str().unwrap_or("")])
        .output()
        .map_err(|e| {
            format!(
                "Failed to run adb: {}\n\nCheck that USB debugging is enabled, the device is connected, and adb is on PATH.",
                e
            )
        })?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if output.status.success() || stdout.contains("Success") {
        Ok("Install succeeded.".to_string())
    } else {
        Err(format!(
            "Install failed:\n{}",
            if stderr.is_empty() { &stdout } else { &stderr }
        ))
    }
}

pub fn install_apk_native(apk_path: &Path) -> Result<String, String> {
    #[cfg(target_os = "android")]
    {
        use std::process::Command;
        let output = Command::new("pm")
            .args(["install", "-r", apk_path.to_str().unwrap_or("")])
            .output()
            .map_err(|e| format!("Install command failed: {}", e))?;

        if output.status.success() {
            Ok("Install succeeded.".to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }

    #[cfg(not(target_os = "android"))]
    {
        if let Ok(result) = install_apk_via_adb(apk_path) {
            Ok(result)
        } else {
            Err("This is not an Android device. Use adb install, copy the APK to a phone, or open the download directory and install manually.".to_string())
        }
    }
}
