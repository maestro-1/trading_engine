#![allow(dead_code)]
use std::collections::HashMap;
use rust_decimal::prelude::*;

#[derive(Debug, Clone)]
pub enum OrderType {
    Bid,
    Ask
}

#[derive(Debug)]
pub struct OrderBook {
    asks: HashMap<Decimal, Limit>,
    bids: HashMap<Decimal, Limit>
}

impl OrderBook {
    pub fn new() -> OrderBook{
        OrderBook { 
            asks: HashMap::new(), 
            bids: HashMap::new() 
        }
    }

    pub fn fill_market_order(&mut self, market_order: &mut Order) {
        let limits = match market_order.order_type {
            OrderType::Bid => self.ask_limits(),
            OrderType::Ask => self.bid_limits()
        };

        for limit_order in limits {
            limit_order.fill_order(market_order);

            if market_order.is_filled(){
                break;
            }
        }
    }

    fn ask_limits(&mut self) -> Vec<& mut Limit> {
        // make type collected explicit with ::<Vec<&mut Limit>>
        let mut limit = self.asks.values_mut().collect::<Vec<&mut Limit>>();
        limit
        .sort_by(|a, b| a.price.cmp(&b.price));
        
        limit
        
    }

    fn bid_limits(&mut self) -> Vec<& mut Limit> {
        // make type collected explicit with ::<Vec<&mut Limit>>
        let mut limit = self.bids.values_mut().collect::<Vec<&mut Limit>>();
        limit
        .sort_by(|a, b| b.price.cmp(&a.price));
        
        limit
        
    }

    pub fn add_limit_order(&mut self, price: Decimal, order: Order){
        match order.order_type {
            OrderType::Bid => {
                match self.bids.get_mut(&price) {
                    Some(limit) => {
                        limit.add_order(order);
                    },
                    None => {
                        let mut limit =  Limit::new(price);
                        limit.add_order(order);
                        self.bids.insert(price, limit);
                    }
                }
            }
            OrderType::Ask => {
                match self.bids.get_mut(&price) {
                    Some(limit) => {
                        limit.add_order(order);
                    },
                    None => {
                        let mut limit =  Limit::new(price);
                        limit.add_order(order);
                        self.asks.insert(price, limit);
                    }
                }
            }
        }
    }
}


#[derive(Debug)]
struct Limit {
    /* 
    f64 isn't good for hashing, it can run into inconsistent values
    */
    price: Decimal,
    orders: Vec<Order>
}

impl  Limit {
    fn new(price: Decimal) -> Limit {
        Limit { price, orders: Vec::new() }
    }

    fn total_volume(&self) -> f64 {
        self.orders
        .iter()
        .map(|order| order.size)
        .reduce(|a, b| a + b).unwrap()
    }
    
    fn fill_order(&mut self, market_order: &mut Order) {
        for limit_order in self.orders.iter_mut() {
            match market_order.size >= limit_order.size {
                true => {
                    market_order.size -= limit_order.size;
                    limit_order.size = 0.0;
                },
                false => {
                    limit_order.size -= market_order.size;
                    market_order.size = 0.0;
                }
            }

            if market_order.is_filled() {
                break;
            }
        }
    }

    fn add_order(&mut self, order: Order){
        self.orders.push(order);
    }
}

#[derive(Debug, Clone)]
pub struct  Order {
    size: f64,
    order_type: OrderType
}

impl Order {
    pub fn new(order_type: OrderType, size: f64) -> Order {
        Order { size, order_type}
    }

    pub fn is_filled(&self) -> bool {
        self.size == 0.0
    }
}




#[cfg(test)]
pub mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    
    #[test]
    fn orderbook_fill_market_order_ask() {
        let mut orderbook = OrderBook::new();
        orderbook.add_limit_order(dec!(500), Order::new(OrderType::Ask, 10.0));
        orderbook.add_limit_order(dec!(200), Order::new(OrderType::Ask, 10.0));
        orderbook.add_limit_order(dec!(400), Order::new(OrderType::Ask, 10.0));
        orderbook.add_limit_order(dec!(300), Order::new(OrderType::Ask, 10.0));
        orderbook.add_limit_order(dec!(100), Order::new(OrderType::Ask, 10.0));

        let mut market_order = Order::new(OrderType::Bid, 10.0);
        orderbook.fill_market_order(&mut market_order);

        let ask_limit = orderbook.ask_limits();
        let match_limit = ask_limit.get(0).unwrap(); //.orders.get(0).unwrap();

        // assert_eq!(match_limit, dec!(100));
    }
    
    
    #[test]
    fn limit_total_volume(){
        let price = dec!(10_000);
        let mut limit = Limit::new(price);

        let buy_limit_order_a = Order::new(OrderType::Bid, 100.0);
        let buy_limit_order_b = Order::new(OrderType::Bid, 100.0);

        limit.add_order(buy_limit_order_a);
        limit.add_order(buy_limit_order_b);

        assert_eq!(limit.total_volume(), 200.0);
    }

    #[test]
    fn limit_order_multi_fill(){
        let price = dec!(10_000);
        let mut limit = Limit::new(price);

        let buy_limit_order_a = Order::new(OrderType::Bid, 100.0);
        let buy_limit_order_b = Order::new(OrderType::Bid, 100.0);
        limit.add_order(buy_limit_order_a);
        limit.add_order(buy_limit_order_b);

        let mut market_sell_order = Order::new(OrderType::Ask, 199.0);
        limit.fill_order(&mut market_sell_order);
        
        assert_eq!(market_sell_order.is_filled(), true);
        assert_eq!(limit.orders.get(0).unwrap().is_filled(), true);
        assert_eq!(limit.orders.get(1).unwrap().is_filled(), false);
        assert_eq!(limit.orders.get(1).unwrap().size, 1.0);
    }

    #[test]
    fn limit_order_single_fill(){
        let price = dec!(10_000);
        let mut limit = Limit::new(price);

        let buy_limit_order = Order::new(OrderType::Bid, 100.0);
        limit.add_order(buy_limit_order);

        let mut market_sell_order = Order::new(OrderType::Ask, 99.0);
        limit.fill_order(&mut market_sell_order);
        
        assert_eq!(market_sell_order.is_filled(), true);
        assert_eq!(limit.orders.get(0).unwrap().size, 1.0);
    }
}