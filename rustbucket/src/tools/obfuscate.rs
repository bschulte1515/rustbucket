use std::{
    fs::File,
    io::{self, Write, BufRead, BufReader},
    path::Path
};

fn lines_from_file(filename: &Path) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn run_obfuscation(filename: &Path, lines: &mut Vec<String>) -> io::Result<()> {
    let mut file = File::create(filename)?; 
    for line in lines {
        let mut comment = true;
        let new_string : String = line.chars().filter(|c| !c.is_whitespace()).collect();
        for c in new_string.chars() {
            if c == '#' || c == '/' {
                writeln!(file, "{line}")?;
                break;
            } else {
                comment = false;
                break;
            }
        }
        if !comment {
            write!(file, "{line}")?;        
        }
    }

    Ok(())
}

pub fn run(filename: impl AsRef<Path>) -> io::Result<()> {
    let file_path = filename.as_ref();
    let mut lines = lines_from_file(&file_path)?;
    run_obfuscation(&file_path, &mut lines)?;

    Ok(())
}
