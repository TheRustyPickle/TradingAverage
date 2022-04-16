use crate::xlsx_reader_binance::read_xlsx_bin;
use crate::sub_func::order_data::OrderData;
use crate::csv_reader::read_csv;
use std::fs;
use std::process;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

// goes through the current directory searching for the supported files
// and initializes the relevant functions. Use rust-analyzer for easier
// view of the numbers sent to the functions.

pub fn file_initiator() -> HashMap<String, Result<OrderData, Box<dyn Error>>>{
    let mut map = HashMap::new();
    let paths = fs::read_dir(".").unwrap();

    for path in paths {
        let mut path = path.unwrap().path().display().to_string();
        path = path.replace("./", "");

        match path.to_lowercase().as_ref() {
            "binance.csv" => {
                map.insert("binance_csv.txt".to_string(), read_csv(&path, 2, 4,
                                                                    9, 8, 11, "FILLED"));
            },
            "binance.xlsx" => {
                map.insert("binance_xlsx.txt".to_string(), read_xlsx_bin(&path));
            },
            "kucoin.csv" => {
                map.insert("kucoin.txt".to_string(), read_csv(&path, 3, 4, 11, 
                                                            9, 16, "done"));
            },
            _ => {},
        }
    }

    if map.is_empty() {
        println!("(filename).csv or (filename).xlsx not found in directory.\n\nAccepted filenames\n\nbinance.csv\nbinance.xlsx\nkucoin.csv");
        let mut open = File::create("data.txt").expect("Failed to create data.txt");
        open.write_all("(filename).csv or (filename).xlsx not found in directory.\n\nAccepted filenames\n\nbinance.csv\nbinance.xlsx\nkucoin.csv"
                            .as_bytes()).expect("Failed to create data.txt");
        process::exit(1)
    }

    map
}
