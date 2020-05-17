use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn parse_markdown_file(_filename: &str) {
    print_short_banner();
    println!("[ INFO ] Trying to parse {}...", _filename);

    let input_filename = Path::new(_filename);

    // Verbose
    // let file = match File::open(&input_filename) {
    //     Err(err) => panic!("[ ERROR ] File not found. {}", err.description()),
    //     Ok(value) => value,
    // };

    // Concise.
    let file = File::open(&input_filename).expect("[ ERROR ] File not found.");

    let mut _htag: bool = false;
    let mut _ptag: bool = false;
    let mut tokens: Vec<String> = Vec::new();

    let reader = BufReader::new(file);
    for line in reader.lines() {
        // Returned output.
        let mut output_line = String::new();

        // Verbose way.
        // let line_contents = match line {
        //     Err(err) => panic!("Jobs done..."),
        //     Ok(data) => data,
        // }
        // Concise way.
        let line_contents = line.unwrap().to_string();

        let mut first_char: Vec<char> = line_contents.chars().take(1).collect();
        match first_char.pop() {
            Some('#') => {
                if _ptag {
                    _ptag = false;
                    output_line.push_str("</p>\n");
                }
                if _htag {
                    _htag = false;
                    output_line.push_str("</h1>\n");
                }

                _htag = true;
                output_line.push_str("<h1>");
                output_line.push_str(&line_contents[2..]);
            }
            _ => {
                _ptag = true;
                output_line.push_str("<p>");
                output_line.push_str(&line_contents);
            }
        }

        if _ptag {
            _ptag = false;
            output_line.push_str("</p>\n");
        }

        if _htag {
            _htag = false;
            output_line.push_str("</h1>\n");
        }

        if output_line != "<p></p>\n" {
            tokens.push(output_line);
        }
    }

    let mut output_filename = String::from(&_filename[.._filename.len() - 3]);
    output_filename.push_str(".html");

    let mut outfile =
        File::create(output_filename.to_string())
            .expect("[ ERROR ] File could not be created.");

    for line in &tokens {
        outfile.write_all(line.as_bytes())
            .expect("[ ERROR ] Could not write to file.");
    }

    println!("[ INFO ] Parsing complete.");
}

fn print_long_banner() {
    print_short_banner();
    println!("Written by: {}", env!("CARGO_PKG_AUTHORS"));
    println!("Homepage: {}", env!("CARGO_PKG_HOMEPAGE"));
    println!("Usage: rdown <somefile>.md");
}

fn print_short_banner() {
    println!("{}", get_title());
}

fn get_title() -> String {
    let mut title = String::from(env!("CARGO_PKG_NAME"));
    title.push_str(" (v");
    title.push_str(env!("CARGO_PKG_VERSION"));
    title.push_str("), ");
    title.push_str(env!("CARGO_PKG_DESCRIPTION"));
    title
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        2 => parse_markdown_file(&args[1]),
        _ => print_long_banner(),
    }
}
