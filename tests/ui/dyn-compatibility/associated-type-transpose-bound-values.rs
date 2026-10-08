//@ check-pass
//@ revisions: current next
//@ ignore-compare-mode-next-solver (explicit revisions)
//@[next] compile-flags: -Znext-solver=globally

mod constrained {
    pub trait Matrix {
        type Coordinates;
    }

    pub trait Matrix2D: Matrix<Coordinates = <Self as Matrix2D>::RowIndex> {
        type RowIndex;
    }

    pub trait Transposable<T: Matrix2D<RowIndex = Self::RowIndex>>:
        Matrix2D<RowIndex = u32>
    {
        fn transpose(&self) -> T;
    }

    fn object<T: Matrix2D<RowIndex = u32>>(
        _: &dyn Transposable<T, RowIndex = u32, Coordinates = u32>,
    ) {
    }
}

mod composite {
    pub trait Matrix {
        type Coordinates;
    }

    pub trait Matrix2D: Matrix<Coordinates = <Self as Matrix2D>::RowIndex> {
        type RowIndex;
    }

    pub trait Target {
        type Value;
    }

    pub trait Transposable<T: Target<Value = (Self::RowIndex, Option<Self::RowIndex>)>>:
        Matrix2D
    {
        fn transpose(&self) -> T;
    }

    fn object<T: Target<Value = (u32, Option<u32>)>>(
        _: &dyn Transposable<T, RowIndex = u32, Coordinates = u32>,
    ) {
    }
}

mod higher_ranked {
    pub trait Matrix<'a> {
        type Coordinates;
    }

    pub trait Matrix2D<'a>: Matrix<'a, Coordinates = <Self as Matrix2D<'a>>::RowIndex> {
        type RowIndex;
    }

    pub trait Transposable<T>: for<'a> Matrix2D<'a>
    where
        for<'a> T: Matrix2D<'a, RowIndex = <Self as Matrix2D<'a>>::RowIndex>,
    {
        fn transpose(&self) -> T;
    }

    fn object<T: for<'a> Matrix2D<'a, RowIndex = u32>>(
        _: &dyn Transposable<T, RowIndex = u32, Coordinates = u32>,
    ) {
    }

    pub trait ConstrainedTransposable<T>: for<'a> Matrix2D<'a, RowIndex = u32>
    where
        for<'a> T: Matrix2D<'a, RowIndex = <Self as Matrix2D<'a>>::RowIndex>,
    {
        fn transpose(&self) -> T;
    }

    fn constrained_object<T: for<'a> Matrix2D<'a, RowIndex = u32>>(
        _: &dyn ConstrainedTransposable<T, RowIndex = u32, Coordinates = u32>,
    ) {
    }
}

fn main() {}
