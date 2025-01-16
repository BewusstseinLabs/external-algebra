// Copyright 2024 Bewusstsein Labs

use std::{
    fmt::Debug,
    ops::{ Add, AddAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign }
};
use num::traits::{ Num, Float };

use linear_algebra::{
    ops::{
        Magnitude,
        Normalize
    },
    vector::Vector
};

use crate::{
    ops::{
        //InteriorProduct,
        //ExteriorProduct
    },
    bivector::BiVector
};

/// A vector type of generic element and size.
///
#[derive( Clone, Copy, Debug )]
pub struct Rotor<T, const DIM: usize>( T, BiVector<T, DIM> )
where
    T: 'static + Default + Copy + Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:;

impl<T, const DIM: usize> Rotor<T, DIM>
where
    T: 'static + Copy + Default + Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    /// Creates a new const [`Vector`].
    ///
    pub const fn new_const( scalar: T, bivector: BiVector<T, DIM> ) -> Self {
        Self ( scalar, bivector )
    }

    /// Creates a new [`Vector`].
    ///
    pub fn new( scalar: T, bivector: BiVector<T, DIM> ) -> Self {
        Self ( scalar, bivector )
    }

    /// Creates a new zero filled [`Vector`].
    ///
    pub fn zero() -> Self
    where
        T: Num
    {
        Self ( T::zero(), BiVector::zero() )
    }

    pub fn scalar( &self ) -> &T
    where
        T: Num + Copy
    {
        &self.0
    }

    pub fn scalar_mut( &mut self ) -> &mut T
    where
        T: Num + Copy
    {
        &mut self.0
    }

    pub fn bivector( &self ) -> &BiVector<T, DIM> {
        &self.1
    }

    pub fn bivector_mut( &mut self ) -> &mut BiVector<T, DIM> {
        &mut self.1
    }
}

impl<T, const DIM: usize> Default for Rotor<T, DIM>
where
    T: 'static + Copy + Default + Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn default() -> Self {
        Self ( T::default(), BiVector::default() )
    }
}

impl<T, const DIM: usize> PartialEq for Rotor<T, DIM>
where
    T: 'static + Copy + Default + Debug + PartialEq,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn eq( &self, other: &Self ) -> bool {
        self.0 == other.0 &&
        self.1 == other.1
    }
}

impl<T, const DIM: usize> Magnitude for Rotor<T, DIM>
where
    T: Default + Copy + Debug + Div<Output = T> + Num + Float,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = T;

    fn magnitude( &self ) -> Self::Output {
        self.0.sqrt() + self.1.magnitude()
    }
}

impl<T, const DIM: usize> Normalize for Rotor<T, DIM>
where
    T: Default + Copy + Debug + Div<Output = T> + DivAssign<T> + Num + Float,
    BiVector<T, DIM>: DivAssign<T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = Rotor<T, DIM>;

    fn normalize( mut self ) -> Self::Output {
        let magnitude = self.magnitude();
        self.0 /= magnitude;
        self.1 /= magnitude;
        self
    }
}
