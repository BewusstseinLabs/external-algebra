use std::{
    fmt::Debug,
    ops::{ Neg, Sub, Mul, Div, AddAssign }
};

use num::traits::{ Num, Float };
use linear_algebra::{
    ops::{
        Magnitude,
        InnerProduct
    },
    vector::Vector
};

use crate::{
    ops::{
        //InteriorProduct,
        ExteriorProduct,
        GeometricAdd,
        GeometricSub,
        GeometricProduct
    },
    bivector::BiVector,
    trivector::TriVector,
    rotor::Rotor,
};

use crate::ops::GeometricInverse;

impl<T, const COL: usize> GeometricInverse for Vector<T, COL>
where
    T: Default + Copy + Debug + Div<Output = T> + Num + Float
{
    type Output = Vector<T, COL>;

    fn geometric_inverse( self ) -> Self::Output {
        self * ( T::one() / self.magnitude() ).powi( 2 )
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
        let mut res = BiVector::<T, COL>::default();
        let mut k = 0;
        for i in 0..( COL - 1 ) {
            for j in ( i + 1 )..COL {
                res[ k ] = self[ i ] * rhs[ j ] - self[ j ] * rhs[ i ];
                k += 1;
            }
        }
        res
    }
}

impl<T, const COL: usize> ExteriorProduct<BiVector<T, COL>> for Vector<T, COL>
where
    T: Default + std::fmt::Debug + Copy + AddAssign + Mul<Output = T>,
    [(); COL * ( COL - 1 ) / 2 ]:
{
    type Output = TriVector<T, COL>;

    fn exterior_product( self, rhs: BiVector<T, COL> ) -> Self::Output {
        rhs.exterior_product( self )
    }
}

impl<T, const COL: usize> GeometricProduct<Vector<T, COL>> for Vector<T, COL>
where
    T: Default + std::fmt::Debug + Copy + Sub<Output = T> + Mul<Output = T> + Num,
    Self: InnerProduct<Vector<T, COL>, Output = T> + ExteriorProduct<Vector<T, COL>, Output = BiVector<T, COL>>,
    [(); COL * ( COL - 1 ) / 2 ]:
{
    type Output = ( T, BiVector<T, COL> );

    fn geometric_product( self, rhs: Vector<T, COL> ) -> Self::Output {
        ( self.inner_product( rhs ), self.exterior_product( rhs ) )
    }
}
