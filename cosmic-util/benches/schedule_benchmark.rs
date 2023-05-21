use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cosmic_util::scheduler::CosmicSchedule;
use cosmic_util::scheduler::CosmicTime;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("run run", |b| {
        b.iter(|| {
            let cron = CosmicSchedule::parse(black_box("0 12 17 2 6")).unwrap();
            let _next_execution = cron
                .next_event(&CosmicTime::from_time_ts(black_box(1638148600)))
                .unwrap();
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
