pub fn get_coin_names(pair: &str) -> String {
    let stablecoins = vec!["USDT", "USDC", "PAX", "BUSD", "DAI"];
    let mut traded_with = String::new();
    for i in stablecoins.iter() {
        let split = pair.split(i);
        let vec:Vec<&str> = split.collect();
        //println!("{:?}", vec);
        if vec.len() > 1 && vec[1] == ""   {
            traded_with = i.to_string()
        }
    }
    traded_with
}