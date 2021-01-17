use criterion::{criterion_group, criterion_main, Criterion};
use ringbuf::RingBuffer;

fn bench_ringbuf_256_x_1_000(c: &mut Criterion) {
    let buffer: RingBuffer<u64> = RingBuffer::new(256);
    let (mut prod, mut cons) = buffer.split();

    c.bench_function("ringbuf 256 x 1_000", |b| b.iter(|| 
        for _ in 0..1_000 {
            for i in 0..256 {
                prod.push(i).unwrap();
            }

            for _ in 0..256 {
                cons.pop();
            }
        }
    ));
}

criterion_group!(benches, bench_ringbuf_256_x_1_000);
criterion_main!(benches);
