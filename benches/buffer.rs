use criterion::{criterion_group, criterion_main, Criterion};
use donut::{
    ArrayBuffer,
    SlabBuffer,
};
use ringbuf::RingBuffer;

fn bench_donut_slab_256_x_1_000(c: &mut Criterion) {
    let buffer: SlabBuffer<u64, 256> = SlabBuffer::with_capacity();
    let (mut prod, mut cons) = buffer.split();

    c.bench_function("donut slab 256 x 1_000", |b| b.iter(|| 
        for _ in 0..1_000 {
            for i in 0..256 {
                prod.push(i);
            }

            for _ in 0..256 {
                cons.pop();
            }
        }
    ));
}

fn bench_donut_array_slab_256_x_1_000(c: &mut Criterion) {
    let buffer: ArrayBuffer<u64, 256> = ArrayBuffer::with_capacity();
    let (mut prod, mut cons) = buffer.split();

    c.bench_function("donut array 256 x 1_000", |b| b.iter(|| 
        for _ in 0..1_000 {
            for i in 0..256 {
                prod.push(i);
            }

            for _ in 0..256 {
                cons.pop();
            }
        }
    ));
}

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

criterion_group!(benches, bench_donut_slab_256_x_1_000, bench_donut_array_slab_256_x_1_000, bench_ringbuf_256_x_1_000);
criterion_main!(benches);
