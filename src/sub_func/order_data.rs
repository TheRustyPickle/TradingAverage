use std::collections::HashMap;
use std::io::prelude::*;
use std::fs::File;

#[derive(Debug)]
pub struct OrderData {
    buy_prices: HashMap<String, Vec<f32>>,
    sell_prices: HashMap<String, Vec<f32>>,
}

impl OrderData  {
    pub fn new() -> Self {
        OrderData {
            buy_prices: HashMap::new(),
            sell_prices: HashMap::new(),
        }
    }

    pub fn new_buy(&mut self, pair: String, buy_price: f32, amount: f32) {
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

    pub fn new_sell(&mut self, pair: String, sell_price: f32, amount: f32) {
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

    pub fn print_write_average(&self, file_name: &str) {
        let mut txt_data = String::new();
        let mut sorted_buy:Vec<_> = self.buy_prices.iter().collect();
        let mut sorted_sell: Vec<_> = self.sell_prices.iter().collect();

        sorted_buy.sort_by_key(|a| a.0);
        sorted_sell.sort_by_key(|a| a.0);

        for (key, value) in sorted_buy {
            let total_value = value[0];
            let total_amount = value[1];
            let average_buy = total_value / total_amount;
            let s1 = format!("average buy price of {} of is {} amount {}\n", key, average_buy, total_amount);
            txt_data += &s1;
        }
        
        for (key, value) in sorted_sell {
            let total_value = value[0];
            let total_amount = value[1];
            let average_buy = total_value / total_amount;
            let s2 = format!("\naverage sell price of {} of is {} amount {}", key, average_buy, total_amount);
            txt_data += &s2;
        }

        let mut open = File::create(file_name).expect("Error creating txt file");
        open.write_all(txt_data.as_bytes()).expect("Error writing txt file");
    }
}