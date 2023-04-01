use std::{
    fs,
    io,
    env,
    process,
    io::BufWriter,
};

use regex::Regex;
use chrono::Utc;

fn match_type_to_macro(cap_type: &str) -> Option<String> {
    match cap_type {
        "Booleans" => Some(String::from("define_boolean_capability")),
        "Strings" => Some(String::from("define_string_capability")),
        "Numbers" => Some(String::from("define_number_capability")),
        _ => None,
    }
}

fn create_file_content(source_content: &str) -> String {
    let mut output_content = String::new();

    let re = Regex::new(r"#\w*\s(?P<name>\w*)\s*\w*\s*(?P<type>Strings|Booleans|Numbers)\[(?P<pos>\d*)\]").unwrap();

    let mut lines: Vec<&str> = source_content.lines()
        .filter(|line| re.is_match(line))
        .collect();

    lines
        .sort_by_key(
                |line|
                {
                    let caps = re.captures(line).unwrap();

                    String::from(&caps["type"])
                });

    lines
        .into_iter()
        .map(
                |line|
                {
                    let caps = re.captures(line).unwrap();

                    let name = &caps["name"];
                    let cap_type = &caps["type"];
                    let pos = &caps["pos"];
                    let macro_name = match_type_to_macro(cap_type).unwrap();

                    format!("{macro_name}!({name}, {pos});\n")
                })
        .for_each(|line| output_content.push_str(&line) );

    output_content
}

fn remove_and_get_old(path: &str) -> Result<String, io::Error> {
    let old_content = fs::read_to_string(path)?;

    match old_content.find("/*\n\tcreate_capabilities_lines") {
        Some(pos) => {
            let removed = old_content.split_at(pos);
            Ok(String::from(removed.0))
        },
        None => Ok(old_content)
    }
}

fn write_content_to_file(content: &str, path: &str) -> Result<(), io::Error>{
    use io::Write;

    let old_content = remove_and_get_old(path)?;
    let file = fs::File::options().write(true).create(true).open(path)?;
    let mut file = BufWriter::with_capacity(1000, file);

    let time = Utc::now().format("%Y-%m-%d %H:%M:%S");
    let msg = format!(
"/*
\tcreate_capabilities_lines
\tCreated with create_capabilities binary crate of project.
\tAt: {}
*/", time);
    let content = format!("{}\n{}\n\n{}", old_content, msg, content);

    file.write_all(content.as_bytes())?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        let program_path = &args[0];

        eprintln!("Missing args");
        eprintln!("Usage:");
        eprintln!("\t{program_path} term.h result.rs");

        process::exit(1);
    }

    let source_path = &args[1];
    let des_path = &args[2];
    let source_content = fs::read_to_string(source_path).unwrap();
    let content = create_file_content(&source_content);
    write_content_to_file(&content, des_path).unwrap();
}