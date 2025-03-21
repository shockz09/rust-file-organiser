use std::io;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
fn main (){
   println!("Please enter the folder destination where the files are located");
   let mut directory = String::new();
   match io::stdin().read_line(&mut directory){
    Ok(_)=>{
        let directory = directory.trim();
        println!("{}",directory);

        let path = Path::new(directory);

        match fs::read_dir(path){
            Ok(entries)=>{
                println!("Contents of directory");
                for entry in entries {
                    match entry {
                        Ok(entry)=>{
                            let path = entry.path();
                            let file_name = entry.file_name();
                            let file_type = entry.file_type().unwrap();
                            if let Some(file_extension) = entry.path().extension() {
                            
                                // Convert &OsStr to PathBuf
                                let file_extension_path: PathBuf = PathBuf::from(file_extension);
                                // Create the new directory path
                                let new_directory_path = path.join(&file_extension_path);
                                if let Some(parent) = path.parent() {
                                    let new_directory_path = parent.join(file_extension);
                                    fs::create_dir_all(new_directory_path.as_path()).unwrap();
                                    let mut destination_file = PathBuf::from(new_directory_path);
                                    destination_file.push(entry.file_name());
                                    fn move_file(path: &Path, destination_file: &Path) -> std::io::Result<()> {
                                        fs::rename(path, destination_file)
                                    }
                                    let heh = path.clone();
                                    move_file(&heh, &destination_file).unwrap();
                                    // Print or use the new directory path
                                    // println!("  - New directory path: {:?}", new_directory_path);
                                }
                                print!(
                                    "- {:?} (TYPE: {:?})) ",
                                    file_name, file_type
                                );
                                if file_type.is_file() {
                                    println!("    - This is a file. {}", file_extension.to_str().unwrap());
                                } else if file_type.is_dir() {
                                    println!("    - This is a directory.");
                                }
                            } else {
                                print!(
                                    "- {:?} (TYPE: {:?})) ",
                                    file_name, file_type
                                );
                                if file_type.is_file() {
                                    println!("    - This is a file with no extension.");
                                } else if file_type.is_dir() {
                                    println!("    - This is a directory.");
                                }
                            }
                        }
                        Err(_err)=>{
                            println!(" Error reading entry ");
                        }
                        Err(_err)=>{
                            println!(" Error reading entry ");
                        
                        }
                        
                    }

                }
            }
            Err(_err)=>{
                println!("Error reading directory");
            }
        }
    }
    
    Err(_error)=>{
        println!("Error reading input");
    }
   }


   
}