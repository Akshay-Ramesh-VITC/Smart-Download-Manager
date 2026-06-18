# Smart Download Manager

🚀 An intelligent Windows download organizer written in Rust that automatically monitors your Downloads folder, categorizes files, and builds a deep directory tree based on file type and size.

## Features

* 📂 Real-time monitoring of the Windows Downloads folder
* ⚡ Automatically detects newly downloaded files
* 🧹 Recursive organization of existing files
* 📦 Categorizes files into 15+ predefined categories
* 📏 Further organizes files by size (Small, Medium, Large)
* 🔄 Handles locked/incomplete downloads with retry logic
* 🛡️ Prevents duplicate processing
* 💾 Supports cross-drive file transfers
* 💤 Near-zero CPU usage while idle
* 🔍 Automatically scans and reorganizes previously downloaded files on startup

---

## Folder Structure

All files are moved into a structured hierarchy:

```text
D:\Downloads
│
├── Documents
│   ├── PDF
│   │   ├── Small
│   │   ├── Medium
│   │   └── Large
│
├── Images
│   ├── JPG
│   ├── PNG
│   └── WEBP
│
├── Videos
│   ├── MP4
│   ├── MKV
│   └── AVI
│
├── Audio
├── Archives
├── Software
├── AI_Models
├── Data_Stores
├── Code
├── Config_Files
├── Jupyter_Notebooks
├── 3D_Models
├── Fonts
└── Others
```

---

## File Size Classification

| Category | Size Range     |
| -------- | -------------- |
| Small    | < 10 MB        |
| Medium   | 10 MB – 500 MB |
| Large    | > 500 MB       |

---

## Supported Categories

### Documents

```text
pdf, docx, doc, txt, rtf, odt, epub, mobi, md
```

### Presentations

```text
pptx, ppt, ppsx, key, odp
```

### Spreadsheets

```text
xlsx, xls, csv, ods, tsv
```

### Images

```text
jpg, jpeg, png, gif, svg, webp, bmp,
tiff, ico, heic, jfif, raw
```

### Design Project Files

```text
psd, ai, xd, fig, sketch, aep, prproj
```

### Videos

```text
mp4, mkv, avi, mov, webm, flv, wmv, m4v
```

### Audio

```text
mp3, wav, flac, aac, ogg, m4a, wma
```

### Archives

```text
zip, rar, 7z, tar, gz, bz2, xz,
cab, iso, img, vdi, vmdk, ova
```

### Software

```text
exe, msi, apk, dmg, deb, rpm,
appimage, bat, sh, ps1
```

### AI Models

```text
h5, keras, pth, pt, onnx,
tflite, pb, safetensors,
gguf, bin, pkl, joblib
```

### Data Stores

```text
parquet, feather, avro, sqlite, db
```

### Source Code

```text
rs, py, js, ts, html, css,
c, cpp, h, hpp, java, class,
jar, go, rb, php, sql, cs,
swift, kt
```

### Config Files

```text
json, xml, yaml, yml, toml, env
```

### Jupyter Notebooks

```text
ipynb
```

### 3D Models

```text
blend, fbx, obj, stl, dwg,
dxf, gltf, glb, step, iges
```

### Fonts

```text
ttf, otf, woff, woff2
```

---

## How It Works

### Startup Scan

When launched, the application:

1. Scans the user's Downloads folder.
2. Scans the target organization directory.
3. Recursively discovers all files.
4. Organizes misplaced files automatically.

### Real-Time Monitoring

After initialization:

1. Watches the Windows Downloads folder.
2. Detects file creation/modification events.
3. Waits for downloads to complete.
4. Moves files into the correct category.

### Smart Retry System

Many browsers lock files while downloading.

The application:

* Detects incomplete downloads.
* Retries every 5 seconds.
* Continues for up to 5 minutes.
* Moves the file immediately after it becomes available.

---

## Ignored Files

The following temporary or system files are skipped:

```text
*.crdownload
*.part
*.tmp
*.download
*.ini
```

---

## Installation

### Prerequisites

* Rust 1.75+
* Cargo
* Windows 10/11

### Clone Repository

```bash
git clone https://github.com/yourusername/smart-download-manager.git

cd smart-download-manager
```

### Build

```bash
cargo build --release
```

### Run

```bash
cargo run --release
```

---

## Configuration

Change the target directory by modifying:

```rust
const TARGET_DIR: &str = r"D:\Downloads";
```

Example:

```rust
const TARGET_DIR: &str = r"E:\SmartDownloads";
```

---

## Running in Background

To hide the console window and run silently:

```rust
#![windows_subsystem = "windows"]
```

Uncomment the line at the top of `main.rs` and rebuild.

---

## Example

Before:

```text
Downloads
├── research.pdf
├── movie.mkv
├── model.gguf
├── dataset.parquet
└── photo.jpg
```

After:

```text
D:\Downloads
├── Documents
│   └── PDF
│       └── Small
│           └── research.pdf
│
├── Videos
│   └── MKV
│       └── Large
│           └── movie.mkv
│
├── AI_Models
│   └── GGUF
│       └── Large
│           └── model.gguf
│
├── Data_Stores
│   └── PARQUET
│       └── Medium
│           └── dataset.parquet
│
└── Images
    └── JPG
        └── Small
            └── photo.jpg
```

---

## Tech Stack

* Rust
* Tokio
* Notify

---

## Future Enhancements

* GUI dashboard
* System tray support
* User-defined categories
* Duplicate file detection
* File hashing
* Automatic archive extraction
* AI-powered file classification
* Statistics and analytics dashboard
* Configuration file support
* Windows startup integration

---

## License

MIT License

Feel free to use, modify, and contribute.
