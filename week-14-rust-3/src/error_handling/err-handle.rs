

pub fn err_handling() {
    let value = find_first_a(String::from("Amit Kumar"));

    match value {
        Some(value) => println!("first a was found at {} ", value),
        None => println!("Could not find an a inside the string"),
    }
}

pub fn find_first_a(s: String) -> Option<i32> {
    
    for (index, char) in s.chars().enumerate(){
        if char == 'a' {
            return Some(index as i32);
        }
     }

    return None;
}