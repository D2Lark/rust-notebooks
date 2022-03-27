pub mod string_array;
pub use string_array::*;
pub mod iterator;
pub use iterator::*;
pub mod scalar;
pub use scalar::*;
pub mod primitive_array;
pub use primitive_array::*;

use crate::scalar::{Scalar, ScalarRef};
pub trait Array: Send + Sync + Sized + 'static + std::fmt::Debug + Clone
where
    for<'a> Self::OwnedItem: Scalar<RefType<'a> = Self::RefItem<'a>>,
{
    type Builder: ArrayBuilder<Array = Self>;
    type OwnedItem: Scalar<ArrayType = Self>;
    type RefItem<'a>: ScalarRef<'a, ScalarType = Self::OwnedItem, ArrayType = Self>;
    fn get(&self, idx: usize) -> Option<Self::RefItem<'_>>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn iter(&self) -> ArrayIterator<Self>;
    fn from_slice(data: &[Option<Self::RefItem<'_>>]) -> Self {
        let mut builder = Self::Builder::with_capacity(data.len());
        for item in data {
            builder.push(*item);
        }
        builder.finish()
    }
}

pub trait ArrayBuilder {
    type Array: Array<Builder = Self>;
    fn with_capacity(capacity: usize) -> Self;
    fn push(&mut self, value: Option<<Self::Array as Array>::RefItem<'_>>);
    fn finish(self) -> Self::Array;
}

#[derive(Clone, Debug)]
pub enum ArrayImpl {
    Int16(I16Array),
    Int32(I32Array),
    String(StringArray),
}

#[derive(Clone, Debug)]
pub enum ArrayImplRef<'a> {
    Int16(&'a I16Array),
    Int32(&'a I32Array),
    String(&'a StringArray),
}

pub enum ArrayBuilderImpl {
    Int16(I16ArrayBuilder),
    Int32(I32ArrayBuilder),
    String(StringArrayBuilder),
}

// #[cfg(test)]
// mod tests{
//     #[test]
//     fn
// }
