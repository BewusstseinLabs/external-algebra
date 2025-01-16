// Copyright 2024 Bewusstsein Labs

use std::{
    fmt::Debug,
    ops::{ Add, AddAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign }
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
#[derive( Clone, Copy, Debug )]
pub struct TriVector<T, const DIM: usize>( T )
where
    T: 'static + Default + Copy + Debug;

impl<T, const DIM: usize> TriVector<T, DIM>
where
    T: 'static + Copy + Default + Debug,
{
    /// Creates a new const [`Vector`].
    ///
    pub const fn new_const( src: T ) -> Self {
        Self ( src )
    }

    /// Creates a new [`Vector`].
    ///
    pub fn new( src: T ) -> Self {
        Self ( src )
    }

    /// Creates a new zero filled [`Vector`].
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
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Target = T;

    fn deref( &self ) -> &Self::Target {
        &self.0
    }
}

impl<T, const DIM: usize> DerefMut for TriVector<T, DIM>
where
    T: 'static + Copy + Default + Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn deref_mut( &mut self ) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, const DIM: usize> Default for TriVector<T, DIM>
where
    T: 'static + Copy + Default + Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn default() -> Self {
        Self ( T::default() )
    }
}

impl<T, const DIM: usize> PartialEq for TriVector<T, DIM>
where
    T: 'static + Copy + Default + Debug + PartialEq,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn eq( &self, other: &Self ) -> bool {
        self.0 == other.0
    }
}
