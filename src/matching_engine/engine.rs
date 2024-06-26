use std::collections::HashMap;
use super::orderbook::{Order, OrderBook};

// BTCUSD
//  BTC => BASE
// USD => QUOTE
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct TradingPair{
    base: String,
    quote: String,
}

impl TradingPair {
    pub fn new(base: String, quote: String) -> TradingPair {
        TradingPair {
            base, quote
        }
    }

    pub fn to_string(&self) -> String {
     format!("{}_{}", self.base, self.quote)   
    }
    
}

pub struct MatchingEngine {
    orderbooks: HashMap<TradingPair, OrderBook>
}

impl MatchingEngine {
    pub fn new() -> MatchingEngine {
        MatchingEngine {
            orderbooks: HashMap::new()
        }
    }

    pub fn new_market(&mut self, pair: &TradingPair){
        self.orderbooks.insert(pair.clone(), OrderBook::new());
        print!("Opening new order book for market {:?}", pair.to_string())
    }

    pub fn place_limit_order(
        &mut self, 
        pair: &TradingPair, 
        price: f64, 
        order: &Order
    ) -> Result<(), String>{
        match self.orderbooks.get_mut(pair) {
            Some(order_book) => {
                order_book.add_order(price, order.clone());

                println!("place limit order: {:?}", &order);
                Ok(())
            },
            None => {
                Err(
                    format!("The order book for the given trading pair {} does not exist",
                    pair.to_string()
                ))
            }
        }
    }
}