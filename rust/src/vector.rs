// Copyright 2024 Bewusstsein Labs

use std::ops::{ Sub, Mul };

use linear_algebra::{
    vector::Vector,
    ops::TensorProduct
};

use crate::ops::{
    InteriorProduct,
    ExteriorProduct
};

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

impl<T, const LHS_COL: usize, const RHS_COL: usize> ExteriorProduct<Vector<T, RHS_COL>> for Vector<T, LHS_COL>
where
    T: Default + std::fmt::Debug + Copy + Sub<Output = T> + Mul<Output = T>,
    [(); LHS_COL * RHS_COL]:,
    [(); RHS_COL * LHS_COL]:
{
    type Output = Vector<T, {LHS_COL * RHS_COL}>;

    fn exterior_product( self, rhs: Vector<T, RHS_COL> ) -> Self::Output {
        self.tensor_product( rhs ) - rhs.tensor_product( self )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exterior_product() {
        let lhs = Vector::<f64, 3>::new( [4.0, 5.0, 6.0] );
        let rhs = Vector::<f64, 3>::new( [4.0, 5.0, 6.0] );

        let result = lhs.exterior_product( rhs );

        assert_eq!( result, Vector::<f64, 9>::new( [ -3.0, 6.0, -3.0, 6.0, -12.0, 6.0, -3.0, 6.0, -3.0 ] ) );
    }
}
