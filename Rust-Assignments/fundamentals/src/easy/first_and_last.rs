/*
  Problem 7: First and Last

  Write a function that takes a slice of i32 values and returns a tuple of the first and last elements.
  If the slice is empty, return None.

  Run the tests for this problem with:
    cargo test --test first_and_last_test
*/

pub fn first_and_last(slice: &[i32]) -> Option<(i32, i32)> {
    // if slice.is_empty() {
    //   None
    // } else{
    //   Some((slice[0], slice[slice.len() - 1]))
    // }
    // match(slice.first(), slice.last()){
    //   (Some(&first), Some(&last)) => Some((first, last)),
    //   _ => None
    // }

    Some((*slice.first()?, *slice.last()?))
}
