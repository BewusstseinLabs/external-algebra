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
        GeometricProduct,
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

// T
// s

impl<T> ScalarComponent<T> for T {
    fn scalar( &self ) -> &T {
        &self
    }
}

impl<T> ScalarComponentMut<T> for T {
    fn scalar_mut( &mut self ) -> &mut T {
        self
    }
}

/// T + ( T + Vector<T, DIM> )  = ( T+T, Vector<T, DIM> )
/// s + ( s + x + y + z )       = ( s+s + x + y + z )
///
impl<T, const DIM: usize> GeometricAdd<( T, Vector<T, DIM> )> for T
where
    T: Default + std::fmt::Debug + Copy + Add<Output = T>
{
    type Output = ( T, Vector<T, DIM> );

    fn geometric_add( self, rhs: ( T, Vector<T, DIM> ) ) -> Self::Output {
        ( self + *rhs.scalar(), *rhs.vector() )
    }
}

/// T - ( T + Vector<T, DIM> )  = ( T-T, -Vector<T, DIM> )
/// s - ( s + x + y + z )       = ( s-s - x - y - z )
///
impl<T, const DIM: usize> GeometricSub<( T, Vector<T, DIM> )> for T
where
    T: Default + std::fmt::Debug + Copy + Sub<Output = T> + Neg<Output = T>
{
    type Output = ( T, Vector<T, DIM> );

    fn geometric_sub( self, rhs: ( T, Vector<T, DIM> ) ) -> Self::Output {
        ( self - *rhs.scalar(), -*rhs.vector() )
    }
}

/// T * ( T + Vector<T, DIM> )  = ( T*T, T*Vector<T, DIM> )
/// s * ( s + x + y + z )       = ( s*s + s*x + s*y + s*z )
///
impl<T, const DIM: usize> GeometricProduct<( T, Vector<T, DIM> )> for T
where
    T: Default + std::fmt::Debug + Copy + Mul<Output = T>
{
    type Output = ( T, Vector<T, DIM> );

    fn geometric_product( self, rhs: ( T, Vector<T, DIM> ) ) -> Self::Output {
        ( self * *rhs.scalar(), *rhs.vector() * self )
    }
}

/// T + ( T + Vector<T, DIM> + BiVector<T, DIM> )   = ( T+T, Vector<T, DIM>, BiVector<T, DIM> )
/// s + ( s + x + y + z + xy + xz + yz )            = ( s+s + x + y + z + xy + xz + yz )
///
impl<T, const DIM: usize> GeometricAdd<( T, Vector<T, DIM>, BiVector<T, DIM> )> for T
where
    T: Default + std::fmt::Debug + Copy + Add<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM> );

    fn geometric_add( self, rhs: ( T, Vector<T, DIM>, BiVector<T, DIM> ) ) -> Self::Output {
        ( self + *rhs.scalar(), *rhs.vector(), *rhs.bivector() )
    }
}

/// T - ( T + Vector<T, DIM> + BiVector<T, DIM> )   = ( T-T, -Vector<T, DIM>, -BiVector<T, DIM> )
/// s - ( s + x + y + z + xy + xz + yz )            = ( s-s - x - y - z - xy - xz - yz )
///
impl<T, const DIM: usize> GeometricSub<( T, Vector<T, DIM>, BiVector<T, DIM> )> for T
where
    T: Default + std::fmt::Debug + Copy + Sub<Output = T> + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM> );

    fn geometric_sub( self, rhs: ( T, Vector<T, DIM>, BiVector<T, DIM> ) ) -> Self::Output {
        ( self - *rhs.scalar(), -*rhs.vector(), -*rhs.bivector() )
    }
}

/// T * ( T + Vector<T, DIM> + BiVector<T, DIM> )   = ( T*T, T*Vector<T, DIM>, T*BiVector<T, DIM> )
/// s * ( s + x + y + z + xy + xz + yz )            = ( s*s + s*x + s*y + s*z + s*xy + s*xz + s*yz )
///
impl<T, const DIM: usize> GeometricProduct<( T, Vector<T, DIM>, BiVector<T, DIM> )> for T
where
    T: Default + std::fmt::Debug + Copy + Mul<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM> );

    fn geometric_product( self, rhs: ( T, Vector<T, DIM>, BiVector<T, DIM> ) ) -> Self::Output {
        ( self * *rhs.scalar(), *rhs.vector() * self, *rhs.bivector() * self )
    }
}

/// T + ( T + Vector<T, DIM> + BiVector<T, DIM> + TriVector<T, DIM> )   = ( T+T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )
/// s + ( s + x + y + z + xy + xz + yz + xyz )                          = ( s+s + x + y + z + xy + xz + yz + xyz )
///
impl<T, const DIM: usize> GeometricAdd<( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )> for T
where
    T: Default + std::fmt::Debug + Copy + Add<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_add( self, rhs: ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self + *rhs.scalar(), *rhs.vector(), *rhs.bivector(), *rhs.trivector() )
    }
}

/// T - ( T + Vector<T, DIM> + BiVector<T, DIM> + TriVector<T, DIM> )   = ( T-T, -Vector<T, DIM>, -BiVector<T, DIM>, -TriVector<T, DIM> )
/// s - ( s + x + y + z + xy + xz + yz + xyz )                          = ( s-s - x - y - z - xy - xz - yz - xyz )
///
impl<T, const DIM: usize> GeometricSub<( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )> for T
where
    T: Default + std::fmt::Debug + Copy + Sub<Output = T> + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_sub( self, rhs: ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self - *rhs.scalar(), -*rhs.vector(), -*rhs.bivector(), -*rhs.trivector() )
    }
}

/// T * ( T + Vector<T, DIM> + BiVector<T, DIM> + TriVector<T, DIM> )   = ( T*T, T*Vector<T, DIM>, T*BiVector<T, DIM>, T*TriVector<T, DIM> )
/// s * ( s + x + y + z + xy + xz + yz + xyz )                          = ( s*s + s*x + s*y + s*z + s*xy + s*xz + s*yz + s*xyz )
///
impl<T, const DIM: usize> GeometricProduct<( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )> for T
where
    T: Default + std::fmt::Debug + Copy + Mul<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_product( self, rhs: ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self * *rhs.scalar(), *rhs.vector() * self, *rhs.bivector() * self, *rhs.trivector() * self )
    }
}

/// T + ( T + Vector<T, DIM> + TriVector<T, DIM> )  = ( T+T, Vector<T, DIM>, TriVector<T, DIM> )
/// s + ( s + x + y + z + xyz )                     = ( s+s + x + y + z + xyz )
///
impl<T, const DIM: usize> GeometricAdd<( T, Vector<T, DIM>, TriVector<T, DIM> )> for T
where
    T: Default + std::fmt::Debug + Copy + Add<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, TriVector<T, DIM> );

    fn geometric_add( self, rhs: ( T, Vector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self + *rhs.scalar(), *rhs.vector(), *rhs.trivector() )
    }
}

/// T - ( T + Vector<T, DIM> + TriVector<T, DIM> )  = ( T-T, -Vector<T, DIM>, -TriVector<T, DIM> )
/// s - ( s + x + y + z + xyz )                     = ( s-s - x - y - z - xyz )
///
impl<T, const DIM: usize> GeometricSub<( T, Vector<T, DIM>, TriVector<T, DIM> )> for T
where
    T: Default + std::fmt::Debug + Copy + Sub<Output = T> + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, TriVector<T, DIM> );

    fn geometric_sub( self, rhs: ( T, Vector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self - *rhs.scalar(), -*rhs.vector(), -*rhs.trivector() )
    }
}

/// T * ( T + Vector<T, DIM> + TriVector<T, DIM> )  = ( T*T, T*Vector<T, DIM>, T*TriVector<T, DIM> )
/// s * ( s + x + y + z + xyz )                     = ( s*s + s*x + s*y + s*z + s*xyz )
///
impl<T, const DIM: usize> GeometricProduct<( T, Vector<T, DIM>, TriVector<T, DIM> )> for T
where
    T: Default + std::fmt::Debug + Copy + Mul<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, TriVector<T, DIM> );

    fn geometric_product( self, rhs: ( T, Vector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self * *rhs.scalar(), *rhs.vector() * self, *rhs.trivector() * self )
    }
}

/// T + ( T + BiVector<T, DIM> )    = ( T+T, BiVector<T, DIM> )
/// s + ( s + xy + xz + yz )        = ( s+s + xy + xz + yz )
///
impl<T, const DIM: usize> GeometricAdd<( T, BiVector<T, DIM> )> for T
where
    T: Default + std::fmt::Debug + Copy + Add<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, BiVector<T, DIM> );

    fn geometric_add( self, rhs: ( T, BiVector<T, DIM> ) ) -> Self::Output {
        ( self + *rhs.scalar(), *rhs.bivector() )
    }
}

/// T - ( T + BiVector<T, DIM> )    = ( T-T, -BiVector<T, DIM> )
/// s - ( s + xy + xz + yz )        = ( s-s - xy - xz - yz )
///
impl<T, const DIM: usize> GeometricSub<( T, BiVector<T, DIM> )> for T
where
    T: Default + std::fmt::Debug + Copy + Sub<Output = T> + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, BiVector<T, DIM> );

    fn geometric_sub( self, rhs: ( T, BiVector<T, DIM> ) ) -> Self::Output {
        ( self - *rhs.scalar(), -*rhs.bivector() )
    }
}

/// T * ( T + BiVector<T, DIM> )    = ( T*T, T*BiVector<T, DIM> )
/// s * ( s + xy + xz + yz )        = ( s*s + s*xy + s*xz + s*yz )
///
impl<T, const DIM: usize> GeometricProduct<( T, BiVector<T, DIM> )> for T
where
    T: Default + std::fmt::Debug + Copy + Mul<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, BiVector<T, DIM> );

    fn geometric_product( self, rhs: ( T, BiVector<T, DIM> ) ) -> Self::Output {
        ( self * *rhs.scalar(), *rhs.bivector() * self )
    }
}

/// T + ( T + BiVector<T, DIM> + TriVector<T, DIM> )    = ( T+T, BiVector<T, DIM>, TriVector<T, DIM> )
/// s + ( s + xy + xz + yz + xyz )                      = ( s+s + xy + xz + yz + xyz )
///
impl<T, const DIM: usize> GeometricAdd<( T, BiVector<T, DIM>, TriVector<T, DIM> )> for T
where
    T: Default + std::fmt::Debug + Copy + Add<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_add( self, rhs: ( T, BiVector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self + *rhs.scalar(), *rhs.bivector(), *rhs.trivector() )
    }
}

/// T - ( T + BiVector<T, DIM> + TriVector<T, DIM> )    = ( T-T, -BiVector<T, DIM>, -TriVector<T, DIM> )
/// s - ( s + xy + xz + yz + xyz )                      = ( s-s - xy - xz - yz - xyz )
///
impl<T, const DIM: usize> GeometricSub<( T, BiVector<T, DIM>, TriVector<T, DIM> )> for T
where
    T: Default + std::fmt::Debug + Copy + Sub<Output = T> + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_sub( self, rhs: ( T, BiVector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self - *rhs.scalar(), -*rhs.bivector(), -*rhs.trivector() )
    }
}

/// T * ( T + BiVector<T, DIM> + TriVector<T, DIM> )    = ( T*T, T*BiVector<T, DIM>, T*TriVector<T, DIM> )
/// s * ( s + xy + xz + yz + xyz )                      = ( s*s + s*xy + s*xz + s*yz + s*xyz )
///
impl<T, const DIM: usize> GeometricProduct<( T, BiVector<T, DIM>, TriVector<T, DIM> )> for T
where
    T: Default + std::fmt::Debug + Copy + Mul<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_product( self, rhs: ( T, BiVector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self * *rhs.scalar(), *rhs.bivector() * self, *rhs.trivector() * self )
    }
}

/// T + ( T + TriVector<T, DIM> )   = ( T+T, TriVector<T, DIM> )
/// s + ( s + xyz )                 = ( s+s + xyz )
///
impl<T, const DIM: usize> GeometricAdd<( T, TriVector<T, DIM> )> for T
where
    T: Default + std::fmt::Debug + Copy + Add<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, TriVector<T, DIM> );

    fn geometric_add( self, rhs: ( T, TriVector<T, DIM> ) ) -> Self::Output {
        ( self + *rhs.scalar(), *rhs.trivector() )
    }
}

/// T - ( T + TriVector<T, DIM> )   = ( T-T, -TriVector<T, DIM> )
/// s - ( s + xyz )                 = ( s-s - xyz )
///
impl<T, const DIM: usize> GeometricSub<( T, TriVector<T, DIM> )> for T
where
    T: Default + std::fmt::Debug + Copy + Sub<Output = T> + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, TriVector<T, DIM> );

    fn geometric_sub( self, rhs: ( T, TriVector<T, DIM> ) ) -> Self::Output {
        ( self - *rhs.scalar(), -*rhs.trivector() )
    }
}

/// T * ( T + TriVector<T, DIM> )   = ( T*T, T*TriVector<T, DIM> )
/// s * ( s + xyz )                 = ( s*s + s*xyz )
///
impl<T, const DIM: usize> GeometricProduct<( T, TriVector<T, DIM> )> for T
where
    T: Default + std::fmt::Debug + Copy + Mul<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, TriVector<T, DIM> );

    fn geometric_product( self, rhs: ( T, TriVector<T, DIM> ) ) -> Self::Output {
        ( self * *rhs.scalar(), *rhs.trivector() * self )
    }
}

/// T + Vector<T, DIM>  = ( T, Vector<T, DIM> )
/// s + ( x + y + z )   = ( s + x + y + z )
///
impl<T, const DIM: usize> GeometricAdd<Vector<T, DIM>> for T
where
    T: Default + std::fmt::Debug + Copy
{
    type Output = ( T, Vector<T, DIM> );

    fn geometric_add( self, rhs: Vector<T, DIM> ) -> Self::Output {
        ( self, rhs )
    }
}

/// T - Vector<T, DIM>  = ( T, -Vector<T, DIM> )
/// s - ( x + y + z )   = ( s - x - y - z )
///
impl<T, const DIM: usize> GeometricSub<Vector<T, DIM>> for T
where
    T: Default + std::fmt::Debug + Copy + Neg<Output = T>
{
    type Output = ( T, Vector<T, DIM> );

    fn geometric_sub( self, rhs: Vector<T, DIM> ) -> Self::Output {
        ( self, -rhs )
    }
}

/// T * Vector<T, DIM>  = ( T*Vector<T, DIM> )
/// s * ( x + y + z )   = ( s*x + s*y + s*z )
///
impl<T, const DIM: usize> GeometricProduct<Vector<T, DIM>> for T
where
    T: Default + std::fmt::Debug + Copy + Mul<Output = T>
{
    type Output = Vector<T, DIM>;

    fn geometric_product( self, rhs: Vector<T, DIM> ) -> Self::Output {
        rhs * self
    }
}

/// T + ( Vector<T, DIM> + BiVector<T, DIM> )   = ( T, Vector<T, DIM>, BiVector<T, DIM> )
/// s + ( x + y + z + xy + xz + yz )            = ( s + x + y + z + xy + xz + yz )
///
impl<T, const DIM: usize> GeometricAdd<( Vector<T, DIM>, BiVector<T, DIM> )> for T
where
    T: Default + std::fmt::Debug + Copy,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM> );

    fn geometric_add( self, rhs: ( Vector<T, DIM>, BiVector<T, DIM> ) ) -> Self::Output {
        ( self, *rhs.vector(), *rhs.bivector() )
    }
}

/// T - ( Vector<T, DIM> + BiVector<T, DIM> )   = ( T, -Vector<T, DIM>, -BiVector<T, DIM> )
/// s - ( x + y + z + xy + xz + yz )            = ( s - x - y - z - xy - xz - yz )
///
impl<T, const DIM: usize> GeometricSub<( Vector<T, DIM>, BiVector<T, DIM> )> for T
where
    T: Default + std::fmt::Debug + Copy + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM> );

    fn geometric_sub( self, rhs: ( Vector<T, DIM>, BiVector<T, DIM> ) ) -> Self::Output {
        ( self, -*rhs.vector(), -*rhs.bivector() )
    }
}

/// T * ( Vector<T, DIM> + BiVector<T, DIM> )   = ( T*Vector<T, DIM>, T*BiVector<T, DIM> )
/// s * ( x + y + z + xy + xz + yz )            = ( s*x + s*y + s*z + s*xy + s*xz + s*yz )
///
impl<T, const DIM: usize> GeometricProduct<( Vector<T, DIM>, BiVector<T, DIM> )> for T
where
    T: Default + std::fmt::Debug + Copy + Mul<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( Vector<T, DIM>, BiVector<T, DIM> );

    fn geometric_product( self, rhs: ( Vector<T, DIM>, BiVector<T, DIM> ) ) -> Self::Output {
        ( *rhs.vector() * self, *rhs.bivector() * self )
    }
}

/// T + ( Vector<T, DIM> + BiVector<T, DIM> + TriVector<T, DIM> )   = ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )
/// s + ( x + y + z + xy + xz + yz + xyz )                          = ( s + x + y + z + xy + xz + yz + xyz )
///
impl<T, const DIM: usize> GeometricAdd<( Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )> for T
where
    T: Default + std::fmt::Debug + Copy,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_add( self, rhs: ( Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self, *rhs.vector(), *rhs.bivector(), *rhs.trivector() )
    }
}

/// T - ( Vector<T, DIM> + BiVector<T, DIM> + TriVector<T, DIM> )   = ( T, -Vector<T, DIM>, -BiVector<T, DIM>, -TriVector<T, DIM> )
/// s - ( x + y + z + xy + xz + yz + xyz )                          = ( s - x - y - z - xy - xz - yz - xyz )
///
impl<T, const DIM: usize> GeometricSub<( Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )> for T
where
    T: Default + std::fmt::Debug + Copy + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_sub( self, rhs: ( Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self, -*rhs.vector(), -*rhs.bivector(), -*rhs.trivector() )
    }
}

/// T * ( Vector<T, DIM> + BiVector<T, DIM> + TriVector<T, DIM> )   = ( T*Vector<T, DIM>, T*BiVector<T, DIM>, T*TriVector<T, DIM> )
/// s * ( x + y + z + xy + xz + yz + xyz )                          = ( s*x + s*y + s*z + s*xy + s*xz + s*yz + s*xyz )
///
impl<T, const DIM: usize> GeometricProduct<( Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )> for T
where
    T: Default + std::fmt::Debug + Copy + Mul<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_product( self, rhs: ( Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( *rhs.vector() * self, *rhs.bivector() * self, *rhs.trivector() * self )
    }
}

/// T + ( Vector<T, DIM> + TriVector<T, DIM> )  = ( T, Vector<T, DIM>, TriVector<T, DIM> )
/// s + ( x + y + z + xyz )                     = ( s + x + y + z + xyz )
///
impl<T, const DIM: usize> GeometricAdd<( Vector<T, DIM>, TriVector<T, DIM> )> for T
where
    T: Default + std::fmt::Debug + Copy,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, TriVector<T, DIM> );

    fn geometric_add( self, rhs: ( Vector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self, *rhs.vector(), *rhs.trivector() )
    }
}

/// T - ( Vector<T, DIM> + TriVector<T, DIM> )  = ( T, -Vector<T, DIM>, -TriVector<T, DIM> )
/// s - ( x + y + z + xyz )                     = ( s - x - y - z - xyz )
///
impl<T, const DIM: usize> GeometricSub<( Vector<T, DIM>, TriVector<T, DIM> )> for T
where
    T: Default + std::fmt::Debug + Copy + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, TriVector<T, DIM> );

    fn geometric_sub( self, rhs: ( Vector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self, -*rhs.vector(), -*rhs.trivector() )
    }
}

/// T * ( Vector<T, DIM> + TriVector<T, DIM> )  = ( T*Vector<T, DIM>, T*TriVector<T, DIM> )
/// s * ( x + y + z + xyz )                     = ( s*x + s*y + s*z + s*xyz )
///
impl<T, const DIM: usize> GeometricProduct<( Vector<T, DIM>, TriVector<T, DIM> )> for T
where
    T: Default + std::fmt::Debug + Copy + Mul<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( Vector<T, DIM>, TriVector<T, DIM> );

    fn geometric_product( self, rhs: ( Vector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( *rhs.vector() * self, *rhs.trivector() * self )
    }
}

/// T + BiVector<T, DIM>    = ( T, BiVector<T, DIM> )
/// s + ( xy + xz + yz )    = ( s + xy + xz + yz )
///
impl<T, const DIM: usize> GeometricAdd<BiVector<T, DIM>> for T
where
    T: Default + std::fmt::Debug + Copy,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, BiVector<T, DIM> );

    fn geometric_add( self, rhs: BiVector<T, DIM> ) -> Self::Output {
        ( self, rhs )
    }
}

/// T - BiVector<T, DIM>    = ( T, -BiVector<T, DIM> )
/// s - ( xy + xz + yz )    = ( s - xy - xz - yz )
///
impl<T, const DIM: usize> GeometricSub<BiVector<T, DIM>> for T
where
    T: Default + std::fmt::Debug + Copy + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, BiVector<T, DIM> );

    fn geometric_sub( self, rhs: BiVector<T, DIM> ) -> Self::Output {
        ( self, -rhs )
    }
}

/// T * BiVector<T, DIM>    = ( T*BiVector<T, DIM> )
/// s * ( xy + xz + yz )    = ( s*xy + s*xz + s*yz )
///
impl<T, const DIM: usize> GeometricProduct<BiVector<T, DIM>> for T
where
    T: Default + std::fmt::Debug + Copy + Mul<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = BiVector<T, DIM>;

    fn geometric_product( self, rhs: BiVector<T, DIM> ) -> Self::Output {
        rhs * self
    }
}

/// T + ( BiVector<T, DIM> + TriVector<T, DIM> )    = ( T, BiVector<T, DIM>, TriVector<T, DIM> )
/// s + ( xy + xz + yz + xyz )                      = ( s + xy + xz + yz + xyz )
///
impl<T, const DIM: usize> GeometricAdd<( BiVector<T, DIM>, TriVector<T, DIM> )> for T
where
    T: Default + std::fmt::Debug + Copy,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_add( self, rhs: ( BiVector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self, *rhs.bivector(), *rhs.trivector() )
    }
}

/// T - ( BiVector<T, DIM> + TriVector<T, DIM> )    = ( T, -BiVector<T, DIM>, -TriVector<T, DIM> )
/// s - ( xy + xz + yz + xyz )                      = ( s - xy - xz - yz - xyz )
///
impl<T, const DIM: usize> GeometricSub<( BiVector<T, DIM>, TriVector<T, DIM> )> for T
where
    T: Default + std::fmt::Debug + Copy + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, BiVector<T, DIM>, TriVector<T, DIM>  );

    fn geometric_sub( self, rhs: ( BiVector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self, -*rhs.bivector(), -*rhs.trivector() )
    }
}

/// T * ( BiVector<T, DIM> + TriVector<T, DIM> )    = ( T*BiVector<T, DIM>, T*TriVector<T, DIM> )
/// s * ( xy + xz + yz + xyz )                      = ( s*xy + s*xz + s*yz + s*xyz )
///
impl<T, const DIM: usize> GeometricProduct<( BiVector<T, DIM>, TriVector<T, DIM> )> for T
where
    T: Default + std::fmt::Debug + Copy + Mul<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( BiVector<T, DIM>, TriVector<T, DIM>  );

    fn geometric_product( self, rhs: ( BiVector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( *rhs.bivector() * self, *rhs.trivector() * self )
    }
}

/// T + TriVector<T, DIM>   = ( T, TriVector<T, DIM> )
/// s + xyz                 = ( s + xyz )
///
impl<T, const DIM: usize> GeometricAdd<TriVector<T, DIM>> for T
where
    T: Default + std::fmt::Debug + Copy,
{
    type Output = ( T, TriVector<T, DIM> );

    fn geometric_add( self, rhs: TriVector<T, DIM> ) -> Self::Output {
        ( self, rhs )
    }
}

/// T - TriVector<T, DIM>   = ( T, -TriVector<T, DIM> )
/// s - xyz                 = ( s - xyz )
///
impl<T, const DIM: usize> GeometricSub<TriVector<T, DIM>> for T
where
    T: Default + std::fmt::Debug + Copy + Neg<Output = T>,
{
    type Output = ( T, TriVector<T, DIM> );

    fn geometric_sub( self, rhs: TriVector<T, DIM> ) -> Self::Output {
        ( self, -rhs )
    }
}

/// T * TriVector<T, DIM>   = ( T*TriVector<T, DIM> )
/// s * xyz                 = ( s*xyz )
///
impl<T, const DIM: usize> GeometricProduct<TriVector<T, DIM>> for T
where
    T: Default + std::fmt::Debug + Copy + Mul<Output = T>,
{
    type Output = TriVector<T, DIM>;

    fn geometric_product( self, rhs: TriVector<T, DIM> ) -> Self::Output {
        rhs * self
    }
}
