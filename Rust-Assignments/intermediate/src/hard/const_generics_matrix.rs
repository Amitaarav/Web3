/*
  Problem 66: Const Generics — Fixed-Size Matrix

  Define a struct Matrix<T, const R: usize, const C: usize> that stores data in
  a 2D array [[T; C]; R]. Implement a new(data: [[T; C]; R]) function and a
  get(r, c) method. Use const generics to enforce dimensions at compile time.

  Run the tests for this problem with:
    cargo test --test const_generics_matrix_test
*/

pub struct Matrix<T, const R: usize, const C: usize> {
    pub data: [[T; C]; R],
}

impl<T: Copy, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn new(data: [[T; C]; R]) -> Self {
        todo!()
    }

    pub fn get(&self, r: usize, c: usize) -> Option<&T> {
        todo!()
    }
}
