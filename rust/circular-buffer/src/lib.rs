pub struct CircularBuffer<T> {
    // We fake using T here, so the compiler does not complain that
    // "parameter `T` is never used". Delete when no longer needed.
    buffer: Vec<T>,
    capacity: usize,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let buffer = Vec::new();
        CircularBuffer { buffer, capacity }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.buffer.len() == self.capacity {
            Err(Error::FullBuffer)
        } else {
            self.buffer.push(element);
            Ok(())
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.buffer.is_empty() {
            Err(Error::EmptyBuffer)
        } else {
            Ok(self.buffer.remove(0))
        }
    }

    pub fn clear(&mut self) {
        self.buffer.clear()
    }

    pub fn overwrite(&mut self, element: T) {
        if self.buffer.len() == self.capacity {
            self.buffer.remove(0);
            self.buffer.insert(self.capacity - 1, element)
        } else {
            self.buffer.push(element)
        }
    }
}
