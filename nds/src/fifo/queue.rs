use crate::interrupt::critical_section;
use core::cell::UnsafeCell;

pub(crate) struct Queue<const N: usize> {
    buffer: [UnsafeCell<u32>; N],
    head: UnsafeCell<usize>,
    tail: UnsafeCell<usize>,
}

unsafe impl<const N: usize> Sync for Queue<N> {}

impl<const N: usize> Queue<N> {
    #[inline]
    pub(crate) const fn new() -> Self {
        const INIT: UnsafeCell<u32> = UnsafeCell::new(0);
        Self { buffer: [INIT; N], head: UnsafeCell::new(0), tail: UnsafeCell::new(0) }
    }

    pub(crate) fn enqueue(&self, word: u32) -> Result<(), u32> {
        critical_section(|| {
            let head = unsafe { *self.head.get() };
            let tail = unsafe { *self.tail.get() };
            let next = (tail + 1) % N;

            if next == head {
                Err(word)
            } else {
                unsafe { self.buffer.get_unchecked(tail).get().write(word) };
                unsafe { *self.tail.get() = next };
                Ok(())
            }
        })
    }

    pub(crate) fn dequeue(&self) -> Option<u32> {
        critical_section(|| {
            let head = unsafe { *self.head.get() };
            let tail = unsafe { *self.tail.get() };

            if head == tail {
                return None;
            }

            let word = unsafe { self.buffer.get_unchecked(head).get().read() };
            unsafe { *self.head.get() = (head + 1) % N };

            Some(word)
        })
    }
}
