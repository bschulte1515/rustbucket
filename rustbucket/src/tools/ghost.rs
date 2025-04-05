use std::{
    io, 
    fs,
    path::PathBuf,    
};


pub fn run() -> io::Result<()> {
    let dir = ".";
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        let metadata = fs::metadata(&path)?;
        
        if metadata.is_file() {
            let mut new_path = String::from(".");
            new_path.push_str(path.file_name().unwrap().to_str().unwrap());

            let mut new_path_buf = PathBuf::new();
            new_path_buf.push(dir);
            new_path_buf.push(&new_path);
                
            fs::rename(path, new_path_buf)?; 
        }

        /* Use this code if we want to recurisve ghost */
        // else if metadata.is_dir() {
        //     ...
        // }
    }
    Ok(())
}
