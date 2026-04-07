# File Organizer (Rust)

A production-ready file organizer written in Rust that automatically sorts files by type into organized folders.
      
## 🚀 Features

- **Sort files by type**: Automatically categorizes files into Images, Videos, Documents, Audio, Archives, Code, Executables, and Others
- **CLI interface**: Simple command-line interface with intuitive arguments
- **Safe error handling**: Comprehensive error handling with descriptive messages
- **Clean structure**: Well-organized code with separation of concerns
- **Extensible**: Easy to add new file categories and customization
- **Dry run mode**: Preview changes before actually moving files
   
## 📁 Supported File Categories

- **Images**: jpg, jpeg, png, gif, bmp, tiff, webp, svg, ico
- **Videos**: mp4, avi, mkv, mov, wmv, flv, webm, m4v
- **Audio**: mp3, wav, flac, aac, ogg, wma, m4a
- **Documents**: pdf, doc, docx, txt, rtf, odt, xls, xlsx, ppt, pptx
- **Archives**: zip, rar, 7z, tar, gz, bz2
- **Code**: rs, py, js, ts, java, cpp, c, h, html, css, php, rb, go, swift, kt
- **Executables**: exe, msi, dmg, deb, rpm, app
- **Others**: Any files with unrecognized extensions

## 🛠️ Installation

1. Make sure you have Rust installed: https://rustup.rs/
2. Clone or download this repository
3. Navigate to the `file_organizer` directory
4. Build the project: `cargo build --release`

## 📖 Usage

```bash
# Basic usage - organize files in a directory
cargo run -- --source /path/to/messy/folder

# Specify a custom target directory
cargo run -- --source /path/to/messy/folder --target /path/to/organized

# Dry run - see what would be moved without actually moving
cargo run -- --source /path/to/messy/folder --dry-run

# Get help
cargo run -- --help
```

## 📁 Project Structure

```
file_organizer/
├── Cargo.toml
└── src/
    ├── main.rs      # Entry point and CLI argument parsing
    ├── organizer.rs # Core file organization logic
    └── utils.rs     # Helper functions for file categorization
```

## 🔧 Dependencies

- `clap`: Command-line argument parsing
- `walkdir`: Directory traversal
- `anyhow`: Error handling

## 🤝 Contributing

Feel free to submit issues and enhancement requests!

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.
