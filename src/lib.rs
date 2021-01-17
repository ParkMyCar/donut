#![feature(get_mut_unchecked)]

mod spsc;
pub use spsc::{
    array::buffer::ArrayBuffer,
    slab::buffer::SlabBuffer,
};
