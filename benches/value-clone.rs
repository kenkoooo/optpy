use criterion::{criterion_group, criterion_main, Criterion};
use optpy_runtime::Value;

fn value_clone(c: &mut Criterion) {
    c.bench_function("clone list value", |b| {
        b.iter(|| {
            let x = Value::from(vec![]);
            for _ in 0..100000 {
                criterion::black_box(x.clone());
            }
        })
    });
    c.bench_function("clone raw list", |b| {
        b.iter(|| {
            let x = Value::from(vec![]);
            let x = match x {
                Value::List(x) => x,
                _ => unreachable!(),
            };
            for _ in 0..100000 {
                criterion::black_box(x.clone());
            }
        })
    });
}

criterion_group!(benches, value_clone);
criterion_main!(benches);
