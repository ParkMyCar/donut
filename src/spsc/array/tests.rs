use super::buffer::ArrayBuffer;

#[test]
fn basic() {
    let buffer: ArrayBuffer<u64, 4> = ArrayBuffer::new();
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