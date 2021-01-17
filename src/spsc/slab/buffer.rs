use slab::Slab;
use std::{
    sync::{
        atomic::{
            AtomicUsize,
            Ordering,
        },
        Arc,
    },
};
use super::{
    consumer::Consumer,
    producer::Producer,
};

pub struct SlabBuffer<T, const N: usize> {
    pub(crate) entries: Slab<T>,
    pub(crate) index: [usize; N],

    pub(crate) read_ptr: AtomicUsize,
    pub(crate) write_ptr: AtomicUsize,
}

impl<T, const N: usize> SlabBuffer<T, N> {
    /// Creates a `SlabBuffer` with a given capacity
    pub fn with_capacity() -> SlabBuffer<T, N> {
        let entries = Slab::with_capacity(N);
        let index = [std::usize::MAX; N];
        let read_ptr = AtomicUsize::new(0);
        let write_ptr = AtomicUsize::new(0);

        SlabBuffer {
            entries,
            index,
            read_ptr,
            write_ptr,
        }
    }

    /// Creates a `Producer` and `Consumer` from a `SlabBuffer`.
    pub fn split(self) -> (Producer<T, N>, Consumer<T, N>) {
        let arc = Arc::new(self);

        (Producer::new(arc.clone()), Consumer::new(arc))
    }

    /// Returns if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        let read_ptr = self.read_ptr.load(Ordering::SeqCst);
        let write_ptr = self.write_ptr.load(Ordering::SeqCst);

        read_ptr == write_ptr
    }

    /// Returns if the buffer is full.
    pub fn is_full(&self) -> bool {
        self.len() == N
    }

    /// The length of the data in the buffer.
    pub fn len(&self) -> usize {
        let read_ptr = self.read_ptr.load(Ordering::SeqCst);
        let write_ptr = self.write_ptr.load(Ordering::SeqCst);

        write_ptr - read_ptr
    }

    /// The capacity of the buffer.
    pub const fn capacity(&self) -> usize {
        N
    }
}
