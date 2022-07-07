use pyo3::prelude::*;

use crate::arena::OrderArena;
use crate::orderbook::OrderBook;
use crate::models::{
    BookDepth, BookLevel, FillMetadata, OrderEvent, OrderType, Side, Trade,
};

/// Python wrappers around rust classes and return types, as we need
/// to deal with types like vectors and BTreeMaps outside of python.
#[pyclass]
pub struct PythonOrderBook{
    orderbook: OrderBook
}

#[pymethods]
impl PythonOrderBook {
    #[new]
    fn py_new(
        arena_capacity: usize,
        queue_capacity: usize,
        precision: u128,
        track_stats: bool) -> PyResult<Self> {
            let orderbook = OrderBook::new(arena_capacity, queue_capacity, precision, track_stats);
            Ok(PythonOrderBook { orderbook })
    }

    /// Return the lowest ask price, if present.
    #[inline(always)]
    pub fn min_ask(self_: PyRef<'_, Self>) -> PyResult<f64> {
        self_.orderbook.min_ask()
    }

    /// Return the lowest ask price, if present.
    #[inline(always)]
    pub fn max_bid(self_: PyRef<'_, Self>) -> PyResult<f64> {
        self_.orderbook.max_bid()
    }
}