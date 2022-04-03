mod csv_reader;
mod xlsx_reader;
mod sub_func;

use crate::sub_func::initiator;
use std::process;
use std::fs::File;
use std::io::prelude::*;


fn main() {
    
    let data = match initiator() {
        Ok(data) => data,
        Err(e) => {
            println!("Error reading file data. Error: {}", e);
            let full_text = format!("Error reading file data. Error: {}", e);
            let mut open = File::create("data.txt").expect("Error creating txt file");
            open.write_all(full_text.as_bytes()).expect("Error writing txt file");
            process::exit(1);
        }
    };
    data.print_write_average();
}