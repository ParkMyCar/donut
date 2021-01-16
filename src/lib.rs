#![feature(get_mut_unchecked)]

mod buffer;
pub use buffer::slab::SlabBuffer;
pub use buffer::array::ArrayBuffer;

mod consumer;
mod producer;
