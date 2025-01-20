// Copyright 2024 Bewusstsein Labs

#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]

pub mod traits;
pub mod ops;
pub mod vector;
pub mod bivector;
pub mod trivector;
//pub mod multivector;
pub mod rotor;

use linear_algebra::{
    ops::InnerProduct,
    vector::Vector
};
use crate::{
    ops::{
        ExteriorProduct,
        GeometricProduct
    },
    bivector::BiVector
};
