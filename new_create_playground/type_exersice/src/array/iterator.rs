use crate::array::Array;
pub struct ArrayIterator<'a, A: Array> {
    array: &'a A,
    pos: usize,
}

impl<'a, A: Array> Iterator for ArrayIterator<'a, A> {
    type Item = Option<A::RefItem<'a>>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.array.len() {
            None
        } else {
            let item = self.array.get(self.pos);
            self.pos += 1;
            Some(item)
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.array.len();
        (len - self.pos, Some(len - self.pos))
    }
}

impl<'a, A: Array> ArrayIterator<'a, A> {
    /// Create an [`ArrayIterator`] from [`Array`].
    pub fn new(array: &'a A) -> Self {
        Self { array, pos: 0 }
    }
}
