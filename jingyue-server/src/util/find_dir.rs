use std::{env, path::PathBuf};

use tracing::error;

pub fn find_available_db_directory() -> Option<PathBuf> {
    let os = env::consts::OS;

    match os {
        "windows" => {
            // 尝试获取 AppData\Roaming 目录
            env::var("APPDATA").ok().map(PathBuf::from)
        }
        "linux" => {
            // 遵循 XDG Base Directory 规范
            // 优先使用 XDG_DATA_HOME
            if let Ok(data_home) = env::var("XDG_DATA_HOME") {
                if !data_home.is_empty() {
                    return Some(PathBuf::from(data_home));
                }
            }
            // 如果 XDG_DATA_HOME 未设置或为空，则使用默认的 ~/.local/share
            if let Ok(home) = env::var("HOME") {
                if !home.is_empty() {
                    let mut path = PathBuf::from(home);
                    path.push(".local");
                    path.push("share");
                    return Some(path);
                }
            }
            // 如果 HOME 也未设置，则无法确定目录
            None
        }
        // 对于 macOS 或其他系统，可以在这里添加逻辑
        _ => {
            error!(
                "Warning: unsupported operating system '{}', unable to automatically determine database directory. ",
                os
            );
            None
        }
    }
}
