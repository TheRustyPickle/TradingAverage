use crate::csv_reader::read_csv;
use crate::xlsx_reader::read_xlsx;
use super::order_data::OrderData;
use std::fs;
use std::process;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn initiator() -> Result<OrderData, Box<dyn Error>> {
    let paths = fs::read_dir(".").unwrap();
    let mut file_to_read = Some(String::new());
    for path in paths {
        let mut path = path.unwrap().path().display().to_string();
        path = path.replace("./", "");
        if path == "data.csv" || path == "data.xlsx" {
            file_to_read = Some(path);
        }
    }
    match file_to_read {
        Some(file) => {
            println!("Reading file {}", file);
            if file == "data.csv" {
                read_csv(&file)
            }
            else if file == "data.xlsx" {
                read_xlsx(&file)
            }

            else {
                process::exit(1)
            }
        },

        None => {
            println!("data.csv or data.xlsx not found in directory");
            let mut open = File::create("data.txt").expect("Error creating txt file");
            open.write_all("data.csv or data.xlsx not found in directory".as_bytes()).expect("Error writing txt file");
            process::exit(1)},
    }
}