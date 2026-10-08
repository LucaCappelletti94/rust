//@ check-pass
//@ revisions: current next
//@ ignore-compare-mode-next-solver (explicit revisions)
//@[next] compile-flags: -Znext-solver=globally

pub trait Matrix {
    type Coordinates;
}

pub trait Matrix2D: Matrix<Coordinates = <Self as Matrix2D>::RowIndex> {
    type RowIndex;
}

pub trait Transposable<T: Matrix2D<RowIndex = Self::RowIndex>>: Matrix2D {
    fn transpose(&self) -> T;
}

fn object<T: Matrix2D>(
    _: &dyn Transposable<T, RowIndex = T::RowIndex, Coordinates = T::RowIndex>,
) {
}

mod two_dimensions {
    pub trait Matrix {
        type Coordinates;
    }

    pub trait Matrix2D:
        Matrix<Coordinates = (<Self as Matrix2D>::RowIndex, <Self as Matrix2D>::ColumnIndex)>
    {
        type RowIndex;
        type ColumnIndex;
    }

    pub trait Transposable<
        T: Matrix2D<RowIndex = Self::ColumnIndex, ColumnIndex = Self::RowIndex>,
    >: Matrix2D
    {
        fn transpose(&self) -> T;
    }

    fn object<T: Matrix2D>(
        _: &dyn Transposable<
            T,
            RowIndex = T::ColumnIndex,
            ColumnIndex = T::RowIndex,
            Coordinates = (T::ColumnIndex, T::RowIndex),
        >,
    ) {
    }
}

fn main() {}
