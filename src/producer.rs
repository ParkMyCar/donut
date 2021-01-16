use crate::buffer::slab::SlabBuffer;
use std::{
    sync::{
        Arc,
        atomic::Ordering,
    },
};

pub struct Producer<T, const N: usize> {
    buffer: Arc<SlabBuffer<T, N>>,
}

impl<T, const N: usize> Producer<T, N> {
    pub(crate) fn new(buffer: Arc<SlabBuffer<T, N>>) -> Producer<T, N> {
        Producer { buffer }
    }

    pub fn push(&mut self, item: T) -> Option<T> {
        if self.buffer.is_full() {
            return Some(item);
        }

        // Get the current write_ptr
        let write_ptr = self.buffer.write_ptr.load(Ordering::Acquire);
        // Get the key for this slot
        let key = self.buffer.index[write_ptr % N];

        unsafe {
            let buffer = Arc::get_mut_unchecked(&mut self.buffer);

            if key == std::usize::MAX {
                let new_key = buffer.entries.insert(item);
                buffer.index[write_ptr % N] = new_key;
            } else {
                let entry = buffer.entries.get_unchecked_mut(key);
                *entry = item;
            };
        }

        // Update the write_ptr, indicating this element has been inserted
        self.buffer.write_ptr.store(write_ptr + 1, Ordering::Release);

        None
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Returns if the buffer is full.
    pub fn is_full(&self) -> bool {
        self.buffer.is_full()
    }

    /// The length of the data in the buffer.
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// The capacity of the buffer.
    pub const fn capacity(&self) -> usize {
        N
    }
}
