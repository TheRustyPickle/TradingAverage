mod csv_reader;
use csv_reader::read_csv;
use std::process;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let csv_data = match read_csv("data.csv") {
        Ok(data) => data,
        Err(e) => {
            println!("Error reading CSV or data.csv not found. Error: {}", e);
            let full_text = format!("Error reading CSV or data.csv not found. Error: {}", e);
            let mut open = File::create("data.txt").expect("Error creating txt file");
            open.write_all(full_text.as_bytes()).expect("Error writing txt file");
            process::exit(1);
        }
    };

    csv_data.print_write_average();
}