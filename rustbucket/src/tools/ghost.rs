use std::{
    io, 
    fs,
    // path::{Path},    
};


pub fn run() -> io::Result<()> {
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let path = entry.path();

        let metadata = fs::metadata(&path)?;
        
        if metadata.is_file() {
            println!("Found file: {:?}", path); 
        } else if metadata.is_dir() {
            println!("Found dir: {:?}", path);
        }
        // let entry = entry.file_name().to_os_string();
        // println!("{:?}", entry);
        // let old_path = entry.path().to_string_lossy().into_owned();
        // let new_path = Path::new(".").join(&old_path).to_string_lossy().to_string();
        
        // println!("{} and now {}", old_path, new_path); 
    }
    Ok(())
}
