use crate::array::iterator::ArrayIterator;
use crate::array::{Array, ArrayBuilder};
use bitvec::prelude::BitVec;
#[derive(Clone, Debug)]
pub struct StringArray {
    data: Vec<u8>,
    offset: Vec<usize>,
    bitmap: BitVec,
}

impl Array for StringArray {
    type Builder = StringArrayBuilder;
    type OwnedItem = String;
    type RefItem<'a> = &'a str;
    fn get(&self, idx: usize) -> Option<&str> {
        if self.bitmap[idx] {
            let range = self.offset[idx]..self.offset[idx + 1];
            Some(unsafe { std::str::from_utf8_unchecked(&self.data[range]) })
        } else {
            None
        }
    }
    fn len(&self) -> usize {
        self.bitmap.len()
    }

    fn iter(&self) -> ArrayIterator<Self> {
        ArrayIterator::new(self)
    }
}
pub struct StringArrayBuilder {
    data: Vec<u8>,
    offset: Vec<usize>,
    bitmap: BitVec,
}

impl ArrayBuilder for StringArrayBuilder {
    type Array = StringArray;
    fn with_capacity(capacity: usize) -> Self {
        let mut offset = Vec::with_capacity(capacity + 1);
        offset.push(0);
        StringArrayBuilder {
            data: Vec::new(),
            offset,
            bitmap: BitVec::new(),
        }
    }
    fn push(&mut self, value: Option<&str>) {
        match value {
            Some(value) => {
                self.data.extend(value.as_bytes());
                self.offset.push(self.data.len());
                self.bitmap.push(true);
            }
            None => {
                self.offset.push(self.data.len());
                self.bitmap.push(false);
            }
        }
    }
    fn finish(self) -> Self::Array {
        StringArray {
            data: self.data,
            offset: self.offset,
            bitmap: self.bitmap,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_string_array_from_ref() {
        let mut builder = StringArrayBuilder::with_capacity(10);
        builder.push(Some("hello"));
        builder.push(Some("world"));
        builder.push(Some("!"));
        builder.push(None);
        let array = builder.finish();
        assert_eq!(array.len(), 4);
        assert_eq!(array.get(0), Some("hello"));
        assert_eq!(array.get(1), Some("world"));
        assert_eq!(array.get(2), Some("!"));
        assert_eq!(array.get(3), None);
        let mut iter = array.iter();
        assert_eq!(iter.next().unwrap(), Some("hello"));
        assert_eq!(iter.next().unwrap(), Some("world"));
        assert_eq!(iter.next().unwrap(), Some("!"));
        assert_eq!(iter.next().unwrap(), None);
    }
}
