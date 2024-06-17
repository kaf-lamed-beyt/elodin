//! Provides definitions and traits for handling operations on tensor dimensions and data types.
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::{ConstDim, RealField};
use nalgebra::{constraint::ShapeConstraint, Const};

use crate::{
    array::ArrayDim, AddDim, BroadcastDim, BroadcastedDim, ConcatDim, ConcatManyDim, DefaultMap,
    DefaultMappedDim, DimGet, DotDim, Field, GetDim, MapDim, MulDim, TensorDim, XlaDim,
};

/// Defines a trait for dimensions supporting tensor operations, XLA compatibility, and array storage.
pub trait Dim: ArrayDim + TensorDim + XlaDim {}
impl<D: ArrayDim + TensorDim + XlaDim> Dim for D {}

/// Represents the interface for data representations in tensor operations.
pub trait Repr {
    type Inner<T, D: Dim>
    where
        T: Copy;

    /// Performs element-wise addition of two tensors, broadcasting as necessary.
    fn add<T, D1, D2>(
        left: &Self::Inner<T, D1>,
        right: &Self::Inner<T, D2>,
    ) -> Self::Inner<T, BroadcastedDim<D1, D2>>
    where
        T: Add<Output = T> + Copy,
        D1: Dim + ArrayDim,
        D2: Dim + ArrayDim,
        ShapeConstraint: BroadcastDim<D1, D2>,
        <ShapeConstraint as BroadcastDim<D1, D2>>::Output: Dim + ArrayDim;

    /// Performs element-wise subtraction of two tensors, broadcasting as necessary.
    fn sub<T, D1, D2>(
        left: &Self::Inner<T, D1>,
        right: &Self::Inner<T, D2>,
    ) -> Self::Inner<T, BroadcastedDim<D1, D2>>
    where
        T: Sub<Output = T> + Copy,
        D1: Dim + ArrayDim,
        D2: Dim + ArrayDim,
        ShapeConstraint: BroadcastDim<D1, D2>,
        <ShapeConstraint as BroadcastDim<D1, D2>>::Output: Dim + ArrayDim;

    /// Performs element-wise multiplication of two tensors, broadcasting as necessary.
    fn mul<T, D1, D2>(
        left: &Self::Inner<T, D1>,
        right: &Self::Inner<T, D2>,
    ) -> Self::Inner<T, BroadcastedDim<D1, D2>>
    where
        T: Mul<Output = T> + Copy,
        D1: Dim + ArrayDim,
        D2: Dim + ArrayDim,
        ShapeConstraint: BroadcastDim<D1, D2>,
        <ShapeConstraint as BroadcastDim<D1, D2>>::Output: Dim + ArrayDim;

    /// Performs element-wise division of two tensors, broadcasting as necessary.
    fn div<T, D1, D2>(
        left: &Self::Inner<T, D1>,
        right: &Self::Inner<T, D2>,
    ) -> Self::Inner<T, BroadcastedDim<D1, D2>>
    where
        T: Div<Output = T> + Copy,
        D1: Dim + ArrayDim,
        D2: Dim + ArrayDim,
        ShapeConstraint: BroadcastDim<D1, D2>,
        <ShapeConstraint as BroadcastDim<D1, D2>>::Output: Dim + ArrayDim;

    /// Computes the dot product of two tensors.
    fn dot<T, D1, D2>(
        left: &Self::Inner<T, D1>,
        right: &Self::Inner<T, D2>,
    ) -> Self::Inner<T, <ShapeConstraint as DotDim<D1, D2>>::Output>
    where
        T: Field + Div<Output = T> + Copy,
        D1: Dim + ArrayDim,
        D2: Dim + ArrayDim,
        ShapeConstraint: DotDim<D1, D2>,
        <ShapeConstraint as DotDim<D1, D2>>::Output: Dim + ArrayDim;

    /// Concatenates two arrays along the first dimension.
    fn concat<T1: Field, D1, D2: Dim + DefaultMap>(
        left: &Self::Inner<T1, D1>,
        right: &Self::Inner<T1, D2>,
    ) -> Self::Inner<T1, ConcatDim<D1, D2>>
    where
        DefaultMappedDim<D1>: nalgebra::DimAdd<DefaultMappedDim<D2>> + nalgebra::Dim,
        DefaultMappedDim<D2>: nalgebra::Dim,
        D2::DefaultMapDim: MapDim<D1>,
        D1::DefaultMapDim: MapDim<D2>,
        D1: Dim + DefaultMap,
        AddDim<DefaultMappedDim<D1>, DefaultMappedDim<D2>>: Dim,
        <<D2 as DefaultMap>::DefaultMapDim as MapDim<D1>>::MappedDim: nalgebra::Dim,
        ConcatDim<D1, D2>: Dim;

    /// Concatenates multiple tensors along the first dimension
    fn concat_many<T1: Field, D1, const N: usize>(
        args: [&Self::Inner<T1, D1>; N],
    ) -> Self::Inner<T1, ConcatManyDim<D1, N>>
    where
        DefaultMappedDim<D1>: nalgebra::DimMul<Const<N>> + nalgebra::Dim,
        D1::DefaultMapDim: MapDim<D1>,
        D1: Dim + DefaultMap,
        MulDim<DefaultMappedDim<D1>, Const<N>>: Dim,
        <<D1 as DefaultMap>::DefaultMapDim as MapDim<D1>>::MappedDim: nalgebra::Dim,
        ConcatManyDim<D1, N>: Dim;

    /// Retrieves a specific tensor based on an index within a dimension.
    fn get<T1: Field, D1: Dim>(
        arg: &Self::Inner<T1, D1>,
        index: usize,
    ) -> Self::Inner<T1, GetDim<D1>>
    where
        ShapeConstraint: DimGet<D1>;

    fn broadcast<D1: Dim, D2: ArrayDim + TensorDim + XlaDim, T1: Field>(
        arg: &Self::Inner<T1, D1>,
    ) -> Self::Inner<T1, BroadcastedDim<D1, D2>>
    where
        ShapeConstraint: BroadcastDim<D1, D2>,
        <ShapeConstraint as BroadcastDim<D1, D2>>::Output: ArrayDim + XlaDim;

    fn scalar_from_const<T1: Field>(value: T1) -> Self::Inner<T1, ()>;

    fn neg<T1, D1: Dim>(arg: &Self::Inner<T1, D1>) -> Self::Inner<T1, D1>
    where
        T1: Field + Neg<Output = T1>;

    fn sqrt<T1: Field + RealField, D1: Dim>(arg: &Self::Inner<T1, D1>) -> Self::Inner<T1, D1>;

    fn sin<T1: Field + RealField, D1: Dim>(arg: &Self::Inner<T1, D1>) -> Self::Inner<T1, D1>;

    fn cos<T1: Field + RealField, D1: Dim>(arg: &Self::Inner<T1, D1>) -> Self::Inner<T1, D1>;

    fn copy_fixed_slice<T1: Field, D1: Dim, D2: Dim + ConstDim>(
        arg: &Self::Inner<T1, D1>,
        offsets: &[usize],
    ) -> Self::Inner<T1, D2>;

    fn reshape<T1: Field, D1: Dim, D2: Dim>(arg: &Self::Inner<T1, D1>) -> Self::Inner<T1, D2>
    where
        ShapeConstraint: BroadcastDim<D1, D2>;
}
