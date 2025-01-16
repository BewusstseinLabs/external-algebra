use std::{
    fmt::Debug,
    ops::Div
};
use num::traits::{ Num, Float };

use linear_algebra::{
    ops::Magnitude,
    vector::Vector
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
