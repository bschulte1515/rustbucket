use std::{
    fs::File,
    io::{self, Write, BufRead, BufReader},
    path::Path
};

use std::fs;
use rand::Rng;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

pub fn trim_trailing_newline(input: &mut String) {
    if let Some('\n')=input.chars().next_back() {
        input.pop();
    }
    if let Some('\r')=input.chars().next_back() {
        input.pop();
    }
}

fn get_confirmation(input: &str, confirmation: &mut String) -> io::Result<()> {
    print!("You entered {input}, is this correct? (Y/N): ");
    io::stdout().flush()?;
    io::stdin().read_line(confirmation)?;
    trim_trailing_newline(confirmation);
    while confirmation.to_uppercase() != "Y" && confirmation.to_uppercase() != "N" {
        confirmation.clear();
        print!("Incorrect input, try again (Y/N): "); 
        io::stdout().flush()?;
        io::stdin().read_line(confirmation)?;
        trim_trailing_newline(confirmation);
    }
    Ok(())
}

pub fn get_filename(recursive: bool) -> io::Result<String> {
    let mut input = String::new();
    let mut confirmation = String::new();
    while confirmation.to_uppercase() != "Y" {
        input.clear();
        confirmation.clear();

        if recursive {
            print!("Enter the directory you would like to obfuscate: ");
        } else {
            print!("Enter the file you would like to obfuscate: ");
        }
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;
        trim_trailing_newline(&mut input);
        
        get_confirmation(&input, &mut confirmation)?;
    }

    Ok(input)
}

fn run_obfuscation(filename: &Path, lines: &mut Vec<String>) -> io::Result<()> {
    let mut file = File::create(filename)?; 
    let mut rng = rand::rng();
    
    let mut i = 0;
    while i < lines.len() {
        let new_string: Vec<char> = lines[i].chars().filter(|c| !c.is_whitespace()).collect();
        if new_string.is_empty() {
            lines.remove(i);
            continue;
        }
        if new_string.len() >= 2 && new_string[0] == '/' && new_string[1] == '/' {
            lines.remove(i);
            continue;
        }

        if let Some(comment_pos) = lines[i].find("//") {
            lines[i].truncate(comment_pos);
        }
        
        write!(file, "{}", lines[i])?;        
        let rand_num: i16 = rng.random_range(0..=15);
        for _ in 0..=rand_num {
            write!(file, " ")?;
        }
        
        i += 1;
    }
    
    Ok(())
}

fn visit_dir(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        // Iterate over the entries in this directory
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                visit_dir(&path)?;
            } else {
                let mut lines = lines_from_file(&path)?;
                run_obfuscation(&path, &mut lines)?;
            }
        }
    }

    Ok(())
}

pub fn run() -> io::Result<()> {
    let mut confirmation = String::new();
    print!("Would you like to obfuscate recursively? (Y/N): ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut confirmation)?;
    trim_trailing_newline(&mut confirmation);
    while confirmation.to_uppercase() != "Y" && confirmation.to_uppercase() != "N" {
        confirmation.clear();
        print!("Incorrect input, try again (Y/N): "); 
        io::stdout().flush()?;
        io::stdin().read_line(&mut confirmation)?;
        trim_trailing_newline(&mut confirmation);
    }

    if confirmation.to_uppercase() == "Y" {
        let path_str = get_filename(true)?;
        let dir_path = Path::new(&path_str);
        visit_dir(&dir_path)?;
    } else {
        let file_path = get_filename(false)?;
        let mut lines = lines_from_file(&file_path)?;
        run_obfuscation(file_path.as_ref(), &mut lines)?;
    }

    Ok(())
}
