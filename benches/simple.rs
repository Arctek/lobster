use criterion::{criterion_group, criterion_main, Criterion};
use lobster::{OrderBook, OrderType, Side};

fn small_limit_ladder(c: &mut Criterion) {
    c.bench_function("small limit ladder", |b| {
        let mut ob = OrderBook::default();
        b.iter(|| {
            for i in 0..5_000 {
                ob.execute(OrderType::Limit {
                    id: i as u128,
                    price: 12345.0 + (i as f64) / 10.0,
                    qty: i as f64,
                    side: Side::Bid,
                });
            }
        });
    });
}

fn big_limit_ladder(c: &mut Criterion) {
    c.bench_function("big limit ladder", |b| {
        let mut ob = OrderBook::default();
        b.iter(|| {
            for i in 0..100_000 {
                ob.execute(OrderType::Limit {
                    id: i as u128,
                    price: 12345.0 + (i as f64) / 10.0,
                    qty: i as f64,
                    side: Side::Bid,
                });
            }
        });
    });
}

criterion_group!(benches, small_limit_ladder, big_limit_ladder);
criterion_main!(benches);
