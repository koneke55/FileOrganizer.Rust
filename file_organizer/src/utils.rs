use std::collections::HashMap;
use std::fs;
use std::path::Path;
use anyhow::{Result, Context};

pub fn get_file_category(extension: &str) -> String {
    let categories: HashMap<&str, &str> = [
        // Images
        ("jpg", "Images"),
        ("jpeg", "Images"),
        ("png", "Images"),
        ("gif", "Images"),
        ("bmp", "Images"),
        ("tiff", "Images"),
        ("webp", "Images"),
        ("svg", "Images"),
        ("ico", "Images"),

        // Videos
        ("mp4", "Videos"),
        ("avi", "Videos"),
        ("mkv", "Videos"),
        ("mov", "Videos"),
        ("wmv", "Videos"),
        ("flv", "Videos"),
        ("webm", "Videos"),
        ("m4v", "Videos"),

        // Audio
        ("mp3", "Audio"),
        ("wav", "Audio"),
        ("flac", "Audio"),
        ("aac", "Audio"),
        ("ogg", "Audio"),
        ("wma", "Audio"),
        ("m4a", "Audio"),

        // Documents
        ("pdf", "Documents"),
        ("doc", "Documents"),
        ("docx", "Documents"),
        ("txt", "Documents"),
        ("rtf", "Documents"),
        ("odt", "Documents"),
        ("xls", "Documents"),
        ("xlsx", "Documents"),
        ("ppt", "Documents"),
        ("pptx", "Documents"),

        // Archives
        ("zip", "Archives"),
        ("rar", "Archives"),
        ("7z", "Archives"),
        ("tar", "Archives"),
        ("gz", "Archives"),
        ("bz2", "Archives"),

        // Code
        ("rs", "Code"),
        ("py", "Code"),
        ("js", "Code"),
        ("ts", "Code"),
        ("java", "Code"),
        ("cpp", "Code"),
        ("c", "Code"),
        ("h", "Code"),
        ("html", "Code"),
        ("css", "Code"),
        ("php", "Code"),
        ("rb", "Code"),
        ("go", "Code"),
        ("swift", "Code"),
        ("kt", "Code"),

        // Executables
        ("exe", "Executables"),
        ("msi", "Executables"),
        ("dmg", "Executables"),
        ("deb", "Executables"),
        ("rpm", "Executables"),
        ("app", "Executables"),
    ].iter().cloned().collect();

    categories.get(extension).unwrap_or(&"Others").to_string()
}

pub fn create_category_dir(base_dir: &Path, category: &str) -> Result<std::path::PathBuf> {
    let category_dir = base_dir.join(category);
    fs::create_dir_all(&category_dir)
        .with_context(|| format!("Failed to create category directory: {:?}", category_dir))?;
    Ok(category_dir)
}