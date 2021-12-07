use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    path::Path,
};

fn parse_markdown_file(filename: &str) {
    print_short_banner();
    println!("[INFO] Trying to parse {}", filename);

    // Prepare objects

    let filepath = Path::new(filename);
    let file = File::open(filepath).expect("[ERROR] Failed to open file!");
    let reader = BufReader::new(file);

    // Start parsing

    let mut ptag = false;

    let mut compiled_lines: Vec<String> = vec![];

    for line_result in reader.lines() {
        let line = line_result.unwrap();

        // TODO: check for space after first char.
        let mut output_line = String::new();
        let first_char = line.chars().take(1).collect::<Vec<char>>().pop();

        match first_char {
            Some('#') => {
                if ptag {
                    output_line.push_str("</p>\n\n");
                    ptag = false;
                }

                output_line.push_str("<h1>");
                output_line.push_str(&line[2..]);
                output_line.push_str("</h1>\n");
            }
            _ => {
                if !ptag {
                    ptag = true;
                    output_line.push_str("<p>");
                }

                output_line.push_str(&line);
            }
        }

        if ptag {
            ptag = false;
            output_line.push_str("</p>\n\n");
        }

        if output_line != "<p></p>\n\n" {
            compiled_lines.push(output_line);
        }
    }

    // Output to new file
    let mut output_filename = String::from(&filename[..filename.len() - 3]);
    output_filename.push_str(".html");

    let mut output_file = File::create(output_filename).expect("[ERROR] Could not create output file!");

    for line in &compiled_lines {
        output_file.write_all(line.as_bytes()).expect("[ERROR] Could not write to output file!");
    }

    println!("[INFO] Compilation complete!");
}

fn get_title() -> String {
    let mut title = String::from(env!("CARGO_PKG_NAME"));
    title.push_str(" (v");
    title.push_str(env!("CARGO_PKG_VERSION"));
    title.push_str("), ");
    title.push_str(env!("CARGO_PKG_DESCRIPTION"));

    title
}

fn print_short_banner() {
    println!("{}", get_title());
}

fn print_long_banner() {
    print_short_banner();
    println!(
        "Written by: {}\nHomepage: {}\nUsage: neotinymd <filename>.md",
        env!("CARGO_PKG_AUTHORS"),
        env!("CARGO_PKG_HOMEPAGE")
    );
}

fn usage() {
    print_long_banner();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        2 => parse_markdown_file(&args[1]),
        _ => usage(),
    }
}
