//! Lobster implements a single-threaded order book. To use Lobster, create an
//! order book instance with default parameters, and send orders for execution:
//!
//! ```rust
//! use lobster::{FillMetadata, OrderBook, OrderEvent, OrderType, Side};
//!
//! let mut ob = OrderBook::default();
//! let event = ob.execute(OrderType::Market { id: 0, qty: 1.0, side: Side::Bid });
//! assert_eq!(event, OrderEvent::Unfilled { id: 0 });
//!
//! let event = ob.execute(OrderType::Limit { id: 1, price: 120.0, qty: 3.0, side: Side::Ask });
//! assert_eq!(event, OrderEvent::Placed { id: 1 });
//!
//! let event = ob.execute(OrderType::Market { id: 2, qty: 4.0, side: Side::Bid });
//! assert_eq!(
//!     event,
//!     OrderEvent::PartiallyFilled {
//!         id: 2,
//!         filled_qty: 3.0,
//!         fills: vec![
//!             FillMetadata {
//!                 order_1: 2,
//!                 order_2: 1,
//!                 qty: 3.0,
//!                 price: 120.0,
//!                 taker_side: Side::Bid,
//!                 total_fill: true,
//!             }
//!         ],
//!     },
//! );
//! ```
//!
//! This fork of Lobster supports floating price points and quantities. Prices and
//! quantities are represented as signed 64-bit floating points.
//! Price points are stored in a discrete fashion internally so there is a
//! conversion to a shifted uinsigned 64-bit integer for referencing the BTreeMaps.
//!
//! Because this fork supports bindings for python, several enums have been changed 

#![warn(missing_docs, missing_debug_implementations, rustdoc::broken_intra_doc_links)]

//use pyo3::prelude::*;

mod arena;
mod models;
mod orderbook;
//mod python;

pub use models::{
    BookDepth, BookLevel, FillMetadata, OrderEvent, OrderType, Side, Trade,
};
pub use orderbook::OrderBook;

/*#[pymodule]
fn lobster_python_module(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<python::PythonOrderBook>()?;
    Ok(())
}*/