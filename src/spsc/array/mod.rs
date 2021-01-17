pub mod buffer;
mod consumer;
mod producer;

#[cfg(test)]
mod tests;

/// Creates a new fixed size Single Producer, Single Consumer ring buffer for the type `T` and of
/// size `N`. Elements are stored in an array on the stack.
///
/// ```
/// use donut::spsc;
///
/// let (mut p, mut c) = spsc::bounded::<u64, 16>();
///
/// p.push(4);
/// assert_eq!(c.pop(), Some(4));
/// ```
pub fn bounded<T, const N: usize>() -> (producer::Producer<T, N>, consumer::Consumer<T, N>) {
    buffer::ArrayBuffer::new().split()
}
