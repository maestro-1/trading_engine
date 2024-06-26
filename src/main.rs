mod matching_engine;
use matching_engine::orderbook::{OrderBook, Order, OrderType};
use matching_engine::engine::{MatchingEngine, TradingPair};
use rust_decimal_macros::dec;


fn main() {
    let buy_order_from_alice = Order::new(OrderType::Bid, 65.3);
    let buy_order_from_bob = Order::new(OrderType::Bid, 65.3);

    let mut order_book = OrderBook::new();

    order_book.add_limit_order(dec!(4.4), buy_order_from_alice);
    order_book.add_limit_order(dec!(4.4), buy_order_from_bob);

    let sell_order_from_alice = Order::new(OrderType::Ask, 245.3);
    let sell_order_from_bod = Order::new(OrderType::Ask, 245.3);

    order_book.add_limit_order(dec!(4.4), sell_order_from_alice);
    order_book.add_limit_order(dec!(4.4), sell_order_from_bod);

    let mut engine = MatchingEngine::new();
    let btc_pair = TradingPair::new("BTC".to_string(), "USD".to_string());
    
    engine.new_market(&btc_pair);

    
    let buy_order = Order::new(OrderType::Bid, 65.3);
    
    engine.place_limit_order(&btc_pair, dec!(10), &buy_order).unwrap();
}
