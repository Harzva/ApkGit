use std::path::Path;

/// 下载管理器 - 使用 reqwest 下载文件
pub fn download_file(url: &str, dest_path: &Path) -> Result<(), String> {
    if url.is_empty() {
        return Err("下载链接为空".to_string());
    }

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(300))
        .build()
        .map_err(|e| format!("客户端创建失败: {}", e))?;

    let mut response = client
        .get(url)
        .send()
        .map_err(|e| format!("下载请求失败: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("下载失败: HTTP {}", response.status()));
    }

    // 确保目录存在
    if let Some(parent) = dest_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("创建目录失败: {}", e))?;
    }

    let mut file = std::fs::File::create(dest_path).map_err(|e| format!("创建文件失败: {}", e))?;

    std::io::copy(&mut response, &mut file).map_err(|e| format!("写入文件失败: {}", e))?;

    Ok(())
}

/// 获取下载目录
pub fn get_download_dir() -> std::path::PathBuf {
    let home = dirs::download_dir()
        .or_else(|| dirs::home_dir())
        .unwrap_or_else(|| std::path::PathBuf::from("."));
    home.join("ApkGit_Downloads")
}

/// 计算 SHA256（用于校验）
pub fn calc_sha256(path: &Path) -> Result<String, String> {
    use sha2::{Digest, Sha256};
    use std::io::Read;

    let mut file = std::fs::File::open(path).map_err(|e| format!("打开文件失败: {}", e))?;

    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];

    loop {
        let n = file
            .read(&mut buffer)
            .map_err(|e| format!("读取失败: {}", e))?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    Ok(hex::encode(hasher.finalize()))
}

/// 获取文件名从 URL
pub fn filename_from_url(url: &str) -> String {
    url.split('/')
        .last()
        .unwrap_or("download.apk")
        .split('?')
        .next()
        .unwrap_or("download.apk")
        .to_string()
}

/// 打开下载目录
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

/// 使用 ADB 安装 APK（需要设备连接和 ADB）
pub fn install_apk_via_adb(apk_path: &Path) -> Result<String, String> {
    let output = std::process::Command::new("adb")
        .args(["install", "-r", apk_path.to_str().unwrap_or("")])
        .output()
        .map_err(|e| {
            format!(
                "ADB 命令执行失败: {}\n\n请确保:\n1. 手机已开启 USB 调试\n2. 已通过 adb connect 连接设备\n3. ADB 已添加到环境变量",
                e
            )
        })?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if output.status.success() || stdout.contains("Success") {
        Ok("安装成功！".to_string())
    } else {
        Err(format!(
            "安装失败:\n{}",
            if stderr.is_empty() { &stdout } else { &stderr }
        ))
    }
}

/// 安装 APK 到本机（如果是 Android 设备）
pub fn install_apk_native(apk_path: &Path) -> Result<String, String> {
    #[cfg(target_os = "android")]
    {
        use std::process::Command;
        let output = Command::new("pm")
            .args(["install", "-r", apk_path.to_str().unwrap_or("")])
            .output()
            .map_err(|e| format!("安装命令失败: {}", e))?;

        if output.status.success() {
            Ok("安装成功！".to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }

    #[cfg(not(target_os = "android"))]
    {
        // 桌面端：尝试 ADB，否则提示用户手动安装
        if let Ok(result) = install_apk_via_adb(apk_path) {
            Ok(result)
        } else {
            Err("当前不是 Android 设备\n\n在桌面端，请通过以下方式安装:\n1. 使用 adb install 命令\n2. 将 APK 推送到手机后手动安装\n3. 点击下方「打开下载目录」手动安装".to_string())
        }
    }
}
