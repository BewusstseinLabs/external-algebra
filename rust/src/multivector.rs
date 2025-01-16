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
        GeometricSub
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

impl<T> GeometricAdd<T> for T
where
    T: Default + std::fmt::Debug + Copy + Add<Output = T>
{
    type Output = T;

    fn geometric_add( self, rhs: T ) -> Self::Output {
        self + rhs
    }
}

impl<T> GeometricSub<T> for T
where
    T: Default + std::fmt::Debug + Copy + Sub<Output = T>
{
    type Output = T;

    fn geometric_sub( self, rhs: T ) -> Self::Output {
        self - rhs
    }
}

impl<T, const DIM: usize> GeometricAdd<Vector<T, DIM>> for T
where
    T: Default + std::fmt::Debug + Copy
{
    type Output = ( T, Vector<T, DIM> );

    fn geometric_add( self, rhs: Vector<T, DIM> ) -> Self::Output {
        ( self, rhs )
    }
}

impl<T, const DIM: usize> GeometricSub<Vector<T, DIM>> for T
where
    T: Default + std::fmt::Debug + Copy + Neg<Output = T>
{
    type Output = ( T, Vector<T, DIM> );

    fn geometric_sub( self, rhs: Vector<T, DIM> ) -> Self::Output {
        ( self, -rhs )
    }
}

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

impl<T, const DIM: usize> GeometricAdd<TriVector<T, DIM>> for T
where
    T: Default + std::fmt::Debug + Copy,
{
    type Output = ( T, TriVector<T, DIM> );

    fn geometric_add( self, rhs: TriVector<T, DIM> ) -> Self::Output {
        ( self, rhs )
    }
}

impl<T, const DIM: usize> GeometricSub<TriVector<T, DIM>> for T
where
    T: Default + std::fmt::Debug + Copy + Neg<Output = T>,
{
    type Output = ( T, TriVector<T, DIM> );

    fn geometric_sub( self, rhs: TriVector<T, DIM> ) -> Self::Output {
        ( self, -rhs )
    }
}

impl<T, const DIM: usize> GeometricAdd<( Vector<T, DIM>, BiVector<T, DIM> )> for T
where
    T: Default + std::fmt::Debug + Copy,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM> );

    fn geometric_add( self, rhs: ( Vector<T, DIM>, BiVector<T, DIM> ) ) -> Self::Output {
        ( self, rhs.0, rhs.1 )
    }
}

impl<T, const DIM: usize> GeometricSub<( Vector<T, DIM>, BiVector<T, DIM> )> for T
where
    T: Default + std::fmt::Debug + Copy + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM> );

    fn geometric_sub( self, rhs: ( Vector<T, DIM>, BiVector<T, DIM> ) ) -> Self::Output {
        ( self, -rhs.0, -rhs.1 )
    }
}

impl<T, const DIM: usize> GeometricAdd<( Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )> for T
where
    T: Default + std::fmt::Debug + Copy,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_add( self, rhs: ( Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self, rhs.0, rhs.1, rhs.2 )
    }
}

impl<T, const DIM: usize> GeometricSub<( Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )> for T
where
    T: Default + std::fmt::Debug + Copy + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_sub( self, rhs: ( Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self, -rhs.0, -rhs.1, -rhs.2 )
    }
}

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

// ( T, Vector<T, DIM> )

impl<T, const DIM: usize> GeometricAdd<T> for ( T, Vector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Add<Output = T>
{
    type Output = ( T, Vector<T, DIM> );

    fn geometric_add( self, rhs: T ) -> Self::Output {
        ( self.0 + rhs, self.1 )
    }
}

impl<T, const DIM: usize> GeometricSub<T> for ( T, Vector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Sub<Output = T>
{
    type Output = ( T, Vector<T, DIM> );

    fn geometric_sub( self, rhs: T ) -> Self::Output {
        ( self.0 - rhs, self.1 )
    }
}

impl<T, const DIM: usize> GeometricAdd<Vector<T, DIM>> for ( T, Vector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Add<Output = T>
{
    type Output = ( T, Vector<T, DIM> );

    fn geometric_add( self, rhs: Vector<T, DIM> ) -> Self::Output {
        ( self.0, self.1 + rhs )
    }
}

impl<T, const DIM: usize> GeometricSub<Vector<T, DIM>> for ( T, Vector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Sub<Output = T>
{
    type Output = ( T, Vector<T, DIM> );

    fn geometric_sub( self, rhs: Vector<T, DIM> ) -> Self::Output {
        ( self.0, self.1 - rhs )
    }
}

impl<T, const DIM: usize> GeometricAdd<BiVector<T, DIM>> for ( T, Vector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM> );

    fn geometric_add( self, rhs: BiVector<T, DIM> ) -> Self::Output {
        ( self.0, self.1, rhs )
    }
}

impl<T, const DIM: usize> GeometricSub<BiVector<T, DIM>> for ( T, Vector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM> );

    fn geometric_sub( self, rhs: BiVector<T, DIM> ) -> Self::Output {
        ( self.0, self.1, -rhs )
    }
}

impl<T, const DIM: usize> GeometricAdd<TriVector<T, DIM>> for ( T, Vector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy,
{
    type Output = ( T, Vector<T, DIM>, TriVector<T, DIM> );

    fn geometric_add( self, rhs: TriVector<T, DIM> ) -> Self::Output {
        ( self.0, self.1, rhs )
    }
}

impl<T, const DIM: usize> GeometricSub<TriVector<T, DIM>> for ( T, Vector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Neg<Output = T>,
{
    type Output = ( T, Vector<T, DIM>, TriVector<T, DIM> );

    fn geometric_sub( self, rhs: TriVector<T, DIM> ) -> Self::Output {
        ( self.0, self.1, -rhs )
    }
}

impl<T, const DIM: usize> GeometricAdd<( Vector<T, DIM>, BiVector<T, DIM> )> for ( T, Vector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Add<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM> );

    fn geometric_add( self, rhs: ( Vector<T, DIM>, BiVector<T, DIM> ) ) -> Self::Output {
        ( self.0, self.1 + rhs.0, rhs.1 )
    }
}

impl<T, const DIM: usize> GeometricSub<( Vector<T, DIM>, BiVector<T, DIM> )> for ( T, Vector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Sub<Output = T> + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM> );

    fn geometric_sub( self, rhs: ( Vector<T, DIM>, BiVector<T, DIM> ) ) -> Self::Output {
        ( self.0, self.1 - rhs.0, -rhs.1 )
    }
}

impl<T, const DIM: usize> GeometricAdd<( Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )> for ( T, Vector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Add<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_add( self, rhs: ( Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self.0, self.1 + rhs.0, rhs.1, rhs.2 )
    }
}

impl<T, const DIM: usize> GeometricSub<( Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )> for ( T, Vector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Sub<Output = T> + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_sub( self, rhs: ( Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self.0, self.1 - rhs.0, -rhs.1, -rhs.2 )
    }
}

impl<T, const DIM: usize> ScalarComponent<T> for ( T, Vector<T, DIM> )
where
    T: 'static + Copy + Default + std::fmt::Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn scalar( &self ) -> &T {
        &self.0
    }
}

impl<T, const DIM: usize> ScalarComponentMut<T> for ( T, Vector<T, DIM> )
where
    T: 'static + Copy + Default + std::fmt::Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn scalar_mut( &mut self ) -> &mut T {
        &mut self.0
    }
}

impl<T, const DIM: usize> VectorComponent<T, DIM> for ( T, Vector<T, DIM> )
where
    T: 'static + Copy + Default + std::fmt::Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn vector( &self ) -> &Vector<T, DIM> {
        &self.1
    }
}

impl<T, const DIM: usize> VectorComponentMut<T, DIM> for ( T, Vector<T, DIM> )
where
    T: 'static + Copy + Default + std::fmt::Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn vector_mut( &mut self ) -> &mut Vector<T, DIM> {
        &mut self.1
    }
}

// ( T, Vector<T, DIM>, BiVector<T, DIM> )

impl<T, const DIM: usize> GeometricAdd<T> for ( T, Vector<T, DIM>, BiVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Add<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM> );

    fn geometric_add( self, rhs: T ) -> Self::Output {
        ( self.0 + rhs, self.1, self.2 )
    }
}

impl<T, const DIM: usize> GeometricSub<T> for ( T, Vector<T, DIM>, BiVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Sub<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM> );

    fn geometric_sub( self, rhs: T ) -> Self::Output {
        ( self.0 - rhs, self.1, self.2 )
    }
}

impl<T, const DIM: usize> GeometricAdd<Vector<T, DIM>> for ( T, Vector<T, DIM>, BiVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Add<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM> );

    fn geometric_add( self, rhs: Vector<T, DIM> ) -> Self::Output {
        ( self.0, self.1 + rhs, self.2 )
    }
}

impl<T, const DIM: usize> GeometricSub<Vector<T, DIM>> for ( T, Vector<T, DIM>, BiVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Sub<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM> );

    fn geometric_sub( self, rhs: Vector<T, DIM> ) -> Self::Output {
        ( self.0, self.1 - rhs, self.2 )
    }
}

impl<T, const DIM: usize> GeometricAdd<BiVector<T, DIM>> for ( T, Vector<T, DIM>, BiVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM> );

    fn geometric_add( self, rhs: BiVector<T, DIM> ) -> Self::Output {
        ( self.0, self.1, self.2 + rhs )
    }
}

impl<T, const DIM: usize> GeometricSub<BiVector<T, DIM>> for ( T, Vector<T, DIM>, BiVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM> );

    fn geometric_sub( self, rhs: BiVector<T, DIM> ) -> Self::Output {
        ( self.0, self.1, self.2 - rhs )
    }
}

impl<T, const DIM: usize> GeometricAdd<TriVector<T, DIM>> for ( T, Vector<T, DIM>, BiVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy,
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_add( self, rhs: TriVector<T, DIM> ) -> Self::Output {
        ( self.0, self.1, self.2, rhs )
    }
}

impl<T, const DIM: usize> GeometricSub<TriVector<T, DIM>> for ( T, Vector<T, DIM>, BiVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Neg<Output = T>,
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_sub( self, rhs: TriVector<T, DIM> ) -> Self::Output {
        ( self.0, self.1, self.2, -rhs )
    }
}

impl<T, const DIM: usize> GeometricAdd<( Vector<T, DIM>, BiVector<T, DIM> )> for ( T, Vector<T, DIM>, BiVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Add<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM> );

    fn geometric_add( self, rhs: ( Vector<T, DIM>, BiVector<T, DIM> ) ) -> Self::Output {
        ( self.0, self.1 + rhs.0, self.2 + rhs.1 )
    }
}

impl<T, const DIM: usize> GeometricSub<( Vector<T, DIM>, BiVector<T, DIM> )> for ( T, Vector<T, DIM>, BiVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Sub<Output = T> + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM> );

    fn geometric_sub( self, rhs: ( Vector<T, DIM>, BiVector<T, DIM> ) ) -> Self::Output {
        ( self.0, self.1 - rhs.0, self.2 - rhs.1 )
    }
}

impl<T, const DIM: usize> GeometricAdd<( Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )> for ( T, Vector<T, DIM>, BiVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Add<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_add( self, rhs: ( Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self.0, self.1 + rhs.0, self.2 + rhs.1, rhs.2 )
    }
}

impl<T, const DIM: usize> GeometricSub<( Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )> for ( T, Vector<T, DIM>, BiVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Sub<Output = T> + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_sub( self, rhs: ( Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self.0, self.1 - rhs.0, self.2 - rhs.1, -rhs.2 )
    }
}

impl<T, const DIM: usize> ScalarComponent<T> for ( T, Vector<T, DIM>, BiVector<T, DIM> )
where
    T: 'static + Copy + Default + std::fmt::Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn scalar( &self ) -> &T {
        &self.0
    }
}

impl<T, const DIM: usize> ScalarComponentMut<T> for ( T, Vector<T, DIM>, BiVector<T, DIM> )
where
    T: 'static + Copy + Default + std::fmt::Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn scalar_mut( &mut self ) -> &mut T {
        &mut self.0
    }
}

impl<T, const DIM: usize> VectorComponent<T, DIM> for ( T, Vector<T, DIM>, BiVector<T, DIM> )
where
    T: 'static + Copy + Default + std::fmt::Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn vector( &self ) -> &Vector<T, DIM> {
        &self.1
    }
}

impl<T, const DIM: usize> VectorComponentMut<T, DIM> for ( T, Vector<T, DIM>, BiVector<T, DIM> )
where
    T: 'static + Copy + Default + std::fmt::Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn vector_mut( &mut self ) -> &mut Vector<T, DIM> {
        &mut self.1
    }
}

impl<T, const DIM: usize> BiVectorComponent<T, DIM> for ( T, Vector<T, DIM>, BiVector<T, DIM> )
where
    T: 'static + Copy + Default + std::fmt::Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn bivector( &self ) -> &BiVector<T, DIM> {
        &self.2
    }
}

impl<T, const DIM: usize> BiVectorComponentMut<T, DIM> for ( T, Vector<T, DIM>, BiVector<T, DIM> )
where
    T: 'static + Copy + Default + std::fmt::Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn bivector_mut( &mut self ) -> &mut BiVector<T, DIM> {
        &mut self.2
    }
}

// ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )

impl<T, const DIM: usize> GeometricAdd<T> for ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Add<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_add( self, rhs: T ) -> Self::Output {
        ( self.0 + rhs, self.1, self.2, self.3 )
    }
}

impl<T, const DIM: usize> GeometricSub<T> for ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Sub<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_sub( self, rhs: T ) -> Self::Output {
        ( self.0 - rhs, self.1, self.2, self.3 )
    }
}

impl<T, const DIM: usize> GeometricAdd<Vector<T, DIM>> for ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Add<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_add( self, rhs: Vector<T, DIM> ) -> Self::Output {
        ( self.0, self.1 + rhs, self.2, self.3 )
    }
}

impl<T, const DIM: usize> GeometricSub<Vector<T, DIM>> for ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Sub<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_sub( self, rhs: Vector<T, DIM> ) -> Self::Output {
        ( self.0, self.1 - rhs, self.2, self.3 )
    }
}

impl<T, const DIM: usize> GeometricAdd<BiVector<T, DIM>> for ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_add( self, rhs: BiVector<T, DIM> ) -> Self::Output {
        ( self.0, self.1, self.2 + rhs, self.3 )
    }
}

impl<T, const DIM: usize> GeometricSub<BiVector<T, DIM>> for ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_sub( self, rhs: BiVector<T, DIM> ) -> Self::Output {
        ( self.0, self.1, self.2 - rhs, self.3 )
    }
}

impl<T, const DIM: usize> GeometricAdd<TriVector<T, DIM>> for ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy,
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_add( self, rhs: TriVector<T, DIM> ) -> Self::Output {
        ( self.0, self.1, self.2, self.3 + rhs )
    }
}

impl<T, const DIM: usize> GeometricSub<TriVector<T, DIM>> for ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Neg<Output = T>,
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_sub( self, rhs: TriVector<T, DIM> ) -> Self::Output {
        ( self.0, self.1, self.2, self.3 - rhs )
    }
}

impl<T, const DIM: usize> GeometricAdd<( Vector<T, DIM>, BiVector<T, DIM> )> for ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Add<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_add( self, rhs: ( Vector<T, DIM>, BiVector<T, DIM> ) ) -> Self::Output {
        ( self.0, self.1 + rhs.0, self.2 + rhs.1, self.3 )
    }
}

impl<T, const DIM: usize> GeometricSub<( Vector<T, DIM>, BiVector<T, DIM> )> for ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Sub<Output = T> + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_sub( self, rhs: ( Vector<T, DIM>, BiVector<T, DIM> ) ) -> Self::Output {
        ( self.0, self.1 - rhs.0, self.2 - rhs.1, self.3 )
    }
}

impl<T, const DIM: usize> GeometricAdd<( Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )> for ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Add<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_add( self, rhs: ( Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self.0, self.1 + rhs.0, self.2 + rhs.1, self.3 + rhs.2 )
    }
}

impl<T, const DIM: usize> GeometricSub<( Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )> for ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: Default + std::fmt::Debug + Copy + Sub<Output = T> + Neg<Output = T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> );

    fn geometric_sub( self, rhs: ( Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> ) ) -> Self::Output {
        ( self.0, self.1 - rhs.0, self.2 - rhs.1, self.3, -rhs.2 )
    }
}

impl<T, const DIM: usize> ScalarComponent<T> for ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: 'static + Copy + Default + std::fmt::Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn scalar( &self ) -> &T {
        &self.0
    }
}

impl<T, const DIM: usize> ScalarComponentMut<T> for ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: 'static + Copy + Default + std::fmt::Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn scalar_mut( &mut self ) -> &mut T {
        &mut self.0
    }
}

impl<T, const DIM: usize> VectorComponent<T, DIM> for ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: 'static + Copy + Default + std::fmt::Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn vector( &self ) -> &Vector<T, DIM> {
        &self.1
    }
}

impl<T, const DIM: usize> VectorComponentMut<T, DIM> for ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: 'static + Copy + Default + std::fmt::Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn vector_mut( &mut self ) -> &mut Vector<T, DIM> {
        &mut self.1
    }
}

impl<T, const DIM: usize> BiVectorComponent<T, DIM> for ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: 'static + Copy + Default + std::fmt::Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn bivector( &self ) -> &BiVector<T, DIM> {
        &self.2
    }
}

impl<T, const DIM: usize> BiVectorComponentMut<T, DIM> for ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: 'static + Copy + Default + std::fmt::Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn bivector_mut( &mut self ) -> &mut BiVector<T, DIM> {
        &mut self.2
    }
}

impl<T, const DIM: usize> TriVectorComponent<T, DIM> for ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: 'static + Copy + Default + std::fmt::Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn trivector( &self ) -> &TriVector<T, DIM> {
        &self.3
    }
}

impl<T, const DIM: usize> TriVectorComponentMut<T, DIM> for ( T, Vector<T, DIM>, BiVector<T, DIM>, TriVector<T, DIM> )
where
    T: 'static + Copy + Default + std::fmt::Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn trivector_mut( &mut self ) -> &mut TriVector<T, DIM> {
        &mut self.3
    }
}
