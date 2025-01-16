// Copyright 2024 Bewusstsein Labs

use linear_algebra::vector::Vector;

use crate::{
    bivector::BiVector,
    trivector::TriVector
};

pub trait XY<T> {
    fn xy( &self ) -> &T;
}

pub trait XYMut<T> {
    fn xy_mut( &mut self ) -> &mut T;
}

pub trait XZ<T> {
    fn xz( &self ) -> &T;
}

pub trait XZMut<T> {
    fn xz_mut( &mut self ) -> &mut T;
}

pub trait YZ<T> {
    fn yz( &self ) -> &T;
}

pub trait YZMut<T> {
    fn yz_mut( &mut self ) -> &mut T;
}

pub trait ScalarComponent<T> {
    fn scalar( &self ) -> &T;
}

pub trait ScalarComponentMut<T> {
    fn scalar_mut( &mut self ) -> &mut T;
}

pub trait VectorComponent<T, const DIM: usize>
where
    T: 'static + Copy + Default + std::fmt::Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn vector( &self ) -> &Vector<T, DIM>;
}

pub trait VectorComponentMut<T, const DIM: usize>
where
    T: 'static + Copy + Default + std::fmt::Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn vector_mut( &mut self ) -> &mut Vector<T, DIM>;
}

pub trait BiVectorComponent<T, const DIM: usize>
where
    T: 'static + Copy + Default + std::fmt::Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn bivector( &self ) -> &BiVector<T, DIM>;
}

pub trait BiVectorComponentMut<T, const DIM: usize>
where
    T: 'static + Copy + Default + std::fmt::Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn bivector_mut( &mut self ) -> &mut BiVector<T, DIM>;
}

pub trait TriVectorComponent<T, const DIM: usize>
where
    T: 'static + Copy + Default + std::fmt::Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn trivector( &self ) -> &TriVector<T, DIM>;
}

pub trait TriVectorComponentMut<T, const DIM: usize>
where
    T: 'static + Copy + Default + std::fmt::Debug,
    [(); DIM * ( DIM - 1 ) / 2 ]:
{
    fn trivector_mut( &mut self ) -> &mut TriVector<T, DIM>;
}
