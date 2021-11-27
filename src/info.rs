use std::path::PathBuf;

pub fn get_info(path: PathBuf) -> String {
    let file_ext = path.extension();
    let file_ext = match file_ext {
        Some(ext) => ext.to_str().unwrap(),
        None => "none",
    };

    match file_ext {
        // Programming language
        "rs" => "Rust source code file.",
        "js" => "JavaScript source code file.",
        "ts" => "TypeScript source code file.",
        "go" => "Go source code file.",
        "sh" => "Bash shell script file.",
        "hs" => "Haskell source code file.",

        // Media file
        "png" => "Portable Network Graphics image file.",
        "jpeg" => "Lossy compressed digital image file.",

        // Other
        "toml" => "TOML file.",
        "yml" | "yaml" => "YAML file.",
        "json" | "jsonc" => "JSON file.",
        "lock" => "Locked file.",
        "md" => "MarkDown file.",

        "none" => "Executable file.",

        _ => "Unknown file.",
    }
    .into()
}
