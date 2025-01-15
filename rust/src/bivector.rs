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
    }
};

/// A vector type of generic element and size.
///
#[derive( Clone, Copy, Debug )]
pub struct BiVector<T: 'static + Default + Copy + Debug, const DIM: usize>( [ T; DIM * ( DIM - 1 ) / 2  ] )
where
    [(); DIM * ( DIM - 1 ) / 2 ]:;

impl<T, const DIM: usize> BiVector<T, DIM>
where
    T: 'static + Copy + Default + Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    /// Creates a new const [`Vector`].
    ///
    pub const fn new_const( src: [T; DIM * ( DIM - 1 ) / 2] ) -> Self {
        Self ( src )
    }

    /// Creates a new [`Vector`].
    ///
    pub fn new( src: [T; DIM * ( DIM - 1 ) / 2] ) -> Self {
        Self ( src )
    }

    /// Creates a new zero filled [`Vector`].
    ///
    pub fn zero() -> Self
    where
        T: Num
    {
        Self ( [T::zero(); DIM * ( DIM - 1 ) / 2] )
    }

    /// Returns an iterator over the elements of the [`Vector`].
    ///
    /// The iterator yields references to the elements of the [`Vector`] in order.
    ///
    pub fn iter( &self ) -> impl Iterator<Item = &T> {
        self.0.iter()
    }

    /// Returns an iterator over mutable references to the elements of the [`Vector`].
    ///
    /// The iterator yields mutable references to the elements of the [`Vector`] in order.
    ///
    pub fn iter_mut( &mut self ) -> impl Iterator<Item = &mut T> {
        self.0.iter_mut()
    }
}

impl<T, const DIM: usize> XY<T> for BiVector<T, DIM>
where
    T: 'static + Copy + Default + Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn xy( &self ) -> &T {
        &self.0[ 0 ]
    }
}

impl<T, const DIM: usize> XYMut<T> for BiVector<T, DIM>
where
    T: 'static + Copy + Default + Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn xy_mut( &mut self ) -> &mut T {
        &mut self.0[ 0 ]
    }
}

impl<T, const DIM: usize> XZ<T> for BiVector<T, DIM>
where
    T: 'static + Copy + Default + Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn xz( &self ) -> &T {
        &self.0[ 1 ]
    }
}

impl<T, const DIM: usize> XZMut<T> for BiVector<T, DIM>
where
    T: 'static + Copy + Default + Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn xz_mut( &mut self ) -> &mut T {
        &mut self.0[ 1 ]
    }
}

impl<T, const DIM: usize> YZ<T> for BiVector<T, DIM>
where
    T: 'static + Copy + Default + Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn yz( &self ) -> &T {
        &self.0[ 2 ]
    }
}

impl<T, const DIM: usize> YZMut<T> for BiVector<T, DIM>
where
    T: 'static + Copy + Default + Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn yz_mut( &mut self ) -> &mut T {
        &mut self.0[ 2 ]
    }
}

impl<T, const DIM: usize> Deref for BiVector<T, DIM>
where
    T: 'static + Copy + Default + Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Target = [T; DIM * ( DIM - 1 ) / 2];

    fn deref( &self ) -> &Self::Target {
        &self.0
    }
}

impl<T, const DIM: usize> DerefMut for BiVector<T, DIM>
where
    T: 'static + Copy + Default + Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn deref_mut( &mut self ) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, const DIM: usize> Index<usize> for BiVector<T, DIM>
where
    T: 'static + Copy + Default + Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = T;

    fn index( &self, index: usize ) -> &Self::Output {
        &self.0[ index ]
    }
}

impl<T, const DIM: usize> IndexMut<usize> for BiVector<T, DIM>
where
    T: 'static + Copy + Default + Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn index_mut( &mut self, index: usize ) -> &mut Self::Output {
        &mut self.0[ index ]
    }
}

impl<T, const DIM: usize> Default for BiVector<T, DIM>
where
    T: 'static + Copy + Default + Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn default() -> Self {
        Self ( [T::default(); DIM * ( DIM - 1 ) / 2] )
    }
}

impl<T, const DIM: usize> From<[T; DIM * ( DIM - 1 ) / 2]> for BiVector<T, DIM>
where
    T: 'static + Copy + Default + Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn from( src: [T; DIM * ( DIM - 1 ) / 2] ) -> Self {
        Self ( src )
    }
}

impl<T, const DIM: usize> PartialEq for BiVector<T, DIM>
where
    T: 'static + Copy + Default + Debug + PartialEq,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn eq( &self, other: &Self ) -> bool {
        self.0 == other.0
    }
}

/*
impl<T, const COL: usize> InteriorProduct for Vector<T, COL>
where
    T: Default + Debug + Copy
{
    type Output;

    fn interior_product( self, rhs: Self ) -> Self::Output {

    }
}
*/

impl<T, const COL: usize> ExteriorProduct<Vector<T, COL>> for Vector<T, COL>
where
    T: Default + std::fmt::Debug + Copy + Sub<Output = T> + Mul<Output = T>,
    [(); COL * ( COL - 1 ) / 2 ]:
{
    type Output = BiVector<T, COL>;

    fn exterior_product( self, rhs: Vector<T, COL> ) -> Self::Output {
        let combinations: [( usize, usize ); COL * ( COL - 1 ) / 2 ] = unique_combinations( progression() );
        let mut res = BiVector::<T, COL>::default();
        for ( k, ( i, j ) ) in combinations.iter().enumerate() {
            res[ k ] = self[ *i ] * rhs[ *j ] - self[ *j ] * rhs[ *i ];
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exterior_product_2() {
        let lhs = Vector::<f64, 2>::new([ 4.0, 5.0 ]);
        let rhs = Vector::<f64, 2>::new([ 8.0, 3.0 ]);

        let result = lhs.exterior_product( rhs );

        assert_eq!(
            result,
            BiVector::<f64, 2>::new([
                lhs[ 0 ] * rhs[ 1 ] - lhs[ 1 ] * rhs[ 0 ], // yz - zy
            ])
        );
    }

    #[test]
    fn exterior_product_3() {
        let lhs = Vector::<f64, 3>::new([ 4.0, 5.0, 6.0 ]);
        let rhs = Vector::<f64, 3>::new([ 8.0, 3.0, 1.0 ]);

        let result = lhs.exterior_product( rhs );

        assert_eq!(
            result,
            BiVector::<f64, 3>::new([
                lhs[ 0 ] * rhs[ 1 ] - lhs[ 1 ] * rhs[ 0 ], // xy - yx
                lhs[ 0 ] * rhs[ 2 ] - lhs[ 2 ] * rhs[ 0 ], // xz - zx
                lhs[ 1 ] * rhs[ 2 ] - lhs[ 2 ] * rhs[ 1 ], // yz - zy
            ])
        );
    }

    #[test]
    fn exterior_product_4() {
        let lhs = Vector::<f64, 4>::new([ 4.0, 5.0, 6.0, 12.0 ]);
        let rhs = Vector::<f64, 4>::new([ 8.0, 3.0, 1.0, 2.0 ]);

        let result = lhs.exterior_product( rhs );

        assert_eq!(
            result,
            BiVector::<f64, 4>::new([
                lhs[ 0 ] * rhs[ 1 ] - lhs[ 1 ] * rhs[ 0 ], // xy - yx
                lhs[ 0 ] * rhs[ 2 ] - lhs[ 2 ] * rhs[ 0 ], // xz - zx
                lhs[ 0 ] * rhs[ 3 ] - lhs[ 3 ] * rhs[ 0 ], // xw - wx
                lhs[ 1 ] * rhs[ 2 ] - lhs[ 2 ] * rhs[ 1 ], // yz - zy
                lhs[ 1 ] * rhs[ 3 ] - lhs[ 3 ] * rhs[ 1 ], // yw - wy
                lhs[ 2 ] * rhs[ 3 ] - lhs[ 3 ] * rhs[ 2 ], // zw - wz
            ])
        );
    }
}
