use directories::UserDirs;
use std::fs;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    if let Some(user_dirs) = UserDirs::new() {
        let dir_paths: Vec<&Path> = [
            user_dirs.document_dir(),
            user_dirs.download_dir(),
            user_dirs.desktop_dir(),
        ]
        .into_iter()
        .filter_map(|dir| if dir.is_some() { dir } else { None })
        .collect();

        for dir_path in dir_paths {
            let entries = fs::read_dir(dir_path)?.filter_map(|entry| {
                entry.ok().and_then(|e| {
                    let file_name = e.file_name().to_string_lossy().into_owned().to_lowercase();
                    if file_name.starts_with("pix") {
                        Some(e.path())
                    } else {
                        None
                    }
                })
            });

            for entry in entries {
                if let Err(err) = fs::remove_file(&entry) {
                    eprintln!("Error removing file {}: {}", entry.display(), err);
                }
            }

            println!("Cleaned up `{}` directory!", dir_path.display());
        }
    }

    Ok(())
}
