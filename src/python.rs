use pyo3::prelude::*;

use crate::orderbook::OrderBook as RustOrderBook;
use crate::models::{
    BookDepth, FillMetadata, OrderEvent as RustOrderEvent, OrderType as RustOrderType, Side, Trade,
};

/// Python wrappers around rust classes and return types, as we need
/// to deal with types like vectors and BTreeMaps outside of python.

#[derive(Debug, Copy, Clone, PartialEq)]
#[pyclass]
pub enum OrderType {
    /// A market order, which is either filled immediately (even partially), or
    /// canceled.
    Market,
    /// A limit order, which is either filled immediately, or added to the order
    /// book.
    Limit,
    /// A cancel order, which removes the order with the specified ID from the
    /// order book.
    Cancel,
}

/// An order to be executed by the order book.
#[derive(Debug, Copy, Clone)]
#[pyclass]
pub struct Order {
    /// The unique ID of this order.
    #[pyo3(get, set)]
    pub id: u128,
    /// The order side. It will be matched against the resting orders on the
    /// other side of the order book.
    #[pyo3(get, set)]
    pub side: Side,
    /// The order quantity.
    #[pyo3(get, set)]
    pub qty: f64,
    /// The limit price. The order book will only match this order with
    /// other orders at this price or better.
    #[pyo3(get, set)]
    pub price: f64,
    /// The order type
    #[pyo3(get, set)]
    pub order_type: OrderType,
}

#[pymethods]
impl Order {
    #[new]
    fn py_new(
        id: u128,
        side: Side,
        qty: f64,
        price: f64,
        order_type: OrderType) -> PyResult<Self> {
            Ok(Order { id, side, qty, price, order_type })
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[pyclass]
pub enum OrderEventType {
    /// Indicating that the corresponding order was not filled. It is only sent
    /// in response to market orders.
    Unfilled,
    /// Indicating that the corresponding order was placed on the order book. It
    /// is only send in response to limit orders.
    Placed,
    /// Indicating that the corresponding order was removed from the order book.
    /// It is only sent in response to cancel orders.
    Canceled,
    /// Indicating that the corresponding order was only partially filled. It is
    /// sent in response to market or limit orders.
    PartiallyFilled,
    /// Indicating that the corresponding order was filled completely. It is
    /// sent in response to market or limit orders.
    Filled,
}

/// An event resulting from the execution of an order.
#[derive(Debug, PartialEq, Clone)]
#[pyclass]
pub struct OrderEvent {
    #[pyo3(get, set)]
    pub id: u128,
    /// The filled quantity.
    #[pyo3(get, set)]
    pub filled_qty: f64,
    /// A vector with information on the order fills.
    #[pyo3(get, set)]
    pub fills: Vec<FillMetadata>,
    /// Type of order event
    #[pyo3(get, set)]
    pub event_type: OrderEventType,
}

#[pymethods]
impl OrderEvent {
    #[new]
    fn py_new(
        id: u128,
        filled_qty: f64,
        fills: Vec<FillMetadata>,
        event_type: OrderEventType) -> PyResult<Self> {
            Ok(OrderEvent { id, filled_qty, fills, event_type })
    }
}

#[derive(Debug)]
#[pyclass]
pub struct OrderBook{
    orderbook: RustOrderBook
}

#[pymethods]
impl OrderBook {
    #[new]
    fn py_new(
        arena_capacity: usize,
        queue_capacity: usize,
        precision: u128,
        track_stats: bool) -> PyResult<Self> {
            let orderbook = RustOrderBook::new(arena_capacity, queue_capacity, precision, track_stats);
            Ok(OrderBook { orderbook })
    }

    #[staticmethod]
    fn default() -> PyResult<OrderBook> {
        let orderbook = RustOrderBook::default();
        Ok(OrderBook { orderbook })
    }

    /// Return the lowest ask price, if present.
    #[inline(always)]
    pub fn min_ask(self_: PyRef<'_, Self>) -> PyResult<Option<f64>> {
        Ok(self_.orderbook.min_ask().clone())
    }

    /// Return the lowest ask price, if present.
    #[inline(always)]
    pub fn max_bid(self_: PyRef<'_, Self>) -> PyResult<Option<f64>> {
        Ok(self_.orderbook.max_bid().clone())
    }

    /// Return the last trade recorded while stats tracking was active as a
    /// [`Trade`] object, if present.
    ///
    /// [`Trade`]: struct.Trade.html
    #[inline(always)]
    pub fn last_trade(self_: PyRef<'_, Self>) -> PyResult<Option<Trade>> {
        Ok(self_.orderbook.last_trade().clone())
    }

    /// Return the total traded volume for all the trades that occurred while
    /// the stats tracking was active.
    #[inline(always)]
    pub fn traded_volume(self_: PyRef<'_, Self>) -> PyResult<f64> {
        Ok(self_.orderbook.traded_volume().clone())
    }

    pub fn depth(self_: PyRef<'_, Self>, levels: usize) -> PyResult<BookDepth> {
        Ok(self_.orderbook.depth(levels).clone())
    }

    /// Toggle the stats tracking on or off, depending on the `track` parameter.
    pub fn track_stats(mut self_: PyRefMut<Self>, track: bool) {
        self_.orderbook.track_stats(track)
    }

    /// Batch submit orders, to avoid memory allocation overhead in Python
    pub fn submit_batch(mut self_: PyRefMut<Self>, ids: Vec<u128>, qtys: Vec<f64>, prices: Vec<f64>, sides: Vec<Side>) -> PyResult<Vec<OrderEvent>> {
        let mut i = 0;
        let len = ids.len();
        let mut results: Vec<OrderEvent> = Vec::new();

        while i < len {
            let id = ids[i];
            let qty = qtys[i];
            let price = prices[i];
            let side = sides[i];
            let event: RustOrderEvent;
            let result: OrderEvent;

            if qty > 0.0 {
                if price > 0.0 {
                    event = self_.orderbook.execute(RustOrderType::Limit {
                        id: id,
                        qty: qty,
                        price: price,
                        side: side
                    });
                }
                else {
                    event = self_.orderbook.execute(RustOrderType::Market {
                        id: id,
                        qty: qty,
                        side: side
                    });
                }
            }
            else {
                event = self_.orderbook.execute(RustOrderType::Cancel {
                    id: id
                });
            }

            match event {
                RustOrderEvent::Unfilled { id } => {
                    result = OrderEvent {
                        id: id,
                        filled_qty: 0.0,
                        fills: Vec::new(),
                        event_type: OrderEventType::Unfilled
                    }
                }
                RustOrderEvent::Placed { id } => {
                    result = OrderEvent {
                        id: id,
                        filled_qty: 0.0,
                        fills: Vec::new(),
                        event_type: OrderEventType::Placed
                    }
                }
                RustOrderEvent::Canceled { id } => {
                    result = OrderEvent {
                        id: id,
                        filled_qty: 0.0,
                        fills: Vec::new(),
                        event_type: OrderEventType::Canceled
                    }
                }
                RustOrderEvent::PartiallyFilled { id, filled_qty, fills } => {
                    result = OrderEvent {
                        id: id,
                        filled_qty: filled_qty,
                        fills: fills.clone(),
                        event_type: OrderEventType::PartiallyFilled
                    }
                }
                RustOrderEvent::Filled { id, filled_qty, fills } => {
                    result = OrderEvent {
                        id: id,
                        filled_qty: filled_qty,
                        fills: fills.clone(),
                        event_type: OrderEventType::Filled
                    }
                }
            }

            results.push(result);
            i = i + 1;
        }
        Ok(results)
    }

    /// Submit a limit order
    pub fn submit_limit(mut self_: PyRefMut<Self>, id: u128, qty: f64, price: f64, side: Side) -> PyResult<OrderEvent> {
        let event: RustOrderEvent;
        let result: OrderEvent;

        event = self_.orderbook.execute(RustOrderType::Limit {
            id: id,
            qty: qty,
            price: price,
            side: side
        });

        match event {
            RustOrderEvent::Unfilled { id } => {
                result = OrderEvent {
                    id: id,
                    filled_qty: 0.0,
                    fills: Vec::new(),
                    event_type: OrderEventType::Unfilled
                }
            }
            RustOrderEvent::Placed { id } => {
                result = OrderEvent {
                    id: id,
                    filled_qty: 0.0,
                    fills: Vec::new(),
                    event_type: OrderEventType::Placed
                }
            }
            RustOrderEvent::Canceled { id } => {
                result = OrderEvent {
                    id: id,
                    filled_qty: 0.0,
                    fills: Vec::new(),
                    event_type: OrderEventType::Canceled
                }
            }
            RustOrderEvent::PartiallyFilled { id, filled_qty, fills } => {
                result = OrderEvent {
                    id: id,
                    filled_qty: filled_qty,
                    fills: fills.clone(),
                    event_type: OrderEventType::PartiallyFilled
                }
            }
            RustOrderEvent::Filled { id, filled_qty, fills } => {
                result = OrderEvent {
                    id: id,
                    filled_qty: filled_qty,
                    fills: fills.clone(),
                    event_type: OrderEventType::Filled
                }
            }
        }
        Ok(result)
    }

    /// Submit a limit order
    pub fn submit_market(mut self_: PyRefMut<Self>, id: u128, qty: f64, side: Side) -> PyResult<OrderEvent> {
        let event: RustOrderEvent;
        let result: OrderEvent;

        event = self_.orderbook.execute(RustOrderType::Market {
            id: id,
            qty: qty,
            side: side
        });

        match event {
            RustOrderEvent::Unfilled { id } => {
                result = OrderEvent {
                    id: id,
                    filled_qty: 0.0,
                    fills: Vec::new(),
                    event_type: OrderEventType::Unfilled
                }
            }
            RustOrderEvent::Placed { id } => {
                result = OrderEvent {
                    id: id,
                    filled_qty: 0.0,
                    fills: Vec::new(),
                    event_type: OrderEventType::Placed
                }
            }
            RustOrderEvent::Canceled { id } => {
                result = OrderEvent {
                    id: id,
                    filled_qty: 0.0,
                    fills: Vec::new(),
                    event_type: OrderEventType::Canceled
                }
            }
            RustOrderEvent::PartiallyFilled { id, filled_qty, fills } => {
                result = OrderEvent {
                    id: id,
                    filled_qty: filled_qty,
                    fills: fills.clone(),
                    event_type: OrderEventType::PartiallyFilled
                }
            }
            RustOrderEvent::Filled { id, filled_qty, fills } => {
                result = OrderEvent {
                    id: id,
                    filled_qty: filled_qty,
                    fills: fills.clone(),
                    event_type: OrderEventType::Filled
                }
            }
        }
        Ok(result)
    }

    /// Submit a cancel
    pub fn submit_cancel(mut self_: PyRefMut<Self>, id: u128) -> PyResult<OrderEvent> {
        self_.orderbook.execute(RustOrderType::Cancel {
            id: id
        });

        Ok(OrderEvent {
            id: id,
            filled_qty: 0.0,
            fills: Vec::new(),
            event_type: OrderEventType::Canceled
        })
    }

    /// Execute an order, returning immediately an event indicating the result.
    pub fn execute(mut self_: PyRefMut<Self>, order: Order) -> PyResult<OrderEvent> {
        let event: RustOrderEvent;
        let result: OrderEvent;

        match order.order_type {
            OrderType::Market => {
                event = self_.orderbook.execute(RustOrderType::Market {
                    id: order.id,
                    qty: order.qty,
                    side: order.side
                });
            }
            OrderType::Limit => {
                event = self_.orderbook.execute(RustOrderType::Limit {
                    id: order.id,
                    qty: order.qty,
                    price: order.price,
                    side: order.side
                });
            }
            OrderType::Cancel => {
                event = self_.orderbook.execute(RustOrderType::Cancel {
                    id: order.id
                });
            }
        }

        match event {
            RustOrderEvent::Unfilled { id } => {
                result = OrderEvent {
                    id: id,
                    filled_qty: 0.0,
                    fills: Vec::new(),
                    event_type: OrderEventType::Unfilled
                }
            }
            RustOrderEvent::Placed { id } => {
                result = OrderEvent {
                    id: id,
                    filled_qty: 0.0,
                    fills: Vec::new(),
                    event_type: OrderEventType::Placed
                }
            }
            RustOrderEvent::Canceled { id } => {
                result = OrderEvent {
                    id: id,
                    filled_qty: 0.0,
                    fills: Vec::new(),
                    event_type: OrderEventType::Canceled
                }
            }
            RustOrderEvent::PartiallyFilled { id, filled_qty, fills } => {
                result = OrderEvent {
                    id: id,
                    filled_qty: filled_qty,
                    fills: fills.clone(),
                    event_type: OrderEventType::PartiallyFilled
                }
            }
            RustOrderEvent::Filled { id, filled_qty, fills } => {
                result = OrderEvent {
                    id: id,
                    filled_qty: filled_qty,
                    fills: fills.clone(),
                    event_type: OrderEventType::Filled
                }
            }
        }
        Ok(result)
    }
}