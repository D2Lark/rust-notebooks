pub mod impls;
pub mod list;
use crate::array::Array;
pub trait Scalar: std::fmt::Debug + Clone + Send + Sync + 'static {
    type ArrayType: Array<OwnedItem = Self>;
    type RefType<'a>: ScalarRef<'a, ScalarType = Self, ArrayType = Self::ArrayType>;
    fn as_scalar_ref(&self) -> Self::RefType<'_>;
}

pub trait ScalarRef<'a>: std::fmt::Debug + Clone + Copy + Send + 'a {
    type ArrayType: Array;
    type ScalarType: Scalar<RefType<'a> = Self>;
    fn to_owned_scalar(&self) -> Self::ScalarType;
}
