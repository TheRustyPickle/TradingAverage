mod sub_func;
mod csv_reader;
mod xlsx_reader_binance;

use sub_func::func_initiator::file_initiator;
use std::process;
use std::fs::File;
use std::io::prelude::*;


fn main() {
    let all_data = file_initiator();
    for (key, value) in all_data.iter() {
        match value {
            Ok(v) => {
                v.print_write_average(key)
            }
            Err(e) => {
                println!("Error reading file data. Error: {}", e);
                let full_text = format!("Error reading file data. Error: {}", e);
                let mut open = File::create("data.txt").expect("Error creating txt file");
                open.write_all(full_text.as_bytes()).expect("Error writing txt file");
                process::exit(1);
            }
        }
    }
}