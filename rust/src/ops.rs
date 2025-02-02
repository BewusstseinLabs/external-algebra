
pub trait GeometricInverse {
    type Output;

    fn geometric_inverse( self ) -> Self::Output;
}

pub trait InteriorProduct<Rhs = Self> {
    type Output;

    fn interior_product( self, rhs: Rhs ) -> Self::Output;
}

pub trait ExteriorProduct<Rhs = Self> {
    type Output;

    fn exterior_product( self, rhs: Rhs ) -> Self::Output;
}

pub trait GeometricAdd<Rhs = Self> {
    type Output;

    fn geometric_add( self, rhs: Rhs ) -> Self::Output;
}

pub trait GeometricSub<Rhs = Self> {
    type Output;

    fn geometric_sub( self, rhs: Rhs ) -> Self::Output;
}

pub trait GeometricProduct<Rhs = Self> {
    type Output;

    fn geometric_product( self, rhs: Rhs ) -> Self::Output;
}
