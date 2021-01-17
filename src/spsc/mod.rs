mod array;
pub use array::bounded;

#[cfg(feature = "slab")]
pub mod slab;
