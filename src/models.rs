use pyo3::prelude::*;

/// An order book side.
#[derive(Debug, Copy, Clone, PartialEq)]
#[pyclass]
pub enum Side {
    /// The bid (or buy) side.
    Bid,
    /// The ask (or sell) side.
    Ask,
}

impl std::ops::Not for Side {
    type Output = Side;

    fn not(self) -> Self::Output {
        match self {
            Side::Bid => Side::Ask,
            Side::Ask => Side::Bid,
        }
    }
}

/// An order to be executed by the order book.
#[derive(Debug, Copy, Clone)]
pub enum OrderType {
    /// A market order, which is either filled immediately (even partially), or
    /// canceled.
    Market {
        /// The unique ID of this order.
        id: u128,
        /// The order side. It will be matched against the resting orders on the
        /// other side of the order book.
        side: Side,
        /// The order quantity.
        qty: f64,
    },
    /// A limit order, which is either filled immediately, or added to the order
    /// book.
    Limit {
        /// The unique ID of this order.
        id: u128,
        /// The order side. It will be matched against the resting orders on the
        /// other side of the order book.
        side: Side,
        /// The order quantity.
        qty: f64,
        /// The limit price. The order book will only match this order with
        /// other orders at this price or better.
        price: f64,
    },
    /// A cancel order, which removes the order with the specified ID from the
    /// order book.
    Cancel {
        /// The unique ID of the order to be canceled.
        id: u128,
    },
}

/// An event resulting from the execution of an order.
#[derive(Debug, PartialEq, Clone)]
pub enum OrderEvent {
    /// Indicating that the corresponding order was not filled. It is only sent
    /// in response to market orders.
    Unfilled {
        /// The ID of the order this event is referring to.
        id: u128,
    },
    /// Indicating that the corresponding order was placed on the order book. It
    /// is only send in response to limit orders.
    Placed {
        /// The ID of the order this event is referring to.
        id: u128,
    },
    /// Indicating that the corresponding order was removed from the order book.
    /// It is only sent in response to cancel orders.
    Canceled {
        /// The ID of the order this event is referring to.
        id: u128,
    },
    /// Indicating that the corresponding order was only partially filled. It is
    /// sent in response to market or limit orders.
    PartiallyFilled {
        /// The ID of the order this event is referring to.
        id: u128,
        /// The filled quantity.
        filled_qty: f64,
        /// A vector with information on the order fills.
        fills: Vec<FillMetadata>,
    },
    /// Indicating that the corresponding order was filled completely. It is
    /// sent in response to market or limit orders.
    Filled {
        /// The ID of the order this event is referring to.
        id: u128,
        /// The filled quantity.
        filled_qty: f64,
        /// A vector with information on the order fills.
        fills: Vec<FillMetadata>,
    },
}

/// Information on a single order fill. When an order is matched with multiple
/// resting orders, it generates multiple `FillMetadata` values.
#[derive(Debug, PartialEq, Copy, Clone)]
#[pyclass]
pub struct FillMetadata {
    /// The ID of the order that triggered the fill (taker).
    #[pyo3(get, set)]
    pub order_1: u128,
    /// The ID of the matching order.
    #[pyo3(get, set)]
    pub order_2: u128,
    /// The quantity that was traded.
    #[pyo3(get, set)]
    pub qty: f64,
    /// The price at which the trade happened.
    #[pyo3(get, set)]
    pub price: f64,
    /// The side of the taker order (order 1)
    #[pyo3(get, set)]
    pub taker_side: Side,
    /// Whether this order was a total (true) or partial (false) fill of the
    /// maker order.
    #[pyo3(get, set)]
    pub total_fill: bool,
}

#[pymethods]
impl FillMetadata {
    #[new]
    fn py_new(
        order_1: u128,
        order_2: u128,
        qty: f64,
        price: f64,
        taker_side: Side,
        total_fill: bool
        ) -> PyResult<Self> {
            Ok(FillMetadata { order_1, order_2, qty, price, taker_side, total_fill })
    }
}

/// A snapshot of the order book up to a certain depth level. Multiple orders at
/// the same price points are merged into a single [`BookLevel`] struct.
///
/// [`BookLevel`]: /struct.BookLevel.html
#[derive(Debug, Clone, PartialEq)]
#[pyclass]
pub struct BookDepth {
    /// The requested level. This field will always contain the level that was
    /// requested, even if some or all levels are empty.
    #[pyo3(get, set)]
    pub levels: usize,
    /// A vector of price points with the associated quantity on the ask side.
    #[pyo3(get, set)]
    pub asks: Vec<BookLevel>,
    /// A vector of price points with the associated quantity on the bid side.
    #[pyo3(get, set)]
    pub bids: Vec<BookLevel>,
}

#[pymethods]
impl BookDepth {
    #[new]
    fn py_new(
        levels: usize,
        asks: Vec<BookLevel>,
        bids: Vec<BookLevel>
        ) -> PyResult<Self> {
            Ok(BookDepth { levels, asks, bids })
    }
}

/// A single level in the order book. This struct is used both for the bid and
/// ask side.
#[derive(Debug, Clone, PartialEq)]
#[pyclass]
pub struct BookLevel {
    /// The price point this level represents.
    #[pyo3(get, set)]
    pub price: f64,
    /// The total quantity of all orders resting at the specified price point.
    #[pyo3(get, set)]
    pub qty: f64,
}

#[pymethods]
impl BookLevel {
    #[new]
    fn py_new(
        price: f64,
        qty: f64
        ) -> PyResult<Self> {
            Ok(BookLevel { price, qty })
    }
}

/// A trade that happened as part of the matching process.
#[derive(Debug, Copy, Clone)]
#[pyclass]
pub struct Trade {
    /// The total quantity transacted as part of this trade.
    #[pyo3(get, set)]
    pub total_qty: f64,
    /// The volume-weighted average price computed from all the order fills
    /// within this trade.
    #[pyo3(get, set)]
    pub avg_price: f64,
    /// The price of the last fill that was part of this trade.
    #[pyo3(get, set)]
    pub last_price: f64,
    /// The quantity of the last fill that was part of this trade.
    #[pyo3(get, set)]
    pub last_qty: f64,
}

#[pymethods]
impl Trade {
    #[new]
    fn py_new(
        total_qty: f64,
        avg_price: f64,
        last_price: f64,
        last_qty: f64
        ) -> PyResult<Self> {
            Ok(Trade { total_qty, avg_price, last_price, last_qty })
    }
}

#[derive(Debug, PartialEq)]
pub struct LimitOrder {
    pub id: u128,
    pub qty: f64,
    pub price: f64,
}

#[cfg(test)]
mod test {
    use super::Side;

    #[test]
    fn side_negation() {
        assert_eq!(!Side::Ask, Side::Bid);
        assert_eq!(!Side::Bid, Side::Ask);
    }
}
