pub struct FixedQueue<T: Clone> {
    head: usize,
    buf: Vec<Option<T>>
}

impl<T: Clone> FixedQueue<T> {
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