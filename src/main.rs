use std::error::Error;
use std::process;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

struct OrderData {
    buy_prices: HashMap<String, Vec<f32>>,
    sell_prices: HashMap<String, Vec<f32>>,
}

impl OrderData  {
    fn new() -> Self {
        OrderData {
            buy_prices: HashMap::new(),
            sell_prices: HashMap::new(),
        }
    }

    fn new_buy(&mut self, pair: String, buy_price: f32, amount: f32) {
        match self.buy_prices.get(&pair) {
            Some(_) => {
                let total_purchase_value = self.buy_prices.get(&pair).unwrap()[0] + amount * buy_price;
                let new_amount = self.buy_prices.get(&pair).unwrap()[1] + amount;
                *self.buy_prices.get_mut(&pair).unwrap() = vec!(total_purchase_value, new_amount)
            },
            None => {
                self.buy_prices.insert(pair, vec!(buy_price * amount, amount));
            }
        }

    }

    fn new_sell(&mut self, pair: String, sell_price: f32, amount: f32) {
        match self.sell_prices.get(&pair) {
            Some(_) => {
                let total_purchase_value = self.sell_prices.get(&pair).unwrap()[0] + amount * sell_price;
                let new_amount = self.sell_prices.get(&pair).unwrap()[1] + amount;
                *self.sell_prices.get_mut(&pair).unwrap() = vec!(total_purchase_value, new_amount)
            },
            None => {
                self.sell_prices.insert(pair, vec!(sell_price * amount, amount));
            }
        }
    }

    fn print_write_average(&self) {
        let mut txt_data = String::new();
        let mut sorted_buy:Vec<_> = self.buy_prices.iter().collect();
        let mut sorted_sell: Vec<_> = self.sell_prices.iter().collect();

        sorted_buy.sort_by_key(|a| a.0);
        sorted_sell.sort_by_key(|a| a.0);

        for (key, value) in sorted_buy {
            let total_value = value[0];
            let total_amount = value[1];
            let average_buy = total_value / total_amount;
            println!("average buy price of {} of is {} amount {}", key, average_buy, total_amount);
            let s1 = format!("average buy price of {} of is {} amount {}\n", key, average_buy, total_amount);
            txt_data += &s1;
        }
        
        for (key, value) in sorted_sell {
            let total_value = value[0];
            let total_amount = value[1];
            let average_buy = total_value / total_amount;
            println!("average sell price of {} of is {} amount {}", key, average_buy, total_amount);
            let s2 = format!("\naverage sell price of {} of is {} amount {}", key, average_buy, total_amount);
            txt_data += &s2;
        }

        let mut open = File::create("data.txt").expect("Error creating txt file");
        open.write_all(txt_data.as_bytes()).expect("Error writing txt file");
    }
}

fn read_csv(file_name: &str) -> Result<OrderData, Box<dyn Error>> {
    let mut new_ins = OrderData::new();
    let file = File::open(file_name)?;
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.records() {
        let record = result?;

        let status = record.get(11).unwrap().to_string();

        if status == "FILLED" {
            let pair = record.get(2).unwrap().to_string();
            let order_side = record.get(4).unwrap();
            let unrefined_order_price = record.get(9).unwrap();
            let unrefined_executed = record.get(8).unwrap();
            
            let order_price: f32 = replace_comma(unrefined_order_price)[..10].trim().parse().expect("Error Parsing order price");
            let executed: f32 = replace_comma(unrefined_executed)[..10].trim().parse().expect("Error Parsing executed");
            
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