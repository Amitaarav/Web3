use unicode_segmentation::UnicodeSegmentation;
struct User{
    active: bool,
    sign_in_count: u64
}

fn main() {
    // let mut user1 = User{
    //     active: true,
    //     sign_in_count: 1
    // };

    // print_name(user1);

    //
    let x: i32 = 9; // 0 - 255
    let y: i32 = 10; // -128 -- 127

    // let a = [10, 20, 30, 40, 50];

    // for element in a.iter(){
    //     print!("{} ", element);
    // }

    let s = String::from("hello");
    // let s2 = s1.clone();

    // println!("{}", s1);
    takes_ownership(s);

    // println!("{}", s);

    let x = 5;
    make_copy(x);
    println!("{}", x);

    let s1 = gives_ownership();
    println!("gives_ownership: {}", s1);
    let s2 = String::from("value");

    let s3 = takes_and_gives_back(s2);
    println!("s1 = {}, s3 = {}", s1, s3);

    let s4 = String::from("hello");
    let len = calculate_length(&s4);
    println!("length of {} is: {}", s4, len);

    let mut s5 = String::from("Amit");
    change_string(&mut s5);
    println!("changed to {}", s5);

    let mut s = String::from("hi there!");
    let r1 = & s;
    let r2 = & s;

    println!("{}, {}: ", r1, r2); // out of scope, r1, r2
    let r3 = &mut s;

    println!("{}", r3);

    // let reference_to_nothing = dangle();


    let mut s = String::from("Hello World");
    let word = first_words(&s);
    println!("word: {}", word);
    s.clear();
    println!("s.first word: {}", first_words(&s));

    let s = "नमस्ते";
    println!("{}", s.chars().count()); // return number of UTF-8 bytes not the number of characters
}

fn print_name(user1: User){
    print!("User 1 username {}", user1.active);
}

fn takes_ownership(some_string: String){
    println!("{}", some_string);
}

fn make_copy(some_integer: i32){
    println!("{}", some_integer);
}

fn gives_ownership() -> String{
    let some_string = String::from("Hii there!");
    return some_string;
}

fn takes_and_gives_back(a_string: String) -> String{
    a_string
}

fn calculate_length(s: &String) -> usize{
    let length = s.len();
    length
}

fn change_string(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String{
//     let s = String::from("Baccho");

//     &s
// }

fn first_words(s: &String) -> usize{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn count_chars(s: &str) -> usize {
    s.graphemes(true).count()
}
