use std::{
    sync::{
        atomic::{
            AtomicUsize,
            Ordering,
        },
        Arc,
    },
};

pub struct ArrayBuffer<T, const N: usize> {
    pub(crate) entries: [Option<T>; N],

    pub(crate) read_ptr: AtomicUsize,
    pub(crate) write_ptr: AtomicUsize,
}

impl<T, const N: usize> ArrayBuffer<T, N> {
    /// Creates an `ArrayBuffer` with a given capacity
    pub fn with_capacity() -> ArrayBuffer<T, N> {
        let entries: [Option<T>; N] = unsafe {
            let mut arr: [Option<T>; N] = std::mem::MaybeUninit::uninit().assume_init();
            for item in &mut arr[..] {
                std::ptr::write(item, None);
            }
            arr
        };

        let read_ptr = AtomicUsize::new(0);
        let write_ptr = AtomicUsize::new(0);

        ArrayBuffer {
            entries,
            read_ptr,
            write_ptr,
        }
    }

    /// Creates a `Producer` and `Consumer` from a `ArrayBuffer`.
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

pub struct Producer<T, const N: usize> {
    buffer: Arc<ArrayBuffer<T, N>>,
}

impl<T, const N: usize> Producer<T, N> {
    pub(crate) fn new(buffer: Arc<ArrayBuffer<T, N>>) -> Producer<T, N> {
        Producer { buffer }
    }

    pub fn push(&mut self, item: T) -> Option<T> {
        if self.buffer.is_full() {
            return Some(item);
        }

        // Get the current write_ptr
        let write_ptr = self.buffer.write_ptr.load(Ordering::Acquire);

        unsafe {
            let buffer = Arc::get_mut_unchecked(&mut self.buffer);
            buffer.entries[write_ptr % N] = Some(item);
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

#[cfg(test)]
mod tests {
    use super::ArrayBuffer;

    #[test]
    fn basic() {
        let buffer: ArrayBuffer<u64, 4> = ArrayBuffer::with_capacity();
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
