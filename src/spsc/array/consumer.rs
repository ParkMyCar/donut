use std::sync::{
    atomic::Ordering,
    Arc,
};
use super::buffer::ArrayBuffer;

pub struct Consumer<T, const N: usize> {
    buffer: Arc<ArrayBuffer<T, N>>,
}

impl<T, const N: usize> Consumer<T, N> {
    pub(crate) fn new(buffer: Arc<ArrayBuffer<T, N>>) -> Consumer<T, N> {
        Consumer { buffer }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.buffer.is_empty() {
            return None;
        }

        // Get the current read_ptr
        let read_ptr = self.buffer.read_ptr.load(Ordering::Acquire);

        let item = unsafe {
            let buffer = Arc::get_mut_unchecked(&mut self.buffer);
            Option::take(&mut buffer.entries[read_ptr % N])
        };

        self.buffer.read_ptr.store(read_ptr + 1, Ordering::Release);
        item
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}