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

/// Vector<T, DIM> + ( T + Vector<T, DIM> )
/// ( x + y + z ) + ( 1 + x + y + z )
///
impl<T, const DIM: usize> GeometricAdd<( T, Vector<T, DIM> )> for Vector<T, DIM>
where
    T: Default + std::fmt::Debug + Copy + Add<Output = T>
{
    type Output = ( T, Vector<T, DIM> );

    fn geometric_add( self, rhs: ( T, Vector<T, DIM> ) ) -> Self::Output {
        ( *rhs.scalar(), self + *rhs.vector() )
    }
}

/// Vector<T, DIM> - ( T + Vector<T, DIM> )
/// ( x + y + z ) - ( 1 + x + y + z )
///
impl<T, const DIM: usize> GeometricSub<( T, Vector<T, DIM> )> for Vector<T, DIM>
where
    T: Default + std::fmt::Debug + Copy + Sub<Output = T> + Neg<Output = T>
{
    type Output = ( T, Vector<T, DIM> );

    fn geometric_sub( self, rhs: ( T, Vector<T, DIM> ) ) -> Self::Output {
        ( -*rhs.scalar(), self - *rhs.vector() )
    }
}

/// Vector<T, DIM> * ( T + Vector<T, DIM> )
/// ( ax + ay + az ) * ( bs + bx + by + bz )
/// ax( bs + bx + by + bz ) + ay( bs + bx + by + bz ) + az( bs + bx + by + bz )
/// ( ax*bs + ax*bx + ax*by + ax*bz ) + ( ay*bs + ay*bx + ay*by + ay*bz ) + ( az*bs + az*bx + az*by + az*bz )
/// ( ax*bs + abxx + abxy + abxz ) + ( ay*bs + abyx + abyy + abyz ) + ( az*bs + abzx + abzy + abzz )
/// ( ax*bs + ab + abxy + abxz ) + ( ay*bs + abyx + ab + abyz ) + ( az*bs + abzx + abzy + ab )
/// 3ab + ( ax*bs + abxy + abxz ) + ( ay*bs - abxy + abyz ) + ( az*bs - abxz - abyz )
/// 3ab + ax*bs + ay*bs + az*bs
impl<T, const DIM: usize> GeometricProduct<( T, Vector<T, DIM> )> for Vector<T, DIM>
where
    T: Default + std::fmt::Debug + Copy + Mul<Output = T>
{
    type Output = ( T, Vector<T, DIM> );

    fn geometric_product( self, rhs: ( T, Vector<T, DIM> ) ) -> Self::Output {
        ( self * *rhs.scalar(), *rhs.vector() * self )
    }
}

/// Vector<T, DIM> + ( T + Vector<T, DIM> + BiVector<T, DIM> )
/// ( x + y + z ) + ( 1 + x + y + z + xy + xz + yz )
///
impl<T, const DIM: usize> GeometricAdd<( T, Vector<T, DIM>, BiVector<T, DIM> )> for Vector<T, DIM>
where
    T: Default + std::fmt::Debug + Copy + Add<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM> );

    fn geometric_add( self, rhs: ( T, Vector<T, DIM>, BiVector<T, DIM> ) ) -> Self::Output {
        ( self + *rhs.scalar(), *rhs.vector(), *rhs.bivector() )
    }
}

/// Vector<T, DIM> - ( T + Vector<T, DIM> + BiVector<T, DIM> )
/// ( x + y + z ) - ( 1 + x + y + z + xy + xz + yz )
///
impl<T, const DIM: usize> GeometricSub<( T, Vector<T, DIM>, BiVector<T, DIM> )> for Vector<T, DIM>
where
    T: Default + std::fmt::Debug + Copy + Sub<Output = T> + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM> );

    fn geometric_sub( self, rhs: ( T, Vector<T, DIM>, BiVector<T, DIM> ) ) -> Self::Output {
        ( self - *rhs.scalar(), -*rhs.vector(), -*rhs.bivector() )
    }
}

/// Vector<T, DIM> * ( T + Vector<T, DIM> + BiVector<T, DIM> )
/// ( x + y + z ) * ( 1 + x + y + z + xy + xz + yz )
///
impl<T, const DIM: usize> GeometricProduct<( T, Vector<T, DIM>, BiVector<T, DIM> )> for Vector<T, DIM>
where
    T: Default + std::fmt::Debug + Copy + Mul<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM> );

    fn geometric_product( self, rhs: ( T, Vector<T, DIM>, BiVector<T, DIM> ) ) -> Self::Output {
        ( self * *rhs.scalar(), *rhs.vector() * self, *rhs.bivector() * self )
    }
}

/// Vector<T, DIM> + ( T + Vector<T, DIM> + BiVector<T, DIM> + TriVector<T, DIM> )
/// ( x + y + z ) + ( 1 + x + y + z + xy + xz + yz + xyz )
///
impl<T, const DIM: usize> GeometricAdd<( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )> for Vector<T, DIM>
where
    T: Default + std::fmt::Debug + Copy + Add<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_add( self, rhs: ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self + *rhs.scalar(), *rhs.vector(), *rhs.bivector(), *rhs.trivector() )
    }
}

/// Vector<T, DIM> - ( T + Vector<T, DIM> + BiVector<T, DIM> + TriVector<T, DIM> )
/// ( x + y + z ) - ( 1 + x + y + z + xy + xz + yz + xyz )
///
impl<T, const DIM: usize> GeometricSub<( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )> for Vector<T, DIM>
where
    T: Default + std::fmt::Debug + Copy + Sub<Output = T> + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_sub( self, rhs: ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self - *rhs.scalar(), -*rhs.vector(), -*rhs.bivector(), -*rhs.trivector() )
    }
}

/// Vector<T, DIM> * ( T + Vector<T, DIM> + BiVector<T, DIM> + TriVector<T, DIM> )
/// ( x + y + z ) * ( 1 + x + y + z + xy + xz + yz + xyz )
///
impl<T, const DIM: usize> GeometricProduct<( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )> for Vector<T, DIM>
where
    T: Default + std::fmt::Debug + Copy + Mul<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_product( self, rhs: ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self * *rhs.scalar(), *rhs.vector() * self, *rhs.bivector() * self, *rhs.trivector() * self )
    }
}

/// Vector<T, DIM> + ( T + Vector<T, DIM> + TriVector<T, DIM> )
/// ( x + y + z ) + ( 1 + x + y + z + xyz )
///
impl<T, const DIM: usize> GeometricAdd<( T, Vector<T, DIM>, TriVector<T, DIM> )> for Vector<T, DIM>
where
    T: Default + std::fmt::Debug + Copy + Add<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, TriVector<T, DIM> );

    fn geometric_add( self, rhs: ( T, Vector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self + *rhs.scalar(), *rhs.vector(), *rhs.trivector() )
    }
}

/// Vector<T, DIM> - ( T + Vector<T, DIM> + TriVector<T, DIM> )
/// ( x + y + z ) - ( 1 + x + y + z + xyz )
///
impl<T, const DIM: usize> GeometricSub<( T, Vector<T, DIM>, TriVector<T, DIM> )> for Vector<T, DIM>
where
    T: Default + std::fmt::Debug + Copy + Sub<Output = T> + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, TriVector<T, DIM> );

    fn geometric_sub( self, rhs: ( T, Vector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self - *rhs.scalar(), -*rhs.vector(), -*rhs.trivector() )
    }
}

/// Vector<T, DIM> * ( T + Vector<T, DIM> + TriVector<T, DIM> )
/// ( x + y + z ) * ( 1 + x + y + z + xyz )
///
impl<T, const DIM: usize> GeometricProduct<( T, Vector<T, DIM>, TriVector<T, DIM> )> for Vector<T, DIM>
where
    T: Default + std::fmt::Debug + Copy + Mul<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, TriVector<T, DIM> );

    fn geometric_product( self, rhs: ( T, Vector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self * *rhs.scalar(), *rhs.vector() * self, *rhs.trivector() * self )
    }
}

/// Vector<T, DIM> + ( T + BiVector<T, DIM> )
/// ( x + y + z ) + ( 1 + xy + xz + yz )
///
impl<T, const DIM: usize> GeometricAdd<( T, BiVector<T, DIM> )> for Vector<T, DIM>
where
    T: Default + std::fmt::Debug + Copy + Add<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, BiVector<T, DIM> );

    fn geometric_add( self, rhs: ( T, BiVector<T, DIM> ) ) -> Self::Output {
        ( self + *rhs.scalar(), *rhs.bivector() )
    }
}

/// Vector<T, DIM> - ( T + BiVector<T, DIM> )
/// ( x + y + z ) - ( 1 + xy + xz + yz )
///
impl<T, const DIM: usize> GeometricSub<( T, BiVector<T, DIM> )> for Vector<T, DIM>
where
    T: Default + std::fmt::Debug + Copy + Sub<Output = T> + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, BiVector<T, DIM> );

    fn geometric_sub( self, rhs: ( T, BiVector<T, DIM> ) ) -> Self::Output {
        ( self - *rhs.scalar(), -*rhs.bivector() )
    }
}

/// Vector<T, DIM> * ( T + BiVector<T, DIM> )
/// ( x + y + z ) * ( 1 + xy + xz + yz )
///
impl<T, const DIM: usize> GeometricProduct<( T, BiVector<T, DIM> )> for Vector<T, DIM>
where
    T: Default + std::fmt::Debug + Copy + Mul<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, BiVector<T, DIM> );

    fn geometric_product( self, rhs: ( T, BiVector<T, DIM> ) ) -> Self::Output {
        ( self * *rhs.scalar(), *rhs.bivector() * self )
    }
}

/// Vector<T, DIM> + ( T + BiVector<T, DIM> + TriVector<T, DIM> )
/// ( x + y + z ) + ( 1 + xy + xz + yz + xyz )
///
impl<T, const DIM: usize> GeometricAdd<( T, BiVector<T, DIM>, TriVector<T, DIM> )> for Vector<T, DIM>
where
    T: Default + std::fmt::Debug + Copy + Add<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_add( self, rhs: ( T, BiVector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self + *rhs.scalar(), *rhs.bivector(), *rhs.trivector() )
    }
}

/// Vector<T, DIM> - ( T + BiVector<T, DIM> + TriVector<T, DIM> )
/// ( x + y + z ) - ( 1 + xy + xz + yz + xyz )
///
impl<T, const DIM: usize> GeometricSub<( T, BiVector<T, DIM>, TriVector<T, DIM> )> for Vector<T, DIM>
where
    T: Default + std::fmt::Debug + Copy + Sub<Output = T> + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_sub( self, rhs: ( T, BiVector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self - *rhs.scalar(), -*rhs.bivector(), -*rhs.trivector() )
    }
}

/// Vector<T, DIM> * ( T + BiVector<T, DIM> + TriVector<T, DIM> )
/// ( x + y + z ) * ( 1 + xy + xz + yz + xyz )
///
impl<T, const DIM: usize> GeometricProduct<( T, BiVector<T, DIM>, TriVector<T, DIM> )> for Vector<T, DIM>
where
    T: Default + std::fmt::Debug + Copy + Mul<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_product( self, rhs: ( T, BiVector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self * *rhs.scalar(), *rhs.bivector() * self, *rhs.trivector() * self )
    }
}

/// Vector<T, DIM> + ( T + TriVector<T, DIM> )
/// ( x + y + z ) + ( 1 + xyz )
///
impl<T, const DIM: usize> GeometricAdd<( T, TriVector<T, DIM> )> for Vector<T, DIM>
where
    T: Default + std::fmt::Debug + Copy + Add<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, TriVector<T, DIM> );

    fn geometric_add( self, rhs: ( T, TriVector<T, DIM> ) ) -> Self::Output {
        ( self + *rhs.scalar(), *rhs.trivector() )
    }
}

/// Vector<T, DIM> - ( T + TriVector<T, DIM> )
/// ( x + y + z ) - ( 1 + xyz )
///
impl<T, const DIM: usize> GeometricSub<( T, TriVector<T, DIM> )> for Vector<T, DIM>
where
    T: Default + std::fmt::Debug + Copy + Sub<Output = T> + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, TriVector<T, DIM> );

    fn geometric_sub( self, rhs: ( T, TriVector<T, DIM> ) ) -> Self::Output {
        ( self - *rhs.scalar(), -*rhs.trivector() )
    }
}

/// Vector<T, DIM> * ( T + TriVector<T, DIM> )
/// ( x + y + z ) * ( 1 + xyz )
///
impl<T, const DIM: usize> GeometricProduct<( T, TriVector<T, DIM> )> for Vector<T, DIM>
where
    T: Default + std::fmt::Debug + Copy + Mul<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, TriVector<T, DIM> );

    fn geometric_product( self, rhs: ( T, TriVector<T, DIM> ) ) -> Self::Output {
        ( self * *rhs.scalar(), *rhs.trivector() * self )
    }
}

/// Vector<T, DIM> + Vector<T, DIM>
/// ( x + y + z ) + ( x + y + z )
///
impl<T, const DIM: usize> GeometricAdd<Vector<T, DIM>> for Vector<T, DIM>
where
    T: Default + std::fmt::Debug + Copy
{
    type Output = ( T, Vector<T, DIM> );

    fn geometric_add( self, rhs: Vector<T, DIM> ) -> Self::Output {
        ( self, rhs )
    }
}

/// Vector<T, DIM> - Vector<T, DIM>
/// ( x + y + z ) - ( x + y + z )
///
impl<T, const DIM: usize> GeometricSub<Vector<T, DIM>> for Vector<T, DIM>
where
    T: Default + std::fmt::Debug + Copy + Neg<Output = T>
{
    type Output = ( T, Vector<T, DIM> );

    fn geometric_sub( self, rhs: Vector<T, DIM> ) -> Self::Output {
        ( self, -rhs )
    }
}

/// Vector<T, DIM> * Vector<T, DIM>
/// ( x + y + z ) * ( x + y + z )
///
impl<T, const DIM: usize> GeometricProduct<Vector<T, DIM>> for Vector<T, DIM>
where
    T: Default + std::fmt::Debug + Copy + Mul<Output = T>
{
    type Output = Vector<T, DIM>;

    fn geometric_product( self, rhs: Vector<T, DIM> ) -> Self::Output {
        rhs * self
    }
}

/// Vector<T, DIM> + ( Vector<T, DIM> + BiVector<T, DIM> )
/// ( x + y + z ) + ( x + y + z + xy + xz + yz )
///
impl<T, const DIM: usize> GeometricAdd<( Vector<T, DIM>, BiVector<T, DIM> )> for Vector<T, DIM>
where
    T: Default + std::fmt::Debug + Copy,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM> );

    fn geometric_add( self, rhs: ( Vector<T, DIM>, BiVector<T, DIM> ) ) -> Self::Output {
        ( self, *rhs.vector(), *rhs.bivector() )
    }
}

/// Vector<T, DIM> - ( Vector<T, DIM> + BiVector<T, DIM> )
/// ( x + y + z ) - ( x + y + z + xy + xz + yz )
///
impl<T, const DIM: usize> GeometricSub<( Vector<T, DIM>, BiVector<T, DIM> )> for Vector<T, DIM>
where
    T: Default + std::fmt::Debug + Copy + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM> );

    fn geometric_sub( self, rhs: ( Vector<T, DIM>, BiVector<T, DIM> ) ) -> Self::Output {
        ( self, -*rhs.vector(), -*rhs.bivector() )
    }
}

/// Vector<T, DIM> * ( Vector<T, DIM> + BiVector<T, DIM> )
/// ( x + y + z ) * ( x + y + z + xy + xz + yz )
///
impl<T, const DIM: usize> GeometricProduct<( Vector<T, DIM>, BiVector<T, DIM> )> for Vector<T, DIM>
where
    T: Default + std::fmt::Debug + Copy + Mul<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( Vector<T, DIM>, BiVector<T, DIM> );

    fn geometric_product( self, rhs: ( Vector<T, DIM>, BiVector<T, DIM> ) ) -> Self::Output {
        ( *rhs.vector() * self, *rhs.bivector() * self )
    }
}

/// Vector<T, DIM> + ( Vector<T, DIM> + BiVector<T, DIM> + TriVector<T, DIM> )
/// ( x + y + z ) + ( x + y + z + xy + xz + yz + xyz )
///
impl<T, const DIM: usize> GeometricAdd<( Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )> for Vector<T, DIM>
where
    T: Default + std::fmt::Debug + Copy,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_add( self, rhs: ( Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self, *rhs.vector(), *rhs.bivector(), *rhs.trivector() )
    }
}

/// Vector<T, DIM> - ( Vector<T, DIM> + BiVector<T, DIM> + TriVector<T, DIM> )
/// ( x + y + z ) - ( x + y + z + xy + xz + yz + xyz )
///
impl<T, const DIM: usize> GeometricSub<( Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )> for Vector<T, DIM>
where
    T: Default + std::fmt::Debug + Copy + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_sub( self, rhs: ( Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self, -*rhs.vector(), -*rhs.bivector(), -*rhs.trivector() )
    }
}

/// Vector<T, DIM> * ( Vector<T, DIM> + BiVector<T, DIM> + TriVector<T, DIM> )
/// ( x + y + z ) * ( x + y + z + xy + xz + yz + xyz )
///
impl<T, const DIM: usize> GeometricProduct<( Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )> for Vector<T, DIM>
where
    T: Default + std::fmt::Debug + Copy + Mul<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_product( self, rhs: ( Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( *rhs.vector() * self, *rhs.bivector() * self, *rhs.trivector() * self )
    }
}

/// Vector<T, DIM> + ( Vector<T, DIM> + TriVector<T, DIM> )
/// ( x + y + z ) + ( x + y + z + xyz )
///
impl<T, const DIM: usize> GeometricAdd<( Vector<T, DIM>, TriVector<T, DIM> )> for Vector<T, DIM>
where
    T: Default + std::fmt::Debug + Copy,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, TriVector<T, DIM> );

    fn geometric_add( self, rhs: ( Vector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self, *rhs.vector(), *rhs.trivector() )
    }
}

/// Vector<T, DIM> - ( Vector<T, DIM> + TriVector<T, DIM> )
/// ( x + y + z ) - ( x + y + z + xyz )
///
impl<T, const DIM: usize> GeometricSub<( Vector<T, DIM>, TriVector<T, DIM> )> for Vector<T, DIM>
where
    T: Default + std::fmt::Debug + Copy + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, TriVector<T, DIM> );

    fn geometric_sub( self, rhs: ( Vector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self, -*rhs.vector(), -*rhs.trivector() )
    }
}

/// Vector<T, DIM> * ( Vector<T, DIM> + TriVector<T, DIM> )
/// ( x + y + z ) * ( x + y + z + xyz )
///
impl<T, const DIM: usize> GeometricProduct<( Vector<T, DIM>, TriVector<T, DIM> )> for Vector<T, DIM>
where
    T: Default + std::fmt::Debug + Copy + Mul<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( Vector<T, DIM>, TriVector<T, DIM> );

    fn geometric_product( self, rhs: ( Vector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( *rhs.vector() * self, *rhs.trivector() * self )
    }
}

/// Vector<T, DIM> + BiVector<T, DIM>
/// ( x + y + z ) + ( xy + xz + yz )
///
impl<T, const DIM: usize> GeometricAdd<BiVector<T, DIM>> for Vector<T, DIM>
where
    T: Default + std::fmt::Debug + Copy,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, BiVector<T, DIM> );

    fn geometric_add( self, rhs: BiVector<T, DIM> ) -> Self::Output {
        ( self, rhs )
    }
}

/// Vector<T, DIM> - BiVector<T, DIM>
/// ( x + y + z ) - ( xy + xz + yz )
///
impl<T, const DIM: usize> GeometricSub<BiVector<T, DIM>> for Vector<T, DIM>
where
    T: Default + std::fmt::Debug + Copy + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, BiVector<T, DIM> );

    fn geometric_sub( self, rhs: BiVector<T, DIM> ) -> Self::Output {
        ( self, -rhs )
    }
}

/// Vector<T, DIM> * BiVector<T, DIM>
/// ( x + y + z ) * ( xy + xz + yz )
///
impl<T, const DIM: usize> GeometricProduct<BiVector<T, DIM>> for Vector<T, DIM>
where
    T: Default + std::fmt::Debug + Copy + Mul<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = BiVector<T, DIM>;

    fn geometric_product( self, rhs: BiVector<T, DIM> ) -> Self::Output {
        rhs * self
    }
}

/// Vector<T, DIM> + ( BiVector<T, DIM> + TriVector<T, DIM> )
/// ( x + y + z ) + ( xy + xz + yz + xyz )
///
impl<T, const DIM: usize> GeometricAdd<( BiVector<T, DIM>, TriVector<T, DIM> )> for Vector<T, DIM>
where
    T: Default + std::fmt::Debug + Copy,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_add( self, rhs: ( BiVector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self, *rhs.bivector(), *rhs.trivector() )
    }
}

/// Vector<T, DIM> - ( BiVector<T, DIM> + TriVector<T, DIM> )
/// ( x + y + z ) - ( xy + xz + yz + xyz )
///
impl<T, const DIM: usize> GeometricSub<( BiVector<T, DIM>, TriVector<T, DIM> )> for Vector<T, DIM>
where
    T: Default + std::fmt::Debug + Copy + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, BiVector<T, DIM>, TriVector<T, DIM>  );

    fn geometric_sub( self, rhs: ( BiVector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self, -*rhs.bivector(), -*rhs.trivector() )
    }
}

/// Vector<T, DIM> * ( BiVector<T, DIM> + TriVector<T, DIM> )
/// ( x + y + z ) * ( xy + xz + yz + xyz )
///
impl<T, const DIM: usize> GeometricProduct<( BiVector<T, DIM>, TriVector<T, DIM> )> for Vector<T, DIM>
where
    T: Default + std::fmt::Debug + Copy + Mul<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( BiVector<T, DIM>, TriVector<T, DIM>  );

    fn geometric_product( self, rhs: ( BiVector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( *rhs.bivector() * self, *rhs.trivector() * self )
    }
}

/// Vector<T, DIM> + TriVector<T, DIM>
/// ( x + y + z ) + xyz
///
impl<T, const DIM: usize> GeometricAdd<TriVector<T, DIM>> for Vector<T, DIM>
where
    T: Default + std::fmt::Debug + Copy,
{
    type Output = ( T, TriVector<T, DIM> );

    fn geometric_add( self, rhs: TriVector<T, DIM> ) -> Self::Output {
        ( self, rhs )
    }
}

/// Vector<T, DIM> - TriVector<T, DIM>
/// ( x + y + z ) - xyz
///
impl<T, const DIM: usize> GeometricSub<TriVector<T, DIM>> for Vector<T, DIM>
where
    T: Default + std::fmt::Debug + Copy + Neg<Output = T>,
{
    type Output = ( T, TriVector<T, DIM> );

    fn geometric_sub( self, rhs: TriVector<T, DIM> ) -> Self::Output {
        ( self, -rhs )
    }
}

/// Vector<T, DIM> * TriVector<T, DIM>
/// ( x + y + z ) * xyz
///
impl<T, const DIM: usize> GeometricProduct<TriVector<T, DIM>> for Vector<T, DIM>
where
    T: Default + std::fmt::Debug + Copy + Mul<Output = T>,
{
    type Output = TriVector<T, DIM>;

    fn geometric_product( self, rhs: TriVector<T, DIM> ) -> Self::Output {
        rhs * self
    }
}
