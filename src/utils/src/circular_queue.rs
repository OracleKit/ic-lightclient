pub struct CircularQueue<T> {
    next: usize,
    buf: Vec<T>,
}

impl<T> CircularQueue<T> {
    pub fn new(size: usize) -> Self {
        let buf = Vec::with_capacity(size);
        Self { next: 0, buf }
    }

    pub fn capacity(&self) -> usize {
        self.buf.capacity()
    }

    pub fn size(&self) -> usize {
        self.buf.len()
    }

    pub fn queue(&mut self, el: T) {
        if self.next == self.size() {
            self.buf.push(el);
        } else {
            self.buf[self.next] = el;
        }

        self.next = (self.next + 1) % self.capacity();
    }

    pub fn tail(&self) -> Option<&T> {
        let size = self.size();
        if size == 0 {
            return None;
        }

        let tail_index = (self.next + size - 1) % size;
        Some(&self.buf[tail_index])
    }

    pub fn head(&self) -> Option<&T> {
        let size = self.size();
        if size == 0 {
            return None;
        }
        if self.next == size {
            return Some(&self.buf[0]);
        }
        Some(&self.buf[self.next])
    }
}

#[cfg(test)]
mod tests {
    use super::CircularQueue;

    #[test]
    fn test_queue_single() {
        let mut queue = CircularQueue::new(2);
        assert_eq!(queue.size(), 0);
        assert_eq!(queue.capacity(), 2);
        assert_eq!(queue.head(), None);
        assert_eq!(queue.tail(), None);

        queue.queue(100);
        assert_eq!(queue.size(), 1);
        assert_eq!(queue.head(), Some(&100));
        assert_eq!(queue.tail(), Some(&100));

        queue.queue(200);
        assert_eq!(queue.size(), 2);
        assert_eq!(queue.head(), Some(&100));
        assert_eq!(queue.tail(), Some(&200));

        queue.queue(300);
        assert_eq!(queue.size(), 2);
        assert_eq!(queue.head(), Some(&200));
        assert_eq!(queue.tail(), Some(&300));

        queue.queue(400);
        assert_eq!(queue.size(), 2);
        assert_eq!(queue.head(), Some(&300));
        assert_eq!(queue.tail(), Some(&400));
    }
}
