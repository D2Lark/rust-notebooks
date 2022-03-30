use crate::array::iterator::ArrayIterator;
use crate::array::{Array, ArrayBuilder};
use crate::scalar::{Scalar, ScalarRef};
use bitvec::prelude::BitVec;

pub trait PrimitiveType: Scalar + Default {}
impl PrimitiveType for i16 {}
impl PrimitiveType for i32 {}
pub type I16Array = PrimitiveArray<i16>;
pub type I32Array = PrimitiveArray<i32>;
pub type I16ArrayBuilder = PrimitiveArrayBuilder<i16>;
pub type I32ArrayBuilder = PrimitiveArrayBuilder<i32>;
#[derive(Clone, Debug)]
pub struct PrimitiveArray<T: PrimitiveType> {
    data: Vec<T>,
    bitmap: BitVec,
}

pub struct PrimitiveArrayBuilder<T: PrimitiveType> {
    data: Vec<T>,
    bitmap: BitVec,
}

impl<T> Array for PrimitiveArray<T>
where
    T: PrimitiveType,
    T: Scalar<ArrayType = Self>,
    for<'a> T: ScalarRef<'a, ScalarType = T, ArrayType = Self>,
    for<'a> T: Scalar<RefType<'a> = T>,
    Self: std::fmt::Debug,
{
    type Builder = PrimitiveArrayBuilder<T>;
    type OwnedItem = T;
    type RefItem<'a> = T;
    fn get(&self, idx: usize) -> Option<T> {
        if self.bitmap[idx] {
            Some(self.data[idx])
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

impl<T> ArrayBuilder for PrimitiveArrayBuilder<T>
where
    T: PrimitiveType,
    T: Scalar<ArrayType = PrimitiveArray<T>>,
    for<'a> T: ScalarRef<'a, ScalarType = T, ArrayType = PrimitiveArray<T>>,
    for<'a> T: Scalar<RefType<'a> = T>,
{
    type Array = PrimitiveArray<T>;
    fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            bitmap: BitVec::with_capacity(capacity),
        }
    }

    fn push(&mut self, value: Option<T>) {
        match value {
            Some(v) => {
                self.data.push(v);
                self.bitmap.push(true);
            }
            None => {
                self.data.push(T::default());
                self.bitmap.push(false);
            }
        }
    }
    fn finish(self) -> Self::Array {
        PrimitiveArray {
            data: self.data,
            bitmap: self.bitmap,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_primitive_array_from_ref() {
        let mut builder = I16ArrayBuilder::with_capacity(4);
        builder.push(Some(1));
        builder.push(Some(2));
        builder.push(Some(3));
        builder.push(None);
        let array = builder.finish();
        assert_eq!(array.len(), 4);
        assert_eq!(array.get(0), Some(1));
        assert_eq!(array.get(1), Some(2));
        assert_eq!(array.get(2), Some(3));
        assert_eq!(array.get(3), None);
        let mut iter = array.iter();
        assert_eq!(iter.next().unwrap(), Some(1));
        assert_eq!(iter.next().unwrap(), Some(2));
        assert_eq!(iter.next().unwrap(), Some(3));
        assert_eq!(iter.next().unwrap(), None);
    }
}
