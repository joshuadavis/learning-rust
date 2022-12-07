
// First, we make a struct to represent the circular buffer.
// We'll give it a type parameter, so we can use it for floating point or integer samples.
#[derive(Default)]
pub struct VecCircBuf<T> {
    buf: Vec<T>,
    write_idx: usize,
}

// Next, we'll add some methods on the struct.
impl<T> VecCircBuf<T>
where
    T: Copy, // We want the elements to be copy-able types (more about that later)...
{
    // Start with a constructor.  We'll initialize the buffer with the specified capacity.
    // Self resolves to CircBuf<T>, so we don't have as many cascading changes if we decide
    // to change something about CircBuf.
    pub fn new(capacity: usize) -> Self {
        Self {
            // Vec will re-allocate itself if we exceed the capacity.  We'll allocate
            // it here, and the other methods will make sure it never grows.
            buf: Vec::<T>::with_capacity(capacity),
            write_idx: 0,
        }
    }

    pub fn capacity(&self) -> usize {
        self.buf.capacity()
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }

    pub fn add(&mut self, sample: T) {
        // If the buffer isn't full yet, append.
        if self.capacity() > self.len() {
            self.buf.push(sample);
        } else {
            // Otherwise, the buffer is full to capacity, write the sample into the buffer.
            self.buf[self.write_idx] = sample;
        }
        // Increment the write index for next time.
        self.write_idx += 1;
        // If the write index has gone beyond the buffer capacity, wrap around to zero.
        if self.write_idx >= self.capacity() {
            self.write_idx = 0;
        }
    }

    pub fn get(&self, k: usize) -> Option<T> {
        // If k is larger than the current size of the buffer, then return None.
        // We can't look back that far.
        if k >= self.len() {
            None
        } else {
            // Note that we need to do the "write_idx - 1" using signed subtraction, so we
            // cast to isize then subtract.
            let i = (self.write_idx as isize - 1) - k as isize;
            if i >= 0 {
                // in bounds, just return it
                Some(self.buf[i as usize])
            } else {
                // needs to "wrap" to the end of the buffer...
                // 'i' is negative, so we can add it to len() to get the index
                // Note that we need to do signed addition, so we cast to isize, then add.
                Some(self.buf[(self.len() as isize + i) as usize])
            }
        }
    }

    // We don't want to expose the details of how this iterator works, so we return the general
    // Iterator trait.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        // Create two slices for the buffer:  One for the samples up to the write index, one for
        // the samples after the write index.  Also, borrow the slices using '&' so that the
        // size of the two variables is known at compile time (both variables are "&[T]").
        let first = &self.buf[..self.write_idx];
        let rest = &self.buf[self.write_idx..self.len()];
        // Now, make a chain out of them, but each part is in reverse order.
        first.iter().rev().chain(rest.iter().rev())
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Bring in the code we want to test from the outer module.

    #[test]
    fn initial_state() {
        let buf = VecCircBuf::<i16>::new(8);
        assert_eq!(0, buf.len());
        assert_eq!(8, buf.capacity());
        assert_eq!(None, buf.get(0));
    }

    #[test]
    fn when_capacity_is_zero() {
        let buf = VecCircBuf::<i16>::new(0);
        assert_eq!(0, buf.len());
        assert_eq!(0, buf.capacity());
        assert_eq!(None, buf.get(0));
    }

    #[test]
    #[should_panic]
    fn when_capacity_is_zero_panic_on_add() {
        let mut buf = VecCircBuf::<i16>::new(0);
        buf.add(1234)
    }

    #[test]
    fn add_samples() {
        let mut buf = VecCircBuf::<i32>::new(8);
        buf.add(1001);
        assert_eq!(1, buf.len());
        assert_eq!(1001, buf.get(0).unwrap());
        assert_eq!(None, buf.get(1));
        assert_eq!(None, buf.get(2));
        buf.add(1002);
        assert_eq!(2, buf.len());
        assert_eq!(1002, buf.get(0).unwrap());
        assert_eq!(1001, buf.get(1).unwrap());
        assert_eq!(None, buf.get(2));
        // Add 6 more to fill the buffer.
        for i in 0..6 {
            buf.add(1003 + i)
        }
        for i in 0..8 as usize {
            let val: i32 = 1008 - i as i32;
            assert_eq!(val, buf.get(i).unwrap())
        }
    }

    #[test]
    fn check_iterator() {
        // Simple tests, adding a few samples and expecting them back in reverse order.
        let mut buf = VecCircBuf::<i32>::new(5);
        buf.add(1);
        let mut iter = buf.iter();
        assert_eq!(1, *iter.next().expect("Nothing?"));
        assert_eq!(None, iter.next());

        let mut buf = VecCircBuf::<i32>::new(5);
        buf.add(1);
        buf.add(2);
        let mut iter = buf.iter();
        assert_eq!(2, *iter.next().expect("Nothing?"));
        assert_eq!(1, *iter.next().expect("Nothing?"));
        assert_eq!(None, iter.next());

        let mut buf = VecCircBuf::<i32>::new(5);
        buf.add(1);
        buf.add(2);
        buf.add(3);
        let mut iter = buf.iter();
        assert_eq!(3, *iter.next().expect("Nothing?"));
        assert_eq!(2, *iter.next().expect("Nothing?"));
        assert_eq!(1, *iter.next().expect("Nothing?"));
        assert_eq!(None, iter.next());

        // Now overwrite old entries.
        let mut buf = VecCircBuf::<i32>::new(5);
        let limit = 100;
        for s in 1..limit {
            buf.add(s)
        }
        let mut iter = buf.iter();
        let oldest: i32 = limit - buf.capacity() as i32;
        for expected in (oldest..limit).rev() {
            let s = *iter.next().expect("Nothing?");
            assert_eq!(expected, s, "s={} expected={}", s, expected);
        }
        assert_eq!(None, iter.next());
    }
}
