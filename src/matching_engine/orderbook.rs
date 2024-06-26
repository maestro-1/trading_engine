use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum OrderType {
    Bid,
    Ask
}

#[derive(Debug)]
pub struct OrderBook {
    asks: HashMap<Price, Limit>,
    bids: HashMap<Price, Limit>
}

impl OrderBook {
    pub fn new() -> OrderBook{
        OrderBook { 
            asks: HashMap::new(), 
            bids: HashMap::new() 
        }
    }

    pub fn fill_market_order(&mut self, market_order: &mut Order) {
        match market_order.order_type {
            OrderType::Bid => {
                for limit_order in self.ask_limits(){
                    limit_order.fill_order(market_order);

                    if market_order.is_filled() {
                        break;
                    }
                }
            },
            OrderType::Ask => {}
        }
    }

    fn ask_limits(&mut self) -> Vec<& mut Limit> {
        // make type collected explicit with ::<Vec<&mut Limit>>
        self.asks.values_mut().collect::<Vec<&mut Limit>>()
        
    }

    fn bid_limits(&mut self) -> Vec<& mut Limit> {
        // make type collected explicit with ::<Vec<&mut Limit>>
        self.bids.values_mut().collect::<Vec<&mut Limit>>()
        
    }

    pub fn add_order(&mut self, price: f64, order: Order){
        let price = Price::new(price);
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


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Price {
    integral: u64,
    fractional: u64,
    scalar: u64
}

impl Price {
    pub fn new(price: f64) -> Price {
        let scalar = 100_000;
        let integral = price as u64;
        let fractional = ((price % 1.0) * scalar as f64) as u64;

        Price {
            scalar, integral, fractional
        }
    }
}

#[derive(Debug)]
struct Limit {
    /* 
    f64 isn't good for hashing, it can run into inconsistent values
    */
    price: Price,
    orders: Vec<Order>
}

impl  Limit {
    fn new(price: Price) -> Limit {
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

    #[test]
    fn limit_total_volume(){
        let price = Price::new(10_000.00);
        let mut limit = Limit::new(price);

        let buy_limit_order_a = Order::new(OrderType::Bid, 100.0);
        let buy_limit_order_b = Order::new(OrderType::Bid, 100.0);

        limit.add_order(buy_limit_order_a);
        limit.add_order(buy_limit_order_b);

        assert_eq!(limit.total_volume(), 200.0);
    }

    #[test]
    fn limit_order_multi_fill(){
        let price = Price::new(10_000.00);
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
        let price = Price::new(10_000.00);
        let mut limit = Limit::new(price);

        let buy_limit_order = Order::new(OrderType::Bid, 100.0);
        limit.add_order(buy_limit_order);

        let mut market_sell_order = Order::new(OrderType::Ask, 99.0);
        limit.fill_order(&mut market_sell_order);
        
        assert_eq!(market_sell_order.is_filled(), true);
        assert_eq!(limit.orders.get(0).unwrap().size, 1.0);
    }
}