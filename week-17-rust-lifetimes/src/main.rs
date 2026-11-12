fn main() {
    let str1 = String::from("Amit");
    let str2 = String::from("Kumar");
    let longest = longest_str(&str1, &str2); // returning data with lifetime `'b`
    println!("{}", longest);

    let a = 5;
    let b = 6;
    let flag = true;

    let res = choose(a, b, flag); // choose stack: a and b copied, and copying is extremely cheap. Copying becomes expensive when data are: String, Vec, HashMap, Large struct
    println!("{}", res); // choose stack: a and b died, original still exist in main

    println!("{} {}", a, b); // main stack: a and b not died

    let new_str = String::from("Aniket");
    print_string(&new_str); // this funtion gains the ownership of new_str, and invalidates the new_str in main, actual heap string "Aniket" is not copied to print_str(new_str)
    // ptr + len + capacity
    // when function ends rust automatically calls: drop(new_str), which frees heap memory
    println!("{}", new_str); // borrowed temporary

    // Lifetimes describe the relationship between references so Rust can guarantee returned references remain valid.

    let v = vec![1, 2, 3];

    let ans = find_element(&v, 1);

    println!("{}", ans);
}

fn longest_str<'a>(a: & 'a String, b: & 'a String) -> & 'a String{
    // same 'a : minimum valid duration (rust automatically chooses)
    if a.len() > b.len(){
        a
    } else {
        b // returning & 'b String
    }
    // return a;
}

fn choose(a: i32, b: i32, flag: bool) -> i32{
    if flag {
        a
    } else {
        b
    }
}

fn print_string(s: &String){
    println!("{}", s);
}

fn find_element<T>(vec: &Vec<T>, itme: T)-> Option<&T>{
    return None;
}

