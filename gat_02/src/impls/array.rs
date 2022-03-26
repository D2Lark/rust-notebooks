use crate::impls::iterator::ArrayIterator;
use crate::traits::array_trait::*;
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
        if idx >= self.data.len() {
            None
        } else {
            let range = self.offset[idx]..self.offset[idx + 1];
            Some(unsafe { std::str::from_utf8_unchecked(&self.data[range]) })
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
