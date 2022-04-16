use crate::sub_func::pair_coins::get_coin_names;
use crate::sub_func::order_data::OrderData;
use std::error::Error;
use std::fs::File;

pub fn read_csv(file_name: &str, pair_num: usize, order_side_num: usize,
    order_price_num: usize, executed_num: usize, status_num: usize,
    status_str: &str) -> Result<OrderData, Box<dyn Error>> {

    let mut new_ins = OrderData::new();
    let file = File::open(file_name)?;
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.records() {
        let record = result?;

        let status = record.get(status_num).unwrap().to_string();

        if status == status_str {
            let mut pair = replace_dash(record.get(pair_num).unwrap());
            let order_side = record.get(order_side_num).unwrap();
            let order_price = replace_dash_comma(record.get(order_price_num).unwrap())[..10].trim().parse::<f32>()?;
            let executed = replace_dash_comma(record.get(executed_num).unwrap())[..10].trim().parse::<f32>()?;

            // this part if added because all USD like stablecoins are all
            // valued 1 USD. So if same coin is traded with with multiple different
            // USD stablecoins, group them together.
            
            let traded_with = get_coin_names(&pair);
            pair = pair.replace(&traded_with, "");

            match order_side.to_lowercase().as_ref() {
                "buy" => new_ins.new_buy(pair, order_price, executed),
                "sell" => new_ins.new_sell(pair, order_price, executed),
                _ => {},
            }
        }
    }
    Ok(new_ins)
}

fn replace_dash_comma(value: &str) -> String {
    let mut val = value.replace(",", "");

    // add extra zero because if the len is < 10, it will panic.
    // 10 is important because binance's 11th to last order price
    // value is not a number.

    if val.len() < 10 && val.contains(".") == false {
        val = format!("{}.0000000000", val);
    }
    else {
        val = format!("{}0000000000", val);
    }
    val
}

// the func below especially required for clearing kucoin pair

fn replace_dash(value: &str) -> String {
    value.replace("-", "")
}