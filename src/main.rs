// #![windows_subsystem = "windows"] // Uncomment this line to make the app invisible for background running

use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;

const TARGET_DIR: &str = r"D:\Downloads";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user_profile = env::var("USERPROFILE").expect("Could not find Windows User Profile");
    let source_dir = Path::new(&user_profile).join("Downloads");

    if !source_dir.exists() {
        println!("❌ Cannot find your Downloads folder at {:?}", source_dir);
        return Ok(());
    }

    fs::create_dir_all(TARGET_DIR)?;

    println!("🚀 Starting Deep-Tree Download Manager (Full Drive Reorganizer)...");
    println!("📂 Watching Source: {:?}", source_dir);
    println!("📂 Target: {}", TARGET_DIR);

    let active_files = Arc::new(Mutex::new(HashSet::new()));

    // 1. Recursively sweep both C: and D: drives for loose or unorganized files
    println!("🧹 Recursively scanning and reorganizing existing files...");
    organize_directory_recursively(&source_dir, active_files.clone()).await;
    organize_directory_recursively(Path::new(TARGET_DIR), active_files.clone()).await;

    // 2. Setup Real-time File Watcher
    let (tx, mut rx) = mpsc::channel(100);
    let mut watcher = RecommendedWatcher::new(
        move |res: notify::Result<Event>| {
            if let Ok(event) = res {
                let _ = tx.blocking_send(event);
            }
        },
        Config::default(),
    )?;

    watcher.watch(&source_dir, RecursiveMode::NonRecursive)?;
    println!("👀 Watching for new downloads (0% CPU Idle Mode)...");

    // 3. Main Event Loop
    while let Some(event) = rx.recv().await {
        if event.kind.is_create() || event.kind.is_modify() {
            for path in event.paths {
                if is_valid_file(&path) {
                    let mut locked_set = active_files.lock().unwrap();
                    if locked_set.insert(path.clone()) {
                        let active_files_clone = active_files.clone();
                        tokio::spawn(async move {
                            handle_new_file(path.clone()).await;
                            active_files_clone.lock().unwrap().remove(&path);
                        });
                    }
                }
            }
        }
    }

    Ok(())
}

/// Recursively burrows through all subdirectories to find and organize every file
async fn organize_directory_recursively(dir_to_sweep: &Path, active_files: Arc<Mutex<HashSet<PathBuf>>>) {
    let mut dirs_to_visit = vec![dir_to_sweep.to_path_buf()];

    while let Some(current_dir) = dirs_to_visit.pop() {
        if let Ok(entries) = fs::read_dir(&current_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                
                if path.is_dir() {
                    dirs_to_visit.push(path);
                } else if path.is_file() {
                    if is_valid_file(&path) {
                        let mut locked_set = active_files.lock().unwrap();
                        if locked_set.insert(path.clone()) {
                            let active_files_clone = active_files.clone();
                            tokio::spawn(async move {
                                handle_new_file(path.clone()).await;
                                active_files_clone.lock().unwrap().remove(&path);
                            });
                        }
                    }
                }
            }
        }
    }
}

/// Checks if the file is a finished download based on its extension
fn is_valid_file(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }
    if let Some(ext) = path.extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();
        // Ignore temporary browser download files AND hidden system config files (like desktop.ini)
        if ext_str == "crdownload" || ext_str == "part" || ext_str == "tmp" || ext_str == "download" || ext_str == "ini" {
            return false;
        }
    }
    true
}

/// Attempts to move a file to the correct deep-tree path
async fn handle_new_file(source_path: PathBuf) {
    let file_name = match source_path.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => return,
    };

    let extension = source_path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
    let category = get_local_category(&extension);

    // Create the dynamic extension folder string
    let ext_folder = if extension.is_empty() {
        "UNKNOWN".to_string()
    } else {
        extension.to_uppercase()
    };

    let mut retries = 0;
    let max_retries = 60;

    while retries < max_retries {
        let metadata = match fs::metadata(&source_path) {
            Ok(m) => m,
            Err(_) => return, 
        };

        let size_folder = get_size_subcategory(metadata.len());
        
        let target_dir = Path::new(TARGET_DIR)
            .join(category)
            .join(&ext_folder) 
            .join(size_folder);
        
        let target_path = target_dir.join(&file_name);

        // --- CRITICAL SAFETY CHECK ---
        // If the file's current path is exactly the same as where it's supposed to go, do nothing!
        if source_path.to_string_lossy().to_lowercase() == target_path.to_string_lossy().to_lowercase() {
            return;
        }

        if let Err(e) = fs::create_dir_all(&target_dir) {
            println!("❌ Failed to create directory {:?}: {}", target_dir, e);
            return;
        }

        // Attempt the Cross-Drive / Deep-Tree Copy
        match fs::copy(&source_path, &target_path) {
            Ok(_) => {
                let _ = fs::remove_file(&source_path);
                println!("✅ Successfully moved: {} -> {:?}", file_name, target_dir);
                return; 
            }
            Err(_) => {
                // File locked by another process (like a browser downloading it)
                retries += 1;
                sleep(Duration::from_secs(5)).await;
            }
        }
    }
    
    println!("⚠️ Gave up on moving {} after 5 minutes of retries.", file_name);
}

/// Massive local categorization dictionary
fn get_local_category(extension: &str) -> &'static str {
    match extension {
        "pdf" | "docx" | "doc" | "txt" | "rtf" | "odt" | "epub" | "mobi" | "md" => "Documents",
        "pptx" | "ppt" | "ppsx" | "key" | "odp" => "Presentations",
        "xlsx" | "xls" | "csv" | "ods" | "tsv" => "Spreadsheets",
        "jpg" | "jpeg" | "png" | "gif" | "svg" | "webp" | "bmp" | "tiff" | "ico" | "heic" | "jfif" | "raw" => "Images",
        "psd" | "ai" | "xd" | "fig" | "sketch" | "aep" | "prproj" => "Design_Project_Files",
        "mp4" | "mkv" | "avi" | "mov" | "webm" | "flv" | "wmv" | "m4v" => "Videos",
        "mp3" | "wav" | "flac" | "aac" | "ogg" | "m4a" | "wma" => "Audio",
        "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" | "xz" | "cab" | "iso" | "img" | "vdi" | "vmdk" | "ova" => "Archives",
        "exe" | "msi" | "apk" | "dmg" | "deb" | "rpm" | "appimage" | "bat" | "sh" | "ps1" => "Software",
        "h5" | "keras" | "pth" | "pt" | "onnx" | "tflite" | "pb" | "safetensors" | "gguf" | "bin" | "pkl" | "joblib" => "AI_Models",
        "parquet" | "feather" | "avro" | "sqlite" | "db" => "Data_Stores",
        "rs" | "py" | "js" | "ts" | "html" | "css" | "c" | "cpp" | "h" | "hpp" | "java" | "class" | "jar" | "go" | "rb" | "php" | "sql" | "cs" | "swift" | "kt" => "Code",
        "json" | "xml" | "yaml" | "yml" | "toml" | "env" => "Config_Files",
        "ipynb" => "Jupyter_Notebooks",
        "blend" | "fbx" | "obj" | "stl" | "dwg" | "dxf" | "gltf" | "glb" | "step" | "iges" => "3D_Models",
        "ttf" | "otf" | "woff" | "woff2" => "Fonts",
        _ => "Others", 
    }
}

/// Appends size category to the folder name
fn get_size_subcategory(file_size: u64) -> &'static str {
    const MB: u64 = 1024 * 1024;
    if file_size < 10 * MB {
        "Small"
    } else if file_size < 500 * MB {
        "Medium"
    } else {
        "Large"
    }
}