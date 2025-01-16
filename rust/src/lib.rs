// Copyright 2024 Bewusstsein Labs

#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]

pub mod traits;
pub mod ops;
pub mod vector;
pub mod bivector;
pub mod trivector;
pub mod multivector;
pub mod rotor;

pub const fn inclusive_combinations<T, const N: usize>( items: [T; N] ) -> [( T, T ); N * N]
where
    T: Copy
{
    let mut result = [( items[ 0 ], items[ 0 ] ); N * N];
    let mut index = 0;
    let mut i = 0;
    while i < N {
        let mut j = 0;
        while j < N {
            result[ index ] = ( items[ i ], items[ j ] );
            index += 1;
            j += 1;
        }
        i += 1;
    }
    result
}

pub const fn exclusive_combinations<T, const N: usize>( items: [T; N] ) -> [( T, T ); N * ( N - 1 )]
where
    T: Copy
{
    let mut result = [( items[ 0 ], items[ 0 ] ); N * (N - 1)];
    let mut index = 0;
    let mut i = 0;
    while i < N {
        let mut j = 0;
        while j < N {
            if i != j {
                result[ index ] = ( items[ i ], items[ j ] );
                index += 1;
            }
            j += 1;
        }
        i += 1;
    }
    result
}

pub const fn unique_combinations<T, const N: usize>( items: [T; N] ) -> [( T, T ); N * ( N - 1 ) / 2]
where
    T: Copy,
{
    let mut result = [( items[ 0 ], items[ 0 ] ); N * ( N - 1 ) / 2];
    let mut index = 0;
    let mut i = 0;
    while i < N {
        let mut j = i + 1;
        while j < N {
            result[index] = ( items[ i ], items[ j ] );
            index += 1;
            j += 1;
        }
        i += 1;
    }
    result
}

pub const fn reverse<const N: usize, T>( arr: [T; N] ) -> [T; N]
where
    T: Copy
{
    let mut reversed = arr;
    let mut i = 0;
    while i < N / 2 {
        let temp = reversed[ i ];
        reversed[ i ] = reversed[ N - 1 - i ];
        reversed[ N - 1 - i ] = temp;
        i += 1;
    }
    reversed
}

pub const fn progression<const N: usize>() -> [usize; N] {
    let mut result = [0; N];
    let mut i = 0;
    while i < N {
        result[i] = i;
        i += 1;
    }
    result
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inclusive_combinations_test() {
        const ITEMS: [u32; 3] = [ 1, 2, 3 ];
        const RESULT: [( u32, u32 ); 9] = inclusive_combinations( ITEMS );

        assert_eq!( RESULT, [
            (1, 1), (1, 2), (1, 3),
            (2, 1), (2, 2), (2, 3),
            (3, 1), (3, 2), (3, 3)
        ]);
    }

    #[test]
    fn exclusive_combinations_test() {
        const ITEMS: [u32; 3] = [ 1, 2, 3 ];
        const RESULT: [( u32, u32 ); 6] = exclusive_combinations( ITEMS );

        assert_eq!(
            RESULT,
            [
                (1, 2),
                (1, 3),
                (2, 1),
                (2, 3),
                (3, 1),
                (3, 2)
            ]
        );
    }
}
