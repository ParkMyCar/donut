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

use crate::{
    consumer::Consumer,
    producer::Producer,
};

pub struct RingBuffer<T, const N: usize> {
    pub(crate) entries: Slab<T>,
    pub(crate) index: [usize; N],

    pub(crate) read_ptr: AtomicUsize,
    pub(crate) write_ptr: AtomicUsize,
}

impl<T, const N: usize> RingBuffer<T, N> {
    /// Creates a `RingBuffer` with a given capacity
    pub fn with_capacity() -> RingBuffer<T, N> {
        let entries = Slab::with_capacity(N);
        let index = [std::usize::MAX; N];
        let read_ptr = AtomicUsize::new(0);
        let write_ptr = AtomicUsize::new(0);

        RingBuffer {
            entries,
            index,
            read_ptr,
            write_ptr,
        }
    }

    /// Creates a `Producer` and `Consumer` from a `RingBuffer`.
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

#[cfg(test)]
mod tests {
    use super::RingBuffer;

    #[test]
    fn basic() {
        let buffer: RingBuffer<u64, 4> = RingBuffer::with_capacity();
        let (mut prod, mut cons) = buffer.split();

        prod.push(64);
        prod.push(32);
        prod.push(16);

        assert_eq!(prod.len(), 3);
        assert_eq!(cons.pop(), Some(64));
        assert_eq!(prod.len(), 2);

        prod.push(8);
        prod.push(4);

        assert_eq!(prod.len(), 4);
        assert!(prod.is_full());

        assert_eq!(prod.push(2), Some(2));

        assert_eq!(cons.pop(), Some(32));
        assert_eq!(cons.pop(), Some(16));
        assert_eq!(cons.pop(), Some(8));
        assert_eq!(cons.pop(), Some(4));
        assert_eq!(cons.pop(), None);

        prod.push(100);
        prod.push(101);

        assert_eq!(cons.pop(), Some(100));
        assert_eq!(cons.pop(), Some(101));
        assert_eq!(cons.pop(), None);
    }
}
