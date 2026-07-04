use directories::ProjectDirs;

pub fn config_dir() -> PathBuf {
    if std::env::var("VOICEPILOT_DEV").is_ok() {
        return PathBuf::from("./.dev/config");
    }

    directories::ProjectDirs::from("com", "voicepilot", "voicepilot")
        .unwrap()
        .config_dir()
        .to_path_buf()
}

pub fn project_dirs() -> Option<ProjectDirs> {
    ProjectDirs::from("dev", "voicepilot", "voicepilot")
}
