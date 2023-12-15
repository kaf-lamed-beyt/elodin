use crate::{Buffer, BufferArg, Builder, Literal, MaybeOwned, Op, ScalarDim, Tensor, ToHost};
use nalgebra::ClosedAdd;
use nalgebra::Scalar as NalgebraScalar;

use std::{marker::PhantomData, ops::Add};
use xla::{ArrayElement, NativeType};

pub type Scalar<T, P = Op> = Tensor<T, ScalarDim, P>;

impl<T: NativeType + ArrayElement> ToHost for Scalar<T, Buffer> {
    type HostTy = T;

    fn to_host(&self) -> Self::HostTy {
        let literal = self.inner.to_literal_sync().unwrap();
        literal.get_first_element().unwrap()
    }
}

impl<T: ClosedAdd + ArrayElement + NativeType> Add<T> for Scalar<T, Op> {
    type Output = Scalar<T, Op>;

    fn add(self, rhs: T) -> Self::Output {
        let rhs = self.inner.builder().c0(rhs).unwrap();
        Scalar {
            inner: (self.inner + rhs).unwrap(),
            phantom: PhantomData,
        }
    }
}

pub trait ScalarExt: Sized {
    fn literal(self) -> Scalar<Self, Literal>;
    fn constant(self, builder: &Builder) -> Scalar<Self, Op>;
}

impl<T> ScalarExt for T
where
    T: ArrayElement + Sized + NativeType,
{
    fn literal(self) -> Scalar<Self, Literal> {
        Scalar {
            inner: xla::Literal::scalar(self),
            phantom: PhantomData,
        }
    }

    fn constant(self, builder: &Builder) -> Scalar<Self, Op> {
        let inner = builder
            .inner
            .constant_r0(self)
            .expect("constant creation failed");

        Scalar {
            inner,
            phantom: PhantomData,
        }
    }
}

impl<T> BufferArg<Scalar<T, Buffer>> for T
where
    T: xla::NativeType + NalgebraScalar + ArrayElement + Copy,
{
    fn as_buffer(&self, client: &crate::Client) -> MaybeOwned<'_, xla::PjRtBuffer> {
        let inner = client
            .0
            .buffer_from_host_buffer(std::slice::from_ref(self), &[], None)
            .unwrap();
        MaybeOwned::Owned(inner)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Client, CompFn};

    use super::*;

    #[test]
    fn test_sqrt_log_opt() {
        let client = Client::cpu().unwrap();
        let comp = (|a: Scalar<f32>| a.sqrt().log()).build().unwrap();
        let exec = comp.compile(&client).unwrap();
        let out = exec.run(&client, 3.141592653589793).unwrap().to_host();
        assert_eq!(out, 0.5723649);
    }
}
