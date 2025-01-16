// Copyright 2024 Bewusstsein Labs

use std::{
    fmt::Debug,
    ops::{ Neg, Add, AddAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign }
};
use num::traits::{ Num, Float };

use linear_algebra::{
    ops::{
        Magnitude,
        //InnerProduct
    },
    //vector::Vector
};

use crate::{
    //reverse,
    //progression,
    //unique_combinations,
    ops::{
        //InteriorProduct,
        //ExteriorProduct,
        //GeometricProduct
    },
    traits::{
        XY,
        XYMut,
        XZ,
        XZMut,
        YZ,
        YZMut
    },
    //trivector::TriVector,
    //multivector::MultiVector1,
    //rotor::Rotor,
};

/// A vector type of generic element and size.
///
#[derive( Clone, Copy, Debug )]
pub struct BiVector<T, const DIM: usize>( [ T; DIM * ( DIM - 1 ) / 2 ] )
where
    T: 'static + Default + Copy + Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:;

impl<T, const DIM: usize> BiVector<T, DIM>
where
    T: 'static + Copy + Default + Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    /// Creates a new const [`BiVector`].
    ///
    pub const fn new_const( src: [T; DIM * ( DIM - 1 ) / 2] ) -> Self {
        Self ( src )
    }

    /// Creates a new [`BiVector`].
    ///
    pub fn new( src: [T; DIM * ( DIM - 1 ) / 2] ) -> Self {
        Self ( src )
    }

    /// Creates a new zero filled [`BiVector`].
    ///
    pub fn zero() -> Self
    where
        T: Num
    {
        Self ( [T::zero(); DIM * ( DIM - 1 ) / 2] )
    }

    /// Returns an iterator over the elements of the [`BiVector`].
    ///
    /// The iterator yields references to the elements of the [`BiVector`] in order.
    ///
    pub fn iter( &self ) -> impl Iterator<Item = &T> {
        self.0.iter()
    }

    /// Returns an iterator over mutable references to the elements of the [`BiVector`].
    ///
    /// The iterator yields mutable references to the elements of the [`BiVector`] in order.
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

impl<T, const DIM: usize> Neg for BiVector<T, DIM>
where
    T: Default + Copy + Debug + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = Self;

    fn neg( mut self ) -> Self::Output {
        self.iter_mut()
            .for_each( |a| *a = -*a );
        self
    }
}

impl<T, const DIM: usize> Add for BiVector<T, DIM>
where
    T: Default + Copy + Debug + Add<Output = T>,
    Self: Clone,
    [(); DIM * ( DIM - 1 ) / 2]:
{
    type Output = Self;

    fn add( mut self, other: Self ) -> Self::Output {
        self.iter_mut().zip( other.iter() )
            .for_each( |( a, &b )| *a = *a + b );
        self
    }
}

impl<T, const DIM: usize> Sub for BiVector<T, DIM>
where
    T: Default + Copy + Debug + Sub<Output = T>,
    Self: Clone,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = Self;

    fn sub( mut self, other: Self ) -> Self::Output {
        self.iter_mut().zip( other.iter() )
            .for_each( |( a, &b )| *a = *a - b );
        self
    }
}

impl<T, const DIM: usize> Add<T> for BiVector<T, DIM>
where
    T: Default + Copy + Debug + Add<Output = T>,
    Self: Clone,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = Self;

    fn add( mut self, scalar: T ) -> Self::Output {
        self.iter_mut()
            .for_each( |a| *a = *a + scalar );
        self
    }
}

impl<T, const DIM: usize> Sub<T> for BiVector<T, DIM>
where
    T: Default + Copy + Debug + Sub<Output = T>,
    Self: Clone,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = Self;

    fn sub( mut self, scalar: T ) -> Self::Output {
        self.iter_mut()
            .for_each( |a| *a = *a - scalar );
        self
    }
}

impl<T, const DIM: usize> Mul<T> for BiVector<T, DIM>
where
    T: Default + Copy + Debug + Mul<Output = T>,
    Self: Clone,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = Self;

    fn mul( mut self, scalar: T ) -> Self::Output {
        self.iter_mut()
            .for_each( |a| *a = *a * scalar );
        self
    }
}

impl<T, const DIM: usize> Div<T> for BiVector<T, DIM>
where
    T: Default + Copy + Debug + Div<Output = T>,
    Self: Clone,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = Self;

    fn div( mut self, scalar: T ) -> Self::Output {
        self.iter_mut()
            .for_each( |a| *a = *a / scalar );
        self
    }
}

impl<T, const DIM: usize> AddAssign for BiVector<T, DIM>
where
    T: Default + Copy + Debug + AddAssign,
    [(); DIM * ( DIM - 1 ) / 2]:
{
    fn add_assign( &mut self, other: Self ) {
        self.iter_mut().zip( other.iter() )
            .for_each( |( a, &b )| *a += b );
    }
}

impl<T, const DIM: usize> SubAssign for BiVector<T, DIM>
where
    T: Default + Copy + Debug + SubAssign,
    [(); DIM * ( DIM - 1 ) / 2]:
{
    fn sub_assign( &mut self, other: Self ) {
        self.iter_mut().zip( other.iter() )
            .for_each( |( a, &b )| *a -= b );
    }
}

impl<T, const DIM: usize> AddAssign<T> for BiVector<T, DIM>
where
    T: Default + Copy + Debug + AddAssign,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn add_assign( &mut self, scalar: T ) {
        self.iter_mut()
            .for_each( |a| *a += scalar );
    }
}

impl<T, const DIM: usize> SubAssign<T> for BiVector<T, DIM>
where
    T: Default + Copy + Debug + SubAssign,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn sub_assign( &mut self, scalar: T ) {
        self.iter_mut()
            .for_each( |a| *a -= scalar );
    }
}

impl<T, const DIM: usize> MulAssign<T> for BiVector<T, DIM>
where
    T: Default + Copy + Debug + MulAssign,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn mul_assign( &mut self, scalar: T ) {
        self.iter_mut()
            .for_each( |a| *a *= scalar );
    }
}

impl<T, const DIM: usize> DivAssign<T> for BiVector<T, DIM>
where
    T: Default + Copy + Debug + DivAssign,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn div_assign( &mut self, scalar: T ) {
        self.iter_mut()
            .for_each( |a| *a /= scalar );
    }
}

impl<T, const DIM: usize> Add<T> for &BiVector<T, DIM>
where
    T: Default + Copy + Debug + Add<Output = T>,
    Self: Clone,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = BiVector<T, DIM>;

    fn add( self, scalar: T ) -> Self::Output {
        let mut result = BiVector::<T, DIM>::default();
        self.iter().zip( result.iter_mut() )
            .for_each( |( &a, c )| *c = a + scalar );
        result
    }
}

impl<T, const DIM: usize> Sub<T> for &BiVector<T, DIM>
where
    T: Default + Copy + Debug + Sub<Output = T>,
    Self: Clone,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = BiVector<T, DIM>;

    fn sub( self, scalar: T ) -> Self::Output {
        let mut result = BiVector::<T, DIM>::default();
        self.iter().zip( result.iter_mut() )
            .for_each( |( &a, c )| *c = a - scalar );
        result
    }
}

impl<T, const DIM: usize> Mul<T> for &BiVector<T, DIM>
where
    T: Default + Copy + Debug + Mul<Output = T>,
    Self: Clone,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = BiVector<T, DIM>;

    fn mul( self, scalar: T ) -> Self::Output {
        let mut result = BiVector::<T, DIM>::default();
        self.iter().zip( result.iter_mut() )
            .for_each( |( &a, c )| *c = a * scalar );
        result
    }
}

impl<T, const DIM: usize> Div<T> for &BiVector<T, DIM>
where
    T: Default + Copy + Debug + Div<Output = T>,
    Self: Clone,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = BiVector<T, DIM>;

    fn div( self, scalar: T ) -> Self::Output {
        let mut result = BiVector::<T, DIM>::default();
        self.iter().zip( result.iter_mut() )
            .for_each( |( &a, c )| *c = a / scalar );
        result
    }
}

impl<T, const DIM: usize> AddAssign<T> for &mut BiVector<T, DIM>
where
    T: Default + Copy + Debug + AddAssign,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn add_assign( &mut self, scalar: T ) {
        self.iter_mut()
            .for_each( |a| *a += scalar );
    }
}

impl<T, const DIM: usize> SubAssign<T> for &mut BiVector<T, DIM>
where
    T: Default + Copy + Debug + SubAssign,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn sub_assign( &mut self, scalar: T ) {
        self.iter_mut()
            .for_each( |a| *a -= scalar );
    }
}

impl<T, const DIM: usize> MulAssign<T> for &mut BiVector<T, DIM>
where
    T: Default + Copy + Debug + MulAssign,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn mul_assign( &mut self, scalar: T ) {
        self.iter_mut()
            .for_each( |a| *a *= scalar );
    }
}

impl<T, const DIM: usize> DivAssign<T> for &mut BiVector<T, DIM>
where
    T: Default + Copy + Debug + DivAssign,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn div_assign( &mut self, scalar: T ) {
        self.iter_mut()
            .for_each( |a| *a /= scalar );
    }
}

impl<T, const DIM: usize> Magnitude for BiVector<T, DIM>
where
    T: Default + Copy + Debug + Div<Output = T> + Num + Float,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = T;

    fn magnitude( &self ) -> Self::Output {
        self.0.iter().fold( T::zero(), |acc, &x| acc + x * x ).sqrt()
    }
}

/*
impl<T, const COL: usize> GeometricProduct<BiVector<T, COL>> for BiVector<T, COL>
where
    T: Default + std::fmt::Debug + Copy + Sub<Output = T> + Mul<Output = T> + Num,
    [(); COL * ( COL - 1 ) / 2 ]:
{
    type Output = MultiVector<T, COL>;

    fn geometric_product( self, rhs: BiVector<T, COL> ) -> Self::Output {
        MultiVector::<T, COL>::new(
            self.inner_product( rhs ),
            self.cross_product( rhs ),
            self.exterior_product( rhs ),
            TriVector::zero()
        )
    }
}
*/

pub type BiVector2<T> = BiVector<T, 2>;
pub type BiVector3<T> = BiVector<T, 3>;
pub type BiVector4<T> = BiVector<T, 4>;

#[cfg(test)]
mod tests {
    use super::*;
    use linear_algebra::vector::Vector;
    use crate::ops::ExteriorProduct;

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
