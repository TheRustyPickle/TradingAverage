use crate::sub_func::pair_coins::get_coin_names;
use crate::sub_func::order_data::OrderData;
use std::error::Error;
use std::fs::File;

pub fn read_csv_bin(file_name: &str) -> Result<OrderData, Box<dyn Error>> {

    let mut new_ins = OrderData::new();
    let file = File::open(file_name)?;
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.records() {
        let record = result?;

        let status = record.get(11).unwrap().to_string();

        if status == "FILLED" {
            let mut pair = record.get(2).unwrap().to_string();
            let order_side = record.get(4).unwrap();
            let unrefined_order_price = record.get(9).unwrap();
            let unrefined_executed = record.get(8).unwrap();
            let traded_with = get_coin_names(&pair);

            pair = pair.replace(&traded_with, "");

            let order_price: f32 = replace_comma(unrefined_order_price)[..10].trim().parse()?;
            let executed: f32 = replace_comma(unrefined_executed)[..10].trim().parse()?;
            
            match order_side {
                "BUY" => new_ins.new_buy(pair, order_price, executed),
                "SELL" => new_ins.new_sell(pair, order_price, executed),
                _ => {},
            }
        }
    }
    Ok(new_ins)
}

fn replace_comma(value: &str) -> String {
    value.replace(",", "")
    
}