use std::{
    fs::File,
    io::{self, Write, BufRead, BufReader},
    path::Path
};

fn lines_from_file(filename: &Path) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn run_obfuscation(filename: &Path, lines: &Vec<String>) -> io::Result<()> {
    let mut file = File::create(filename)?; 
    for line in lines {
        write!(file, "{line}")?;        
    }
    Ok(())
}

pub fn run(filename: impl AsRef<Path>) -> io::Result<()> {
    let file_path = filename.as_ref();
    let lines = lines_from_file(&file_path)?;
    run_obfuscation(&file_path, &lines)?;
    Ok(())
}
