pub fn vec_generics(){
    let v = vec![10, 20, 30];
    let v2 = vec!([String::from("Amit"), String::from("Kumar"), String::from("Gupta")]);

    println!("{:?}", first_element(v));
    println!("{:?}", first_element(v2));
}

fn first_element<T>(v:  Vec<T> ) -> T{
    return v.into_iter().nth(0).unwrap();
}