// Copyright 2024 Bewusstsein Labs

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
