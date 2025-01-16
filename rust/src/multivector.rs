// Copyright 2024 Bewusstsein Labs

use std::{
    fmt::Debug,
    ops::{ Add, AddAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign }
};
use num::traits::Num;

use linear_algebra::vector::Vector;

use crate::{
    ops::{
        //InteriorProduct,
        //ExteriorProduct
    },
    bivector::BiVector,
    trivector::TriVector
};

/// A vector type of generic element and size.
///
#[derive( Clone, Copy, Debug )]
pub struct MultiVector<T, const DIM: usize>( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: 'static + Default + Copy + Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:;

impl<T, const DIM: usize> MultiVector<T, DIM>
where
    T: 'static + Copy + Default + Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    /// Creates a new const [`Vector`].
    ///
    pub const fn new_const( magnitude: T, vector: Vector<T, DIM>, bivector: BiVector<T, DIM>, trivector: TriVector<T, DIM> ) -> Self {
        Self ( magnitude, vector, bivector, trivector )
    }

    /// Creates a new [`Vector`].
    ///
    pub fn new( magnitude: T, vector: Vector<T, DIM>, bivector: BiVector<T, DIM>, trivector: TriVector<T, DIM> ) -> Self {
        Self ( magnitude, vector, bivector, trivector )
    }

    /// Creates a new zero filled [`Vector`].
    ///
    pub fn zero() -> Self
    where
        T: Num
    {
        Self ( T::zero(), Vector::zero(), BiVector::zero(), TriVector::zero() )
    }

    pub fn magnitude( &self ) -> &T {
        &self.0
    }

    pub fn magnitude_mut( &mut self ) -> &mut T {
        &mut self.0
    }

    pub fn vector( &self ) -> &Vector<T, DIM> {
        &self.1
    }

    pub fn vector_mut( &mut self ) -> &mut Vector<T, DIM> {
        &mut self.1
    }

    pub fn bivector( &self ) -> &BiVector<T, DIM> {
        &self.2
    }

    pub fn bivector_mut( &mut self ) -> &mut BiVector<T, DIM> {
        &mut self.2
    }

    pub fn trivector( &self ) -> &TriVector<T, DIM> {
        &self.3
    }

    pub fn trivector_mut( &mut self ) -> &mut TriVector<T, DIM> {
        &mut self.3
    }
}

impl<T, const DIM: usize> Default for MultiVector<T, DIM>
where
    T: 'static + Copy + Default + Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn default() -> Self {
        Self ( T::default(), Vector::default(), BiVector::default(), TriVector::default() )
    }
}

impl<T, const DIM: usize> PartialEq for MultiVector<T, DIM>
where
    T: 'static + Copy + Default + Debug + PartialEq,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn eq( &self, other: &Self ) -> bool {
        self.0 == other.0 &&
        self.1 == other.1 &&
        self.2 == other.2 &&
        self.3 == other.3
    }
}
