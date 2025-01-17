// Copyright 2024 Bewusstsein Labs

use std::{
    fmt::Debug,
    ops::{ Neg, Add, AddAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign }
};
use num::traits::Num;

use linear_algebra::vector::Vector;

use crate::{
    reverse,
    progression,
    unique_combinations,
    ops::{
        //InteriorProduct,
        ExteriorProduct
    },
};

/// A vector type of generic element and size.
///
#[derive( Clone, Copy, Debug, Default )]
pub struct TriVector<T, const DIM: usize>( T )
where
    T: 'static + Default + Copy + Debug;

impl<T, const DIM: usize> TriVector<T, DIM>
where
    T: 'static + Copy + Default + Debug,
{
    /// Creates a new const [`TriVector`].
    ///
    pub const fn new_const( src: T ) -> Self {
        Self ( src )
    }

    /// Creates a new [`TriVector`].
    ///
    pub fn new( src: T ) -> Self {
        Self ( src )
    }

    /// Creates a new zero filled [`TriVector`].
    ///
    pub fn zero() -> Self
    where
        T: Num
    {
        Self ( T::zero() )
    }
}

impl<T, const DIM: usize> Deref for TriVector<T, DIM>
where
    T: 'static + Copy + Default + Debug,
{
    type Target = T;

    fn deref( &self ) -> &Self::Target {
        &self.0
    }
}

impl<T, const DIM: usize> DerefMut for TriVector<T, DIM>
where
    T: 'static + Copy + Default + Debug,
{
    fn deref_mut( &mut self ) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, const DIM: usize> PartialEq for TriVector<T, DIM>
where
    T: 'static + Copy + Default + Debug + PartialEq,
{
    fn eq( &self, other: &Self ) -> bool {
        self.0 == other.0
    }
}

impl<T, const DIM: usize> Neg for TriVector<T, DIM>
where
    T: Default + Copy + Debug + Neg<Output = T>,
{
    type Output = Self;

    fn neg( mut self ) -> Self::Output {
        self.0 = -self.0;
        self
    }
}

impl<T, const DIM: usize> Add for TriVector<T, DIM>
where
    T: Default + Copy + Debug + Add<Output = T>,
    Self: Clone
{
    type Output = Self;

    fn add( mut self, other: Self ) -> Self::Output {
        self.0 = self.0 + other.0;
        self
    }
}

impl<T, const DIM: usize> Sub for TriVector<T, DIM>
where
    T: Default + Copy + Debug + Sub<Output = T>,
    Self: Clone
{
    type Output = Self;

    fn sub( mut self, other: Self ) -> Self::Output {
        self.0 = self.0 - other.0;
        self
    }
}

impl<T, const DIM: usize> Add<T> for TriVector<T, DIM>
where
    T: Default + Copy + Debug + Add<Output = T>,
    Self: Clone
{
    type Output = Self;

    fn add( mut self, other: T ) -> Self::Output {
        self.0 = self.0 + other;
        self
    }
}

impl<T, const DIM: usize> Sub<T> for TriVector<T, DIM>
where
    T: Default + Copy + Debug + Sub<Output = T>,
    Self: Clone
{
    type Output = Self;

    fn sub( mut self, other: T ) -> Self::Output {
        self.0 = self.0 - other;
        self
    }
}

impl<T, const DIM: usize> Mul<T> for TriVector<T, DIM>
where
    T: Default + Copy + Debug + Mul<Output = T>,
    Self: Clone
{
    type Output = Self;

    fn mul( mut self, other: T ) -> Self::Output {
        self.0 = self.0 * other;
        self
    }
}

impl<T, const DIM: usize> Div<T> for TriVector<T, DIM>
where
    T: Default + Copy + Debug + Div<Output = T>,
    Self: Clone
{
    type Output = Self;

    fn div( mut self, other: T ) -> Self::Output {
        self.0 = self.0 / other;
        self
    }
}

impl<T, const DIM: usize> AddAssign for TriVector<T, DIM>
where
    T: Default + Copy + Debug + AddAssign
{
    fn add_assign( &mut self, other: Self ) {
        self.0 += other.0;
    }
}

impl<T, const DIM: usize> SubAssign for TriVector<T, DIM>
where
    T: Default + Copy + Debug + SubAssign
{
    fn sub_assign( &mut self, other: Self ) {
        self.0 -= other.0;
    }
}

pub type TriVector3<T> = TriVector<T, 3>;
pub type TriVector4<T> = TriVector<T, 4>;
