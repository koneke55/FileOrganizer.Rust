use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;
use anyhow::{Result, Context};

use crate::utils::{get_file_category, create_category_dir};

pub struct FileOrganizer {
    source_dir: PathBuf,
    target_dir: PathBuf,
    dry_run: bool,
}

impl FileOrganizer {
    pub fn new(source_dir: PathBuf, target_dir: PathBuf) -> Self {
        Self {
            source_dir,
            target_dir,
            dry_run: false,
        }
    }

    pub fn with_dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = dry_run;
        self
    }

    pub fn organize(&self) -> Result<()> {
        // Create target directory if it doesn't exist (only if not dry run)
        if !self.dry_run {
            fs::create_dir_all(&self.target_dir)
                .with_context(|| format!("Failed to create target directory: {:?}", self.target_dir))?;
        }

        let mut moved_files = 0;
        let mut category_counts: HashMap<String, usize> = HashMap::new();

        // Walk through all files in source directory
        for entry in WalkDir::new(&self.source_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();

            // Skip directories
            if path.is_dir() {
                continue;
            }

            // Skip files in the target directory (to avoid re-processing moved files)
            if path.starts_with(&self.target_dir) {
                continue;
            }

            // Get file extension and determine category
            if let Some(extension) = path.extension() {
                let ext_str = extension.to_string_lossy().to_lowercase();
                let category = get_file_category(&ext_str);

                // Create category directory (only if not dry run)
                if !self.dry_run {
                    create_category_dir(&self.target_dir, &category)?;
                }

                // Create new file path
                let file_name = path.file_name().unwrap();
                let new_path = self.target_dir.join(&category).join(file_name);

                // Move file (only if not dry run)
                if self.dry_run {
                    println!("Would move: {:?} -> {:?}", path, new_path);
                } else {
                    fs::rename(path, &new_path)
                        .with_context(|| format!("Failed to move file {:?} to {:?}", path, new_path))?;
                    println!("Moved: {:?} -> {:?}", path, new_path);
                }

                moved_files += 1;
                *category_counts.entry(category.clone()).or_insert(0) += 1;
            }
        }

        // Print summary
        println!("\nOrganization complete!");
        if self.dry_run {
            println!("This was a dry run - no files were actually moved.");
        }
        println!("Total files {}: {}", if self.dry_run { "that would be moved" } else { "moved" }, moved_files);
        for (category, count) in category_counts {
            println!("  {}: {} files", category, count);
        }

        Ok(())
    }
}