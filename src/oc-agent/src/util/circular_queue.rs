pub struct CircularQueue<T: Clone> {
    head: usize,
    buf: Vec<Option<T>>
}

impl<T: Clone> CircularQueue<T> {
    pub fn new(size: usize) -> Self {
        let mut buf = Vec::new();
        buf.resize(size, None);
        
        Self {
            head: 0,
            buf
        }
    }

    pub fn queue(&mut self, el: T) {
        self.buf[self.head] = Some(el);
        self.head = (self.head + 1) % self.buf.len();
    }

    pub fn size(&self) -> usize {
        self.buf.len()
    }

    pub fn head(&self) -> &Option<T> {
        &self.buf[self.head]
    }

    pub fn tail(&self) -> &Option<T> {
        let tail_index = (self.head + self.buf.len() - 1) % self.buf.len();
        &self.buf[tail_index]
    }

    pub fn at_offset(&self, offset: usize) -> &Option<T> {
        let offset = (self.head + offset) % self.buf.len();
        &self.buf[offset]
    }
}

#[cfg(test)]
mod tests {
    use super::CircularQueue;

    #[test]
    fn test_queue_single() {
        let mut queue = CircularQueue::new(2);
        assert_eq!(queue.size(), 2);
        assert_eq!(queue.head(), &None);
        assert_eq!(queue.tail(), &None);

        queue.queue(10);
        assert_eq!(queue.head(), &None);
        assert_eq!(queue.tail(), &Some(10));
    }

    #[test]
    fn test_queue_multi() {
        let size = 10;
        let mut queue = CircularQueue::new(size);
        
        for i in 0..(2*size) {
            queue.queue(i);
        }

        for i in 0..size {
            assert_eq!(queue.at_offset(i), &Some(size + i));
        }
    }
}