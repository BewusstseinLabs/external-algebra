use std::{
    fmt::Debug,
    ops::{ Neg, Sub, Mul, Div }
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
    progression,
    unique_combinations,
    ops::{
        //InteriorProduct,
        ExteriorProduct,
        GeometricAdd,
        GeometricSub,
        GeometricProduct
    },
    bivector::BiVector,
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
        let combinations: [( usize, usize ); COL * ( COL - 1 ) / 2 ] = unique_combinations( progression() );
        let mut res = BiVector::<T, COL>::default();
        for ( k, ( i, j ) ) in combinations.iter().enumerate() {
            res[ k ] = self[ *i ] * rhs[ *j ] - self[ *j ] * rhs[ *i ];
        }
        res
    }
}

impl<T, const COL: usize> GeometricProduct<Vector<T, COL>> for Vector<T, COL>
where
    T: Default + std::fmt::Debug + Copy + Sub<Output = T> + Mul<Output = T> + Num,
    Self: InnerProduct<Vector<T, COL>, Output = T> + ExteriorProduct<Vector<T, COL>, Output = BiVector<T, COL>>,
    [(); COL * ( COL - 1 ) / 2 ]:
{
    type Output = Rotor<T, COL>;

    fn geometric_product( self, rhs: Vector<T, COL> ) -> Self::Output {
        Rotor::new( self.inner_product( rhs ), self.exterior_product( rhs ) )
    }
}
