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
    traits::{
        XY,
        XYMut,
        XZ,
        XZMut,
        YZ,
        YZMut
    },
    bivector::BiVector
};

/// A vector type of generic element and size.
///
#[derive( Clone, Copy, Debug )]
pub struct Rotor<T: 'static + Default + Copy + Debug, const DIM: usize>
where
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    r: T,
    b: BiVector<T, DIM>
}

impl<T, const DIM: usize> Rotor<T, DIM>
where
    T: 'static + Copy + Default + Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    /// Creates a new const [`Vector`].
    ///
    pub const fn new_const( r: T, bivector: BiVector<T, DIM> ) -> Self {
        Self {
            r,
            b: bivector
        }
    }

    /// Creates a new [`Vector`].
    ///
    pub fn new( r: T, bivector: BiVector<T, DIM> ) -> Self {
        Self {
            r,
            b: bivector
        }
    }

    /// Creates a new zero filled [`Vector`].
    ///
    pub fn zero() -> Self
    where
        T: Num
    {
        Self {
            r: T::zero(),
            b: BiVector::zero()
        }
    }

    /// Returns an iterator over the elements of the [`Vector`].
    ///
    /// The iterator yields references to the elements of the [`Vector`] in order.
    ///
    pub fn iter( &self ) -> impl Iterator<Item = &T> {
        self.b.iter()
    }

    /// Returns an iterator over mutable references to the elements of the [`Vector`].
    ///
    /// The iterator yields mutable references to the elements of the [`Vector`] in order.
    ///
    pub fn iter_mut( &mut self ) -> impl Iterator<Item = &mut T> {
        self.b.iter_mut()
    }
}

impl<T, const DIM: usize> XY<T> for Rotor<T, DIM>
where
    T: 'static + Copy + Default + Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn xy( &self ) -> &T {
        self.b.xy()
    }
}

impl<T, const DIM: usize> XYMut<T> for Rotor<T, DIM>
where
    T: 'static + Copy + Default + Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn xy_mut( &mut self ) -> &mut T {
        self.b.xy_mut()
    }
}

impl<T, const DIM: usize> XZ<T> for Rotor<T, DIM>
where
    T: 'static + Copy + Default + Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn xz( &self ) -> &T {
        self.b.xz()
    }
}

impl<T, const DIM: usize> XZMut<T> for Rotor<T, DIM>
where
    T: 'static + Copy + Default + Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn xz_mut( &mut self ) -> &mut T {
        self.b.xz_mut()
    }
}

impl<T, const DIM: usize> YZ<T> for Rotor<T, DIM>
where
    T: 'static + Copy + Default + Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn yz( &self ) -> &T {
        self.b.yz()
    }
}

impl<T, const DIM: usize> YZMut<T> for Rotor<T, DIM>
where
    T: 'static + Copy + Default + Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn yz_mut( &mut self ) -> &mut T {
        self.b.yz_mut()
    }
}

impl<T, const DIM: usize> Deref for Rotor<T, DIM>
where
    T: 'static + Copy + Default + Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Target = [T; DIM * ( DIM - 1 ) / 2];

    fn deref( &self ) -> &Self::Target {
        &self.0
    }
}

impl<T, const DIM: usize> DerefMut for Rotor<T, DIM>
where
    T: 'static + Copy + Default + Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn deref_mut( &mut self ) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, const DIM: usize> Index<usize> for Rotor<T, DIM>
where
    T: 'static + Copy + Default + Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = T;

    fn index( &self, index: usize ) -> &Self::Output {
        &self.0[ index ]
    }
}

impl<T, const DIM: usize> IndexMut<usize> for Rotor<T, DIM>
where
    T: 'static + Copy + Default + Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn index_mut( &mut self, index: usize ) -> &mut Self::Output {
        &mut self.0[ index ]
    }
}

impl<T, const DIM: usize> Default for Rotor<T, DIM>
where
    T: 'static + Copy + Default + Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn default() -> Self {
        Self ( [T::default(); DIM * ( DIM - 1 ) / 2] )
    }
}

impl<T, const DIM: usize> From<[T; DIM * ( DIM - 1 ) / 2]> for Rotor<T, DIM>
where
    T: 'static + Copy + Default + Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn from( src: [T; DIM * ( DIM - 1 ) / 2] ) -> Self {
        Self ( src )
    }
}

impl<T, const DIM: usize> PartialEq for Rotor<T, DIM>
where
    T: 'static + Copy + Default + Debug + PartialEq,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn eq( &self, other: &Self ) -> bool {
        self.0 == other.0
    }
}
