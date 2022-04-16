use crate::sub_func::pair_coins::get_coin_names;
use crate::sub_func::order_data::OrderData;
use calamine::{Reader, open_workbook, Xlsx};
use std::error::Error;

pub fn read_xlsx_bin(file_name: &str) -> Result<OrderData, Box<dyn Error>> {
    let mut workbook: Xlsx<_> = open_workbook(file_name)?;
    let mut new_ins = OrderData::new();

    if let Some(Ok(range)) = workbook.worksheet_range("sheet1 -1") {
        for cell in range.rows().skip(1) {
            match cell[8].get_string() {
                Some(v) => {
                    if v == "Filled" {
                        let mut pair = cell[1].get_string().unwrap().to_string();
                        let order_side = cell[2].get_string().unwrap();
                        let order_price: f32 = cell[5].get_string().unwrap().parse()?;
                        let executed: f32 = cell[6].get_string().unwrap().parse()?;
                        let traded_with = get_coin_names(&pair);

                        pair = pair.replace(&traded_with, "");

                        match order_side {
                            "BUY" => new_ins.new_buy(pair, order_price, executed),
                            "SELL" => new_ins.new_sell(pair, order_price, executed),
                            _ => {},
                        }
                    }
                },
                None => {}
            }
        }    
    }
    Ok(new_ins)
}