// Copyright 2024 Bewusstsein Labs

use std::{
    fmt::Debug,
    ops::{ Neg, Add, AddAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign }
};
use num::traits::{ Num, Float };

use linear_algebra::{
    ops::{
        Magnitude,
        Normalize,
        Conjugate
    },
    vector::Vector
};

use crate::{
    ops::{
        //InteriorProduct,
        //ExteriorProduct
    },
    traits::{
        ScalarComponent,
        ScalarComponentMut,
        BiVectorComponent,
        BiVectorComponentMut
    },
    bivector::BiVector
};

/// A rotor type of generic element and size.
///
#[derive( Clone, Copy, Debug, Default )]
pub struct Rotor<T, const DIM: usize>( T, BiVector<T, DIM> )
where
    T: 'static + Default + Copy + Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:;

impl<T, const DIM: usize> Rotor<T, DIM>
where
    T: 'static + Copy + Default + Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    /// Creates a new const [`Rotor`].
    ///
    pub const fn new_const( scalar: T, bivector: BiVector<T, DIM> ) -> Self {
        Self ( scalar, bivector )
    }

    /// Creates a new [`Rotor`].
    ///
    pub fn new( scalar: T, bivector: BiVector<T, DIM> ) -> Self {
        Self ( scalar, bivector )
    }

    /// Creates a new zero filled [`Rotor`].
    ///
    pub fn zero() -> Self
    where
        T: Num
    {
        Self ( T::zero(), BiVector::zero() )
    }
}

impl<T, const DIM: usize> ScalarComponent<T> for Rotor<T, DIM>
where
    T: 'static + Copy + Default + std::fmt::Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn scalar( &self ) -> &T {
        &self.0
    }
}

impl<T, const DIM: usize> ScalarComponentMut<T> for Rotor<T, DIM>
where
    T: 'static + Copy + Default + std::fmt::Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn scalar_mut( &mut self ) -> &mut T {
        &mut self.0
    }
}

impl<T, const DIM: usize> BiVectorComponent<T, DIM> for Rotor<T, DIM>
where
    T: 'static + Copy + Default + std::fmt::Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn bivector( &self ) -> &BiVector<T, DIM> {
        &self.1
    }
}

impl<T, const DIM: usize> BiVectorComponentMut<T, DIM> for Rotor<T, DIM>
where
    T: 'static + Copy + Default + std::fmt::Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn bivector_mut( &mut self ) -> &mut BiVector<T, DIM> {
        &mut self.1
    }
}

impl<T, const DIM: usize> PartialEq for Rotor<T, DIM>
where
    T: 'static + Copy + Default + Debug + PartialEq,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn eq( &self, other: &Self ) -> bool {
        self.0 == other.0 &&
        self.1 == other.1
    }
}

impl<T, const DIM: usize> Magnitude for Rotor<T, DIM>
where
    T: Default + Copy + Debug + Div<Output = T> + Num + Float,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = T;

    fn magnitude( &self ) -> Self::Output {
        self.0.sqrt() + self.1.magnitude()
    }
}

impl<T, const DIM: usize> Normalize for Rotor<T, DIM>
where
    T: Default + Copy + Debug + Div<Output = T> + DivAssign<T> + Num + Float,
    BiVector<T, DIM>: DivAssign<T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = Rotor<T, DIM>;

    fn normalize( mut self ) -> Self::Output {
        let magnitude = self.magnitude();
        self.0 /= magnitude;
        self.1 /= magnitude;
        self
    }
}

impl<T, const DIM: usize> Conjugate for Rotor<T, DIM>
where
    T: Default + Copy + Debug + Neg<Output = T>,
    BiVector<T, DIM>: Neg<Output = BiVector<T, DIM>>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = Rotor<T, DIM>;

    fn conjugate( mut self ) -> Self::Output {
        self.1 = -self.1;
        self
    }
}

/*
impl<T, const COL: usize> GeometricProduct<Vector<T, COL>> for Rotor<T, DIM>
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
*/

/*
impl<T, const DIM: usize> Mul<Vector<T, DIM>> for Rotor<T, DIM>
where
    T: Default + Copy + Debug + Div<Output = T> + DivAssign<T> + Num + Float,
    BiVector<T, DIM>: DivAssign<T>,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    type Output = Vector<T, DIM>;

    fn mul( self, rhs: Vector<T, DIM> ) -> Vector<T, DIM> {
        self.
    }
}
*/

pub type Rotor2<T> = Rotor<T, 2>;
pub type Rotor3<T> = Rotor<T, 3>;
pub type Rotor4<T> = Rotor<T, 4>;
