/*
  Problem 24: Vec — Running Sum

  Write a function that takes a Vec<i32> and returns a new Vec<i32> where each element is the running sum up to that index.
  For example, [1, 2, 3] becomes [1, 3, 6].

  Run the tests for this problem with:
    cargo test --test running_sum_test
*/

pub fn running_sum(v: Vec<i32>) -> Vec<i32> {
    // let mut res: Vec<i32> = Vec::new(); //  here we have explicite type
    // let mut res = Vec::with_capacity(v.len());
    // let mut prefix_sum = 0;
    // for val in v {
    //   prefix_sum += val;
    //   res.push(prefix_sum);
    // }

    // return res;
    let mut v = v;
    for i in 1..v.len(){
      v[i] += v[i - 1];
    }
    v

    // let mut sum = 0;
    // v.into_iter()
    //   .map(|x|{
    //     sum += x
    //     sum
    //   })
    //   .collect()
}
