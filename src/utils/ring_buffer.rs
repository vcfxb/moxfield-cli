pub struct RingBuffer<T> {
    buffer: Box<[T]>,
    start: usize,
    size: usize,
}

impl<T> RingBuffer<T> {
    pub fn new(size: usize, fill: T) -> Self
    where
        T: Copy,
    {
        RingBuffer {
            buffer: vec![fill; size].into_boxed_slice(),
            start: 0,
            size: 0,
        }
    }

    pub const fn capacity(&self) -> usize {
        self.buffer.len()
    }

    pub const fn len(&self) -> usize {
        self.size
    }

    pub const fn is_full(&self) -> bool {
        self.capacity() == self.len()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len() {
            return None;
        }

        let adjusted = (self.start + index) % self.capacity();

        Some(&self.buffer[adjusted])
    }

    pub fn push(&mut self, t: T) {
        let write_index = (self.start + self.size) % self.capacity();
        self.buffer[write_index] = t;

        if !self.is_full() {
            self.size += 1;
        } else {
            self.start += 1;
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        let mut index = 0;
        std::iter::from_fn(move || {
            let r = self.get(index);
            index += 1;
            r
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::ring_buffer::RingBuffer;

    #[test]
    fn test_simple() {
        let mut rb = RingBuffer::new(5, 0u8);

        for x in 0..=10 {
            rb.push(x);
        }

        assert_eq!(
            rb.iter().copied().collect::<Vec<u8>>(),
            vec![6, 7, 8, 9, 10]
        );
    }
}
