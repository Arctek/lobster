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
//! conversion to a shifted unsigned 64-bit integer for referencing the BTreeMaps.
//!
//! Support has been added for python. Since python doesn't currently support complex
//! enums the python parameters and return types are slightly different.

#![warn(missing_docs, missing_debug_implementations, rustdoc::broken_intra_doc_links)]

use pyo3::prelude::*;

mod arena;
mod models;
mod orderbook;
mod python;

pub use models::{
    BookDepth, BookLevel, FillMetadata, OrderEvent, OrderType, Side, Trade,
};
pub use orderbook::OrderBook;

#[pymodule]
fn lobster(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<python::OrderBook>()?;
    m.add_class::<python::OrderType>()?;
    m.add_class::<python::Order>()?;
    m.add_class::<python::OrderEventType>()?;
    m.add_class::<python::OrderEvent>()?;
    m.add_class::<models::BookDepth>()?;
    m.add_class::<models::BookLevel>()?;
    m.add_class::<models::FillMetadata>()?;
    m.add_class::<models::Side>()?;
    m.add_class::<models::Trade>()?;

    Ok(())
}