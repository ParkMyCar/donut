use crate::buffer::slab::SlabBuffer;
use std::{
    sync::{
        Arc,
        atomic::Ordering,
    },
};

pub struct Consumer<T, const N: usize> {
    buffer: Arc<SlabBuffer<T, N>>,
}

impl<T, const N: usize> Consumer<T, N> {
    pub(crate) fn new(buffer: Arc<SlabBuffer<T, N>>) -> Consumer<T, N> {
        Consumer { buffer }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.buffer.is_empty() {
            return None;
        }

        // Get the current read_ptr
        let read_ptr = self.buffer.read_ptr.load(Ordering::Acquire);
        // Get the key for this slot
        let key = self.buffer.index[read_ptr % N];

        unsafe {
            let buffer = Arc::get_mut_unchecked(&mut self.buffer);

            if key == std::usize::MAX {
                None
            } else {
                let entry = buffer.entries.remove(key);
                buffer.index[read_ptr % N] = std::usize::MAX;
                buffer.read_ptr.store(read_ptr + 1, Ordering::Release);

                Some(entry)
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}
