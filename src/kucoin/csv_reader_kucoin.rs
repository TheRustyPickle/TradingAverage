use crate::sub_func::pair_coins::get_coin_names;
use crate::sub_func::order_data::OrderData;
use std::error::Error;
use std::fs::File;

pub fn read_csv_kucoin(file_name: &str) -> Result<OrderData, Box<dyn Error>> {

    let mut new_ins = OrderData::new();
    let file = File::open(file_name)?;
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.records() {
        let record = result?;

        let status = record.get(16).unwrap().to_string();

        if status == "done" {
            let mut pair = replace_dash(&record.get(3).unwrap().to_string());
            let order_side = record.get(4).unwrap();
            let executed: f32 = record.get(9).unwrap().trim().parse()?;
            let order_price: f32 = record.get(11).unwrap().trim().parse()?;
            let traded_with = get_coin_names(&pair);

            pair = pair.replace(&traded_with, "");
            
            match order_side {
                "buy" => new_ins.new_buy(pair, order_price, executed),
                "sell" => new_ins.new_sell(pair, order_price, executed),
                _ => {},
            }
        }
    }
    Ok(new_ins)
}

fn replace_dash(value: &str) -> String {
    value.replace("-", "")
    
}