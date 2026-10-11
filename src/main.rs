use std::env;
use std::fs;
use std::io;
use std::process;

fn process_file(file_path: &str) -> io::Result<String> {
    let content = fs::read_to_string(file_path)?;
    let dir = file_path.rfind('/').map_or("", |i| &file_path[..i]);
    let mut lines: Vec<String> = Vec::new();
    for line in content.lines() {
        // find link to .md file
        if line.starts_with('[') && line.ends_with(".md)") {
            if let Some(rest) = line.split('(').nth(1) {
                let mut link = rest.to_string();
                link.pop();
                let target = format!("{}/{}", dir, link.replace("%20", " "));
                lines.push(process_file(&target)?);
                continue;
            }
        }
        lines.push(line.to_string());
    }
    Ok(lines.join("\n"))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: md-merger <folder> <start> <output>");
        eprintln!("Merge markdown files into one markdown file");
        process::exit(2);
    }
    let (folder, start, output) = (&args[1], &args[2], &args[3]);

    let result = process_file(&format!("{}/{}", folder, start))
        .and_then(|content| fs::write(format!("{}/{}", folder, output), content));
    if let Err(e) = result {
        eprintln!("error: {}", e);
        process::exit(1);
    }
}
