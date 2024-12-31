
pub trait InteriorProduct<Rhs = Self> {
    type Output;

    fn interior_product( self, rhs: Rhs ) -> Self::Output;
}

pub trait ExteriorProduct<Rhs = Self> {
    type Output;

    fn exterior_product( self, rhs: Rhs ) -> Self::Output;
}
