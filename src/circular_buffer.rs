use std::{
    fmt::{Debug, Display},
    ops::Index,
};

#[derive(Debug, Clone)]
pub struct CircularBuffer<T> {
    pub inner: Vec<T>,
    current: usize,
}
impl<T> Default for CircularBuffer<T> {
    fn default() -> Self {
        // Rather abitrary choice, but I assume that
        // the buffer keeps track of up to the last
        // 32 logs
        Self::with_capacity(32)
    }
}
impl<T> CircularBuffer<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
            current: 0,
        }
    }
    pub fn append(&mut self, element: T) {
        if self.inner.len() == self.inner.capacity() {
            self.inner[self.current] = element;
            self.current = (self.current + 1) % self.inner.capacity();
        } else {
            self.inner.push(element);
        }
    }
    pub fn iter(&self) -> CircularBufferIterator<T> {
        CircularBufferIterator {
            buffer: self,
            remaining: self.inner.len(),
        }
    }
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }
}
impl<T> Index<usize> for CircularBuffer<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}
#[derive(Debug)]
pub struct CircularBufferIterator<'a, T> {
    buffer: &'a CircularBuffer<T>,
    remaining: usize,
}
impl<'a, T> Iterator for CircularBufferIterator<'a, T>
where
    T: std::fmt::Debug,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            None
        } else {
            let current =
                (self.buffer.current + (self.buffer.len() - self.remaining)) % (self.buffer.len());
            let result = Some(&self.buffer[current]);
            self.remaining -= 1;
            result
        }
    }
}

impl<'a, T> IntoIterator for &'a CircularBuffer<T>
where
    T: Debug,
{
    type Item = &'a T;
    type IntoIter = CircularBufferIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl<T> Display for CircularBuffer<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for elem in self.iter() {
            write!(f, " {:?} ", elem)?
        }
        writeln!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_circular_buffer() {
        let mut circular_buffer = CircularBuffer::with_capacity(5);
        circular_buffer.append(1);
        circular_buffer.append(2);
        circular_buffer.append(3);

        println!("{}, {:?}", circular_buffer, circular_buffer);

        circular_buffer.append(3);

        println!("{}, {:?}", circular_buffer, circular_buffer);

        circular_buffer.append(3);

        println!("{}, {:?}", circular_buffer, circular_buffer);

        circular_buffer.append(1);

        println!("{}, {:?}", circular_buffer, circular_buffer);
    }
}
