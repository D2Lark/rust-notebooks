use super::primitive_array::*;
use super::string_array::StringArray;
use crate::scalar::{Scalar, ScalarRef};
impl<'a> ScalarRef<'a> for &'a str {
    type ArrayType = StringArray;
    type ScalarType = String;
    fn to_owned_scalar(&self) -> Self::ScalarType {
        self.to_string()
    }
}

impl Scalar for String {
    type ArrayType = StringArray;
    type RefType<'a> = &'a str;

    fn as_scalar_ref(&self) -> &str {
        self.as_str()
    }
}
impl Scalar for i16 {
    type ArrayType = I16Array;
    type RefType<'a> = i16;
    fn as_scalar_ref(&self) -> i16 {
        *self
    }
}

impl<'a> ScalarRef<'a> for i16 {
    type ArrayType = I16Array;
    type ScalarType = i16;
    fn to_owned_scalar(&self) -> i16 {
        *self
    }
}

impl Scalar for i32 {
    type ArrayType = I32Array;
    type RefType<'a> = i32;
    fn as_scalar_ref(&self) -> i32 {
        *self
    }
}

impl<'a> ScalarRef<'a> for i32 {
    type ArrayType = I32Array;
    type ScalarType = i32;
    fn to_owned_scalar(&self) -> i32 {
        *self
    }
}
