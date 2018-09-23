use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;

fn main() {
    let declaration = String::from("fn english_frequencies() -> HashMap<u8, f32> {[");

    // csv must be 2 columns, no header
    // ascii number, frequency as percentage
    // 32,17.16660
    let file = File::open("data/english.csv").unwrap();
    let reader = BufReader::new(&file);

    let formatted_lines = reader
        .lines()
        .map(|line| format!("({}),\n", line.unwrap()))
        .collect();

    let close = String::from("].iter().cloned().collect()}");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("english_frequencies.rs");
    let mut f = File::create(&dest_path).unwrap();

    f.write_all(
        &[declaration, formatted_lines, close]
            .join("\n")
            .into_bytes(),
    ).unwrap();
}
