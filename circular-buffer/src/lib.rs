pub struct CircularBuffer<T: Default + Clone> {
    vec: Vec<T>,
    start_index: usize,
    next_index: usize,
    pub len: usize,
}

impl<T: Default + Clone> CircularBuffer<T> {
    pub fn new(size: usize) -> CircularBuffer<T> {
        let mut vector = Vec::with_capacity(size);
        vector.resize(size, T::default());

        CircularBuffer {
            vec: vector,
            len: 0,
            start_index: 0,
            next_index: 0,
        }
    }

    pub fn push(&mut self, v: T) {
        self.vec[self.next_index] = v;
        self.len = (self.len + 1).min(self.vec.capacity());
        self.next_index = (self.next_index + 1) % self.vec.capacity();
        if self.len == self.vec.capacity() {
            self.start_index = self.next_index;
        }
    }

    pub fn to_vector(&self) -> Vec<T> {
        let mut vector = Vec::with_capacity(self.vec.len());

        let start_index: u32 = self
            .start_index
            .try_into()
            .expect("can't convert start index to u32");

        let capacity: u32 = self
            .vec
            .capacity()
            .try_into()
            .expect("can't convert capacity to u32");

        for i in 0..self.len {
            let absolute_index: u32 = i.try_into().expect("can't convert i to u32");
            let relative_index: usize = ((start_index + absolute_index) % capacity)
                .try_into()
                .expect("can't convert relative index to usize");
            vector.push(self.vec[relative_index].clone())
        }

        vector
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_overwrite() {
        let mut buf = CircularBuffer::new(3);
        buf.push("hello".to_string());
        buf.push("world".to_string());

        let vec = buf.to_vector();

        assert_eq!(2, buf.len, "unexpected buffer size");
        assert_eq!(2, vec.len(), "unexpected vector size");
        assert_eq!(vec[0], "hello".to_string(), "unexpected value at index 0");
        assert_eq!(vec[1], "world".to_string(), "unexpected value at index 1");
    }

    #[test]
    fn with_overwrite() {
        let mut buf = CircularBuffer::new(3);
        buf.push("some");
        buf.push("long");
        buf.push("sentence");

        buf.push("that");
        buf.push("overwrite");
        buf.push("the");

        buf.push("initial");
        buf.push("one");

        let vec = buf.to_vector();

        assert_eq!(3, buf.len, "unexpected buffer size");
        assert_eq!(3, vec.len(), "unexpected vector size");
        assert_eq!(vec[0], "the".to_string(), "unexpected value at index 0");
        assert_eq!(vec[1], "initial".to_string(), "unexpected value at index 1");
        assert_eq!(vec[2], "one".to_string(), "unexpected value at index 1");
    }
}
