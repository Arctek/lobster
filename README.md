<div align="center">
  <img alt="Lobster logo" src="https://github.com/Arctek/lobster/raw/master/images/logo.png" height="130" />
</div>

<div align="center">
  <h1>Lobster (now with Python)</h1>
  <p>A fast in-memory limit order book (LOB).</p>
  <a target="_blank" href="https://travis-ci.org/Arctek/lobster">
    <img src="https://img.shields.io/travis/Arctek/lobster?style=for-the-badge" alt="Build">
  </a>
  <a target="_blank" href="https://coveralls.io/github/Arctek/lobster">
    <img src="https://img.shields.io/coveralls/github/Arctek/lobster?style=for-the-badge" alt="Code Coverage">
  </a>
  <a target="_blank" href="https://crates.io/crates/lobster">
   <img src="https://img.shields.io/crates/d/lobster?style=for-the-badge" alt="Downloads (all time)">
  <a>
  <a href="https://github.com/Arctek/lobster/blob/master/LICENSE">
    <img src="https://img.shields.io/crates/l/lobster?style=for-the-badge" alt="ISC License">
  </a>
  <br>
  <br>
</div>


# Quickstart
To use Lobster, create an order book instance with default parameters, and send
orders for execution:

```rust
use lobster::{FillMetadata, OrderBook, OrderEvent, OrderType, Side};

let mut ob = OrderBook::default();
let event = ob.execute(OrderType::Market { id: 0, qty: 1.0, side: Side::Bid });
assert_eq!(event, OrderEvent::Unfilled { id: 0 });

let event = ob.execute(OrderType::Limit { id: 1, price: 120.0, qty: 3.0, side: Side::Ask });
assert_eq!(event, OrderEvent::Placed { id: 1 });

let event = ob.execute(OrderType::Market { id: 2, qty: 4.0, side: Side::Bid });
assert_eq!(
    event,
    OrderEvent::PartiallyFilled {
        id: 2,
        filled_qty: 3.0,
        fills: vec![
            FillMetadata {
                order_1: 2,
                order_2: 1,
                qty: 3.0,
                price: 120.0,
                taker_side: Side::Bid,
                total_fill: true,
            }
        ],
    },
);
```

```python
import unittest
from lobster import FillMetadata, OrderBook, Order, OrderType, OrderEvent, OrderEventType, Side

def order_event_equality(first, second, msg):
    passed = first.id == second.id and\
             first.filled_qty == second.filled_qty and\
             first.event_type == second.event_type and\
             len(first.fills) == len(second.fills)

    if first.event_type == OrderEventType.PartiallyFilled or\
       first.event_type == OrderEventType.Filled:
        for i in range(0, len(first.fills)):
            passed = passed and\
                     first.fills[i].order_1 == second.fills[i].order_1 and\
                     first.fills[i].order_2 == second.fills[i].order_2 and\
                     first.fills[i].qty == second.fills[i].qty and\
                     first.fills[i].price == second.fills[i].price and\
                     first.fills[i].side == second.fills[i].side and\
                     first.fills[i].total_fill == second.fills[i].total_fill

test_case = unittest.TestCase()
test_case.addTypeEqualityFunc(OrderEvent, order_event_equality)

ob = OrderBook.default()
event = ob.execute(Order(id=0, price=0.0, qty=1.0, side=Side.Bid, order_type=OrderType.Market))
test_case.assertEqual(event, OrderEvent(id=0, filled_qty=0.0, fills=[], event_type=OrderEventType.Unfilled))

event = ob.execute(Order(id=1, price=120.0, qty=3.0, side=Side.Ask, order_type=OrderType.Limit))
test_case.assertEqual(event, OrderEvent(id=1, filled_qty=0.0, fills=[], event_type=OrderEventType.Placed))

event = ob.execute(Order(id=2, price=0.0, qty=4.0, side=Side.Bid, order_type=OrderType.Market))
test_case.assertEqual(event, OrderEvent(
    id=1,
    filled_qty=3.0,
    fills=[
        FillMetadata(
            order_1=2,
            order_2=1,
            qty=3.0,
            price=120.0,
            taker_side=Side.Bid,
            total_fill=True
        )
    ],
    event_type=OrderEventType.PartiallyFilled
))
```

This fork of Lobster handles floating points for prices and quantities.
Price points are stored in a discrete fashion internally so there is a
conversion to a shifted unsigned 64-bit integer for referencing the BTreeMaps,
this is to 8 significant digits by default but can be changed.

Support has been added for python bindings, using PyO3. In tests it's
approximately 2.5x as slow as pure rust.

<div>
  <small>
    Logo made by <a href="https://www.flaticon.com/authors/turkkub"
    title="turkkub">turkkub</a> from <a href="https://www.flaticon.com/"
    title="Flaticon">www.flaticon.com</a>.
  </small>
</div>
