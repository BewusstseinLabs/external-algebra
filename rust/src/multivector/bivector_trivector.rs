// Copyright 2024 Bewusstsein Labs

use std::{
    fmt::Debug,
    ops::{ Neg, Add, AddAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign }
};
use num::traits::Num;

use linear_algebra::vector::Vector;

use crate::{
    ops::{
        //InteriorProduct,
        //ExteriorProduct
        GeometricAdd,
        GeometricSub,
        GeometricProduct
    },
    traits::{
        ScalarComponent,
        ScalarComponentMut,
        VectorComponent,
        VectorComponentMut,
        BiVectorComponent,
        BiVectorComponentMut,
        TriVectorComponent,
        TriVectorComponentMut
    },
    bivector::BiVector,
    trivector::TriVector
};

// ( BiVector<T, DIM>, TriVector<T, DIM> )
// xy + xz + yz + xyz

impl<T, const DIM: usize> BiVectorComponent<T, DIM> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: 'static + Copy + Default + std::fmt::Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn bivector( &self ) -> &BiVector<T, DIM> {
        &self.0
    }
}

impl<T, const DIM: usize> BiVectorComponentMut<T, DIM> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: 'static + Copy + Default + std::fmt::Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn bivector_mut( &mut self ) -> &mut BiVector<T, DIM> {
        &mut self.0
    }
}

impl<T, const DIM: usize> TriVectorComponent<T, DIM> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: 'static + Copy + Default + std::fmt::Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn trivector( &self ) -> &TriVector<T, DIM> {
        &self.1
    }
}

impl<T, const DIM: usize> TriVectorComponentMut<T, DIM> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: 'static + Copy + Default + std::fmt::Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn trivector_mut( &mut self ) -> &mut TriVector<T, DIM> {
        &mut self.1
    }
}

/// ( BiVector<T, DIM>, TriVector<T, DIM> ) + ( T + Vector<T, DIM> )
/// ( xy + xz + yz + xyz ) + ( s + x + y + z )
/// =
///
impl<T, const DIM: usize> GeometricAdd<( T, Vector<T, DIM> )> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Add<Output = T>
{
    type Output = ( T, Vector<T, DIM> );

    fn geometric_add( self, rhs: ( T, Vector<T, DIM> ) ) -> Self::Output {
        ( self + *rhs.scalar(), *rhs.vector() )
    }
}

/// ( BiVector<T, DIM>, TriVector<T, DIM> ) - ( T + Vector<T, DIM> )
/// ( xy + xz + yz + xyz ) - ( s + x + y + z )
///
impl<T, const DIM: usize> GeometricSub<( T, Vector<T, DIM> )> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Sub<Output = T> + Neg<Output = T>
{
    type Output = ( T, Vector<T, DIM> );

    fn geometric_sub( self, rhs: ( T, Vector<T, DIM> ) ) -> Self::Output {
        ( self - *rhs.scalar(), -*rhs.vector() )
    }
}

/// ( BiVector<T, DIM>, TriVector<T, DIM> ) * ( T + Vector<T, DIM> )
/// ( xy + xz + yz + xyz ) * ( 1 + x + y + z ) = ( xy*s + xy*x + xy*y + xy*z + xz*s + xz*x + xz*y + xz*z + yz*s + yz*x + yz*y + yz*z + xyz*s + xyz*x + xyz*y + xyz*z )
///
impl<T, const DIM: usize> GeometricProduct<( T, Vector<T, DIM> )> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Mul<Output = T>
{
    type Output = ( T, Vector<T, DIM> );

    fn geometric_product( self, rhs: ( T, Vector<T, DIM> ) ) -> Self::Output {
        ( self * *rhs.scalar(), *rhs.vector() * self )
    }
}

/// ( BiVector<T, DIM>, TriVector<T, DIM> ) + ( T + Vector<T, DIM> + BiVector<T, DIM> )
/// ( xy + xz + yz + xyz ) + ( 1 + x + y + z + xy + xz + yz )
///
impl<T, const DIM: usize> GeometricAdd<( T, Vector<T, DIM>, BiVector<T, DIM> )> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Add<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM> );

    fn geometric_add( self, rhs: ( T, Vector<T, DIM>, BiVector<T, DIM> ) ) -> Self::Output {
        ( self + *rhs.scalar(), *rhs.vector(), *rhs.bivector() )
    }
}

/// ( BiVector<T, DIM>, TriVector<T, DIM> ) - ( T + Vector<T, DIM> + BiVector<T, DIM> )
/// ( xy + xz + yz + xyz ) - ( 1 + x + y + z + xy + xz + yz )
///
impl<T, const DIM: usize> GeometricSub<( T, Vector<T, DIM>, BiVector<T, DIM> )> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Sub<Output = T> + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM> );

    fn geometric_sub( self, rhs: ( T, Vector<T, DIM>, BiVector<T, DIM> ) ) -> Self::Output {
        ( self - *rhs.scalar(), -*rhs.vector(), -*rhs.bivector() )
    }
}

/// ( BiVector<T, DIM>, TriVector<T, DIM> ) * ( T + Vector<T, DIM> + BiVector<T, DIM> )
/// ( xy + xz + yz + xyz ) * ( 1 + x + y + z + xy + xz + yz )
///
impl<T, const DIM: usize> GeometricProduct<( T, Vector<T, DIM>, BiVector<T, DIM> )> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Mul<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM> );

    fn geometric_product( self, rhs: ( T, Vector<T, DIM>, BiVector<T, DIM> ) ) -> Self::Output {
        ( self * *rhs.scalar(), *rhs.vector() * self, *rhs.bivector() * self )
    }
}

/// ( BiVector<T, DIM>, TriVector<T, DIM> ) + ( T + Vector<T, DIM> + BiVector<T, DIM> + TriVector<T, DIM> )
/// ( xy + xz + yz + xyz ) + ( 1 + x + y + z + xy + xz + yz + xyz )
///
impl<T, const DIM: usize> GeometricAdd<( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Add<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_add( self, rhs: ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self + *rhs.scalar(), *rhs.vector(), *rhs.bivector(), *rhs.trivector() )
    }
}

/// ( BiVector<T, DIM>, TriVector<T, DIM> ) - ( T + Vector<T, DIM> + BiVector<T, DIM> + TriVector<T, DIM> )
/// ( xy + xz + yz + xyz ) - ( 1 + x + y + z + xy + xz + yz + xyz )
///
impl<T, const DIM: usize> GeometricSub<( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Sub<Output = T> + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_sub( self, rhs: ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self - *rhs.scalar(), -*rhs.vector(), -*rhs.bivector(), -*rhs.trivector() )
    }
}

/// ( BiVector<T, DIM>, TriVector<T, DIM> ) * ( T + Vector<T, DIM> + BiVector<T, DIM> + TriVector<T, DIM> )
/// ( xy + xz + yz + xyz ) * ( 1 + x + y + z + xy + xz + yz + xyz )
///
impl<T, const DIM: usize> GeometricProduct<( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Mul<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_product( self, rhs: ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self * *rhs.scalar(), *rhs.vector() * self, *rhs.bivector() * self, *rhs.trivector() * self )
    }
}

/// ( BiVector<T, DIM>, TriVector<T, DIM> ) + ( T + Vector<T, DIM> + TriVector<T, DIM> )
/// ( xy + xz + yz + xyz ) + ( 1 + x + y + z + xyz )
///
impl<T, const DIM: usize> GeometricAdd<( T, Vector<T, DIM>, TriVector<T, DIM> )> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Add<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, TriVector<T, DIM> );

    fn geometric_add( self, rhs: ( T, Vector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self + *rhs.scalar(), *rhs.vector(), *rhs.trivector() )
    }
}

/// ( BiVector<T, DIM>, TriVector<T, DIM> ) - ( T + Vector<T, DIM> + TriVector<T, DIM> )
/// ( xy + xz + yz + xyz ) - ( 1 + x + y + z + xyz )
///
impl<T, const DIM: usize> GeometricSub<( T, Vector<T, DIM>, TriVector<T, DIM> )> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Sub<Output = T> + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, TriVector<T, DIM> );

    fn geometric_sub( self, rhs: ( T, Vector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self - *rhs.scalar(), -*rhs.vector(), -*rhs.trivector() )
    }
}

/// ( BiVector<T, DIM>, TriVector<T, DIM> ) * ( T + Vector<T, DIM> + TriVector<T, DIM> )
/// ( xy + xz + yz + xyz ) * ( 1 + x + y + z + xyz )
///
impl<T, const DIM: usize> GeometricProduct<( T, Vector<T, DIM>, TriVector<T, DIM> )> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Mul<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, TriVector<T, DIM> );

    fn geometric_product( self, rhs: ( T, Vector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self * *rhs.scalar(), *rhs.vector() * self, *rhs.trivector() * self )
    }
}

/// ( BiVector<T, DIM>, TriVector<T, DIM> ) + ( T + BiVector<T, DIM> )
/// ( xy + xz + yz + xyz ) + ( 1 + xy + xz + yz )
///
impl<T, const DIM: usize> GeometricAdd<( T, BiVector<T, DIM> )> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Add<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, BiVector<T, DIM> );

    fn geometric_add( self, rhs: ( T, BiVector<T, DIM> ) ) -> Self::Output {
        ( self + *rhs.scalar(), *rhs.bivector() )
    }
}

/// ( BiVector<T, DIM>, TriVector<T, DIM> ) - ( T + BiVector<T, DIM> )
/// ( xy + xz + yz + xyz ) - ( 1 + xy + xz + yz )
///
impl<T, const DIM: usize> GeometricSub<( T, BiVector<T, DIM> )> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Sub<Output = T> + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, BiVector<T, DIM> );

    fn geometric_sub( self, rhs: ( T, BiVector<T, DIM> ) ) -> Self::Output {
        ( self - *rhs.scalar(), -*rhs.bivector() )
    }
}

/// ( BiVector<T, DIM>, TriVector<T, DIM> ) * ( T + BiVector<T, DIM> )
/// ( xy + xz + yz + xyz ) * ( 1 + xy + xz + yz )
///
impl<T, const DIM: usize> GeometricProduct<( T, BiVector<T, DIM> )> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Mul<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, BiVector<T, DIM> );

    fn geometric_product( self, rhs: ( T, BiVector<T, DIM> ) ) -> Self::Output {
        ( self * *rhs.scalar(), *rhs.bivector() * self )
    }
}

/// ( BiVector<T, DIM>, TriVector<T, DIM> ) + ( T + BiVector<T, DIM> + TriVector<T, DIM> )
/// ( xy + xz + yz + xyz ) + ( 1 + xy + xz + yz + xyz )
///
impl<T, const DIM: usize> GeometricAdd<( T, BiVector<T, DIM>, TriVector<T, DIM> )> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Add<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_add( self, rhs: ( T, BiVector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self + *rhs.scalar(), *rhs.bivector(), *rhs.trivector() )
    }
}

/// ( BiVector<T, DIM>, TriVector<T, DIM> ) - ( T + BiVector<T, DIM> + TriVector<T, DIM> )
/// ( xy + xz + yz + xyz ) - ( 1 + xy + xz + yz + xyz )
///
impl<T, const DIM: usize> GeometricSub<( T, BiVector<T, DIM>, TriVector<T, DIM> )> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Sub<Output = T> + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_sub( self, rhs: ( T, BiVector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self - *rhs.scalar(), -*rhs.bivector(), -*rhs.trivector() )
    }
}

/// ( BiVector<T, DIM>, TriVector<T, DIM> ) * ( T + BiVector<T, DIM> + TriVector<T, DIM> )
/// ( xy + xz + yz + xyz ) * ( 1 + xy + xz + yz + xyz )
///
impl<T, const DIM: usize> GeometricProduct<( T, BiVector<T, DIM>, TriVector<T, DIM> )> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Mul<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_product( self, rhs: ( T, BiVector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self * *rhs.scalar(), *rhs.bivector() * self, *rhs.trivector() * self )
    }
}

/// ( BiVector<T, DIM>, TriVector<T, DIM> ) + ( T + TriVector<T, DIM> )
/// ( xy + xz + yz + xyz ) + ( 1 + xyz )
///
impl<T, const DIM: usize> GeometricAdd<( T, TriVector<T, DIM> )> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Add<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, TriVector<T, DIM> );

    fn geometric_add( self, rhs: ( T, TriVector<T, DIM> ) ) -> Self::Output {
        ( self + *rhs.scalar(), *rhs.trivector() )
    }
}

/// ( BiVector<T, DIM>, TriVector<T, DIM> ) - ( T + TriVector<T, DIM> )
/// ( xy + xz + yz + xyz ) - ( 1 + xyz )
///
impl<T, const DIM: usize> GeometricSub<( T, TriVector<T, DIM> )> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Sub<Output = T> + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, TriVector<T, DIM> );

    fn geometric_sub( self, rhs: ( T, TriVector<T, DIM> ) ) -> Self::Output {
        ( self - *rhs.scalar(), -*rhs.trivector() )
    }
}

/// ( BiVector<T, DIM>, TriVector<T, DIM> ) * ( T + TriVector<T, DIM> )
/// ( xy + xz + yz + xyz ) * ( 1 + xyz )
///
impl<T, const DIM: usize> GeometricProduct<( T, TriVector<T, DIM> )> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Mul<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, TriVector<T, DIM> );

    fn geometric_product( self, rhs: ( T, TriVector<T, DIM> ) ) -> Self::Output {
        ( self * *rhs.scalar(), *rhs.trivector() * self )
    }
}

/// ( BiVector<T, DIM>, TriVector<T, DIM> ) + Vector<T, DIM>
/// ( xy + xz + yz + xyz ) + ( x + y + z )
///
impl<T, const DIM: usize> GeometricAdd<Vector<T, DIM>> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy
{
    type Output = ( T, Vector<T, DIM> );

    fn geometric_add( self, rhs: Vector<T, DIM> ) -> Self::Output {
        ( self, rhs )
    }
}

/// ( BiVector<T, DIM>, TriVector<T, DIM> ) - Vector<T, DIM>
/// ( xy + xz + yz + xyz ) - ( x + y + z )
///
impl<T, const DIM: usize> GeometricSub<Vector<T, DIM>> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Neg<Output = T>
{
    type Output = ( T, Vector<T, DIM> );

    fn geometric_sub( self, rhs: Vector<T, DIM> ) -> Self::Output {
        ( self, -rhs )
    }
}

/// ( BiVector<T, DIM>, TriVector<T, DIM> ) * Vector<T, DIM>
/// ( xy + xz + yz + xyz ) * ( x + y + z )
///
impl<T, const DIM: usize> GeometricProduct<Vector<T, DIM>> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Mul<Output = T>
{
    type Output = Vector<T, DIM>;

    fn geometric_product( self, rhs: Vector<T, DIM> ) -> Self::Output {
        rhs * self
    }
}

/// ( BiVector<T, DIM>, TriVector<T, DIM> ) + ( Vector<T, DIM> + BiVector<T, DIM> )
/// ( xy + xz + yz + xyz ) + ( x + y + z + xy + xz + yz )
///
impl<T, const DIM: usize> GeometricAdd<( Vector<T, DIM>, BiVector<T, DIM> )> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM> );

    fn geometric_add( self, rhs: ( Vector<T, DIM>, BiVector<T, DIM> ) ) -> Self::Output {
        ( self, *rhs.vector(), *rhs.bivector() )
    }
}

/// ( BiVector<T, DIM>, TriVector<T, DIM> ) - ( Vector<T, DIM> + BiVector<T, DIM> )
/// ( xy + xz + yz + xyz ) - ( x + y + z + xy + xz + yz )
///
impl<T, const DIM: usize> GeometricSub<( Vector<T, DIM>, BiVector<T, DIM> )> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM> );

    fn geometric_sub( self, rhs: ( Vector<T, DIM>, BiVector<T, DIM> ) ) -> Self::Output {
        ( self, -*rhs.vector(), -*rhs.bivector() )
    }
}

/// ( BiVector<T, DIM>, TriVector<T, DIM> ) * ( Vector<T, DIM> + BiVector<T, DIM> )
/// ( xy + xz + yz + xyz ) * ( x + y + z + xy + xz + yz )
///
impl<T, const DIM: usize> GeometricProduct<( Vector<T, DIM>, BiVector<T, DIM> )> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Mul<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( Vector<T, DIM>, BiVector<T, DIM> );

    fn geometric_product( self, rhs: ( Vector<T, DIM>, BiVector<T, DIM> ) ) -> Self::Output {
        ( *rhs.vector() * self, *rhs.bivector() * self )
    }
}

/// ( BiVector<T, DIM>, TriVector<T, DIM> ) + ( Vector<T, DIM> + BiVector<T, DIM> + TriVector<T, DIM> )
/// ( xy + xz + yz + xyz ) + ( x + y + z + xy + xz + yz + xyz )
///
impl<T, const DIM: usize> GeometricAdd<( Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_add( self, rhs: ( Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self, *rhs.vector(), *rhs.bivector(), *rhs.trivector() )
    }
}

/// ( BiVector<T, DIM>, TriVector<T, DIM> ) - ( Vector<T, DIM> + BiVector<T, DIM> + TriVector<T, DIM> )
/// ( xy + xz + yz + xyz ) - ( x + y + z + xy + xz + yz + xyz )
///
impl<T, const DIM: usize> GeometricSub<( Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_sub( self, rhs: ( Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self, -*rhs.vector(), -*rhs.bivector(), -*rhs.trivector() )
    }
}

/// ( BiVector<T, DIM>, TriVector<T, DIM> ) * ( Vector<T, DIM> + BiVector<T, DIM> + TriVector<T, DIM> )
/// ( xy + xz + yz + xyz ) * ( x + y + z + xy + xz + yz + xyz )
///
impl<T, const DIM: usize> GeometricProduct<( Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Mul<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_product( self, rhs: ( Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( *rhs.vector() * self, *rhs.bivector() * self, *rhs.trivector() * self )
    }
}

/// ( BiVector<T, DIM>, TriVector<T, DIM> ) + ( Vector<T, DIM> + TriVector<T, DIM> )
/// ( xy + xz + yz + xyz ) + ( x + y + z + xyz )
///
impl<T, const DIM: usize> GeometricAdd<( Vector<T, DIM>, TriVector<T, DIM> )> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, TriVector<T, DIM> );

    fn geometric_add( self, rhs: ( Vector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self, *rhs.vector(), *rhs.trivector() )
    }
}

/// ( BiVector<T, DIM>, TriVector<T, DIM> ) - ( Vector<T, DIM> + TriVector<T, DIM> )
/// ( xy + xz + yz + xyz ) - ( x + y + z + xyz )
///
impl<T, const DIM: usize> GeometricSub<( Vector<T, DIM>, TriVector<T, DIM> )> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, TriVector<T, DIM> );

    fn geometric_sub( self, rhs: ( Vector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self, -*rhs.vector(), -*rhs.trivector() )
    }
}

/// ( BiVector<T, DIM>, TriVector<T, DIM> ) * ( Vector<T, DIM> + TriVector<T, DIM> )
/// ( xy + xz + yz + xyz ) * ( x + y + z + xyz )
///
impl<T, const DIM: usize> GeometricProduct<( Vector<T, DIM>, TriVector<T, DIM> )> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Mul<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( Vector<T, DIM>, TriVector<T, DIM> );

    fn geometric_product( self, rhs: ( Vector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( *rhs.vector() * self, *rhs.trivector() * self )
    }
}

/// ( BiVector<T, DIM>, TriVector<T, DIM> ) + BiVector<T, DIM>
/// ( xy + xz + yz + xyz ) + ( xy + xz + yz )
///
impl<T, const DIM: usize> GeometricAdd<BiVector<T, DIM>> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, BiVector<T, DIM> );

    fn geometric_add( self, rhs: BiVector<T, DIM> ) -> Self::Output {
        ( self, rhs )
    }
}

/// ( BiVector<T, DIM>, TriVector<T, DIM> ) - BiVector<T, DIM>
/// ( xy + xz + yz + xyz ) - ( xy + xz + yz )
///
impl<T, const DIM: usize> GeometricSub<BiVector<T, DIM>> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, BiVector<T, DIM> );

    fn geometric_sub( self, rhs: BiVector<T, DIM> ) -> Self::Output {
        ( self, -rhs )
    }
}

/// ( BiVector<T, DIM>, TriVector<T, DIM> ) * BiVector<T, DIM>
/// ( xy + xz + yz + xyz ) * ( xy + xz + yz )
///
impl<T, const DIM: usize> GeometricProduct<BiVector<T, DIM>> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Mul<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = BiVector<T, DIM>;

    fn geometric_product( self, rhs: BiVector<T, DIM> ) -> Self::Output {
        rhs * self
    }
}

/// ( BiVector<T, DIM>, TriVector<T, DIM> ) + ( BiVector<T, DIM> + TriVector<T, DIM> )
/// ( xy + xz + yz + xyz ) + ( xy + xz + yz + xyz )
///
impl<T, const DIM: usize> GeometricAdd<( BiVector<T, DIM>, TriVector<T, DIM> )> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_add( self, rhs: ( BiVector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self, *rhs.bivector(), *rhs.trivector() )
    }
}

/// ( BiVector<T, DIM>, TriVector<T, DIM> ) - ( BiVector<T, DIM> + TriVector<T, DIM> )
/// ( xy + xz + yz + xyz ) - ( xy + xz + yz + xyz )
///
impl<T, const DIM: usize> GeometricSub<( BiVector<T, DIM>, TriVector<T, DIM> )> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, BiVector<T, DIM>, TriVector<T, DIM>  );

    fn geometric_sub( self, rhs: ( BiVector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self, -*rhs.bivector(), -*rhs.trivector() )
    }
}

/// ( BiVector<T, DIM>, TriVector<T, DIM> ) * ( BiVector<T, DIM> + TriVector<T, DIM> )
/// ( xy + xz + yz + xyz ) * ( xy + xz + yz + xyz )
///
impl<T, const DIM: usize> GeometricProduct<( BiVector<T, DIM>, TriVector<T, DIM> )> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Mul<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( BiVector<T, DIM>, TriVector<T, DIM>  );

    fn geometric_product( self, rhs: ( BiVector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( *rhs.bivector() * self, *rhs.trivector() * self )
    }
}

/// ( BiVector<T, DIM>, TriVector<T, DIM> ) + TriVector<T, DIM>
/// ( xy + xz + yz + xyz ) + xyz
///
impl<T, const DIM: usize> GeometricAdd<TriVector<T, DIM>> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy,
{
    type Output = ( T, TriVector<T, DIM> );

    fn geometric_add( self, rhs: TriVector<T, DIM> ) -> Self::Output {
        ( self, rhs )
    }
}

/// ( BiVector<T, DIM>, TriVector<T, DIM> ) - TriVector<T, DIM>
/// ( xy + xz + yz + xyz ) - xyz
///
impl<T, const DIM: usize> GeometricSub<TriVector<T, DIM>> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Neg<Output = T>,
{
    type Output = ( T, TriVector<T, DIM> );

    fn geometric_sub( self, rhs: TriVector<T, DIM> ) -> Self::Output {
        ( self, -rhs )
    }
}

/// ( BiVector<T, DIM>, TriVector<T, DIM> ) * TriVector<T, DIM>
/// ( xy + xz + yz + xyz ) * xyz
///
impl<T, const DIM: usize> GeometricProduct<TriVector<T, DIM>> for ( BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Mul<Output = T>,
{
    type Output = TriVector<T, DIM>;

    fn geometric_product( self, rhs: TriVector<T, DIM> ) -> Self::Output {
        rhs * self
    }
}
