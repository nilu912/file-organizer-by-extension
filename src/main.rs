// use std::io;
// use std::path::Path;
// fn main() {
//     let mut fName = String::new();
//     println!("Enter FileName: ");
//     io::stdin().read_line(&mut fName).expect("Failed to read line!");
//     let path = Path::new(&fName);
//     println!("{} {}",fName,path.file_stem());
// }
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn main() {
    let mut source_path = String::new();
    let mut destination_path = String::new();

    println!("Enter source folder path: ");
    io::stdin()
        .read_line(&mut source_path)
        .expect("Failed to read source path!");
    let source_path = source_path.trim();

    // Get destination directory path
    println!("Enter destination folder path: ");
    io::stdin()
        .read_line(&mut destination_path)
        .expect("Failed to read destination path!");
    let destination_path = destination_path.trim();

    // Convert input strings to Pathse
    let source_dir = Path::new(&source_path);
    let destination_dir = Path::new(&destination_path);

    if !source_dir.exists(){
        println!("The source folder doesn't exist. Please check the path.");
        return;
    }

    if !destination_dir.exists() {
        fs::create_dir_all(destination_dir).expect("Couldn't create the destination folder!");
    }

    // Organize files by extension
    // println!("{}", source_dir);
    organize_files_by_extension(source_dir, destination_dir);
    println!("our files have been organized successfully!");
}

fn organize_files_by_extension(source_dir: &Path, destination_dir: &Path) {
    for i in fs::read_dir(source_dir).expect("Failed to read source directory!") {
        if let Ok(i) = i {
            let path = i.path();
            
            // Skip if not a file
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    // Create a folder name based on the extension
                    let folder_name = format!("{}_files", extension.to_string_lossy());
                    let folder_path = destination_dir.join(folder_name);

                    // Create the folder if it doesn't exist
                    if !folder_path.exists() {
                        fs::create_dir_all(&folder_path).expect("Failed to create folder for extension!");
                    }

                    // Move the file to the folder
                    let file_name = path.file_name().expect("Failed to get file name!");
                    let destination_path = folder_path.join(file_name);

                    fs::rename(&path, &destination_path).expect("Failed to move file!");
                }
            }
        }
    }
}
