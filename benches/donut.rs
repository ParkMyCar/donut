use criterion::{criterion_group, criterion_main, Criterion};
use donut::spsc;

mod array {
    use super::*;

    pub fn bench_donut_array_256_x_1_000(c: &mut Criterion) {
        let (mut prod, mut cons) = spsc::bounded::<u64, 256>();
    
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
}
criterion_group!(array, array::bench_donut_array_256_x_1_000);

#[cfg(feature = "slab")]
mod slab {
    use super::*;

    pub fn bench_donut_slab_256_x_1_000(c: &mut Criterion) {
        let (mut prod, mut cons) = spsc::slab::bounded::<u64, 256>();
    
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
}
#[cfg(feature = "slab")]
criterion_group!(slab, slab::bench_donut_slab_256_x_1_000);


// includes the slab related benchmarks
#[cfg(feature = "slab")]
criterion_main!(array, slab);

// does not include the slab related benchmarks
#[cfg(not(feature = "slab"))]
criterion_main!(array);
